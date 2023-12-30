use core::panic;
use std::{str::FromStr, vec, collections::{HashMap, VecDeque}};

#[derive(Debug)]
struct Rule {
    variable: String,
    range: (usize, usize),
    action: String,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(":").collect::<Vec<_>>();

        match split.len() {
            2 => {
                let variable = split[0][0..1].trim().to_string();
                let ord = split[0][1..2].trim().to_string();
                let ranges = {
                    match ord.as_str() {
                        "<" => {
                            let num = split[0][2..].trim().parse::<usize>().unwrap();
                            (usize::MIN, num)
                        },
                        ">" => {
                            let num = split[0][2..].trim().parse::<usize>().unwrap();
                            (num, usize::MAX)
                        },
                        _ => {
                            eprintln!("Invalid rule: {}", s);
                            panic!("Invalid rule");
                        }
                    }
                };
                let action = split[1].trim().to_string();

                Ok(Rule { variable, range: ranges, action })
            }
            1 => {
                let variable = String::new();
                let ranges = (0, 0);
                let action = split[0].trim().to_string();

                Ok(Rule { variable, range: ranges, action })
            }
            _ => {
                eprintln!("Invalid rule: {}", s);
                Err(())
            }
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("{").collect::<Vec<_>>();
        let name = split[0].trim().to_string();
        
        let rules = split[1]
                            .split("}")
                            .collect::<Vec<_>>()[0]
                            .split(",")
                            .map(|rule| rule.parse::<Rule>().unwrap())
                            .collect::<Vec<_>>();

        Ok(Workflow { name, rules })
    }
}

fn part1(input: &str) -> usize {
    let content = std::fs::read_to_string(input).unwrap();
    let split = content.split("\r\n\r\n").collect::<Vec<_>>();
    let (workflows, parts) = (split[0], split[1]);

    let mut workflows  = workflows
        .lines()
        .map(|line| {
            line.parse::<Workflow>().unwrap()
        })
        .collect::<Vec<_>>();

    // convert workflow vector to hashmap
    let mut workflow_map = HashMap::new();
    for workflow in workflows.iter_mut() {
        workflow_map.insert(workflow.name.clone(), workflow);
    }  

    let parts = parts
        .lines()
        .map(|line| {
                            line[1..line.len()-1]
                                        .to_string()
                                        .split(",")
                                        .map(|part| {
                                            let split = part.split("=").collect::<Vec<_>>();
                                            let name = split[0].trim().to_string();
                                            let value = split[1].trim().parse::<usize>().unwrap();

                                            (name, value)
                                        })
                                        .collect::<HashMap<_, _>>()
                        })
                        .collect::<Vec<_>>();

    let mut sum = 0;

    for part in parts.iter() {
        let mut workflow = workflow_map.get("in").unwrap();
        let mut keep_on_going = true;
        while keep_on_going
        {
            for rule in workflow.rules.iter() {
                let eval_var = rule.variable.clone();

                let action:Option<String> = {
                                                match eval_var.as_str() {
                                                    "" => {
                                                        Some(rule.action.clone())
                                                    },
                                                    s => {
                                                        let &eval_val = part.get(s).unwrap();
                                                        if rule.range.0 < eval_val && eval_val < rule.range.1 {
                                                            Some(rule.action.clone())
                                                        } else {
                                                            None
                                                        }
                                                    },
                                                }
                                            };

                match action {
                    Some(action) => {
                        match action.as_str() {
                            "R" => {
                                keep_on_going = false;
                                break;
                            },
                            "A" => {
                                sum += part.get("x").unwrap();
                                sum += part.get("m").unwrap();
                                sum += part.get("a").unwrap();
                                sum += part.get("s").unwrap();
                                keep_on_going = false;
                                break;
                            },
                            _ => {
                                workflow = workflow_map.get(&action).unwrap();
                                break;
                            }
                        }
                    },
                    None => {},
                }
            }
        }
    }

    sum
}

struct Ranges {
    xmas: HashMap<String, (usize, usize)>,
}

fn part2(input: &str) -> usize {
    let content = std::fs::read_to_string(input).unwrap();
    let split = content.split("\r\n\r\n").collect::<Vec<_>>();
    let (workflows, _) = (split[0], split[1]);

    let mut workflows  = workflows
        .lines()
        .map(|line| {
            line.parse::<Workflow>().unwrap()
        })
        .collect::<Vec<_>>();

    // convert workflow vector to hashmap
    let mut workflow_map = HashMap::new();
    for workflow in workflows.iter_mut() {
        workflow_map.insert(workflow.name.clone(), workflow);
    }  


    let mut ranges: (String, Ranges) = ("in".to_string(), Ranges {
        xmas: {
            let mut xmas = HashMap::new();
            xmas.insert("x".to_string(), (0, 4000));
            xmas.insert("m".to_string(), (0, 4000));
            xmas.insert("a".to_string(), (0, 4000));
            xmas.insert("s".to_string(), (0, 4000));
            xmas
        },
    });

    let mut deque = VecDeque::new();
    deque.push_back(ranges);

    let mut sum = 0;

    while !deque.is_empty() {
        let (workflow_name, ranges) = deque.pop_front().unwrap();
        let workflow = workflow_map.get(&workflow_name).unwrap();

        let mut deque_ranges = ranges.xmas.clone();
        for rule in workflow.rules.iter() {
            let rule_var = &rule.variable;

            let mut act_on_ranges = deque_ranges.clone();

            let action = {
                                            match rule_var.as_str() {
                                                "" => {
                                                    Some(&rule.action)
                                                },
                                                rule_category => {
                                                    let deque_range = deque_ranges.get_mut(rule_category).unwrap();
                                                    let rule_range = &rule.range;

                                                    let act_on_range = act_on_ranges.get_mut(rule_category).unwrap();
                                                    
                                                    if     (deque_range.0 < rule_range.0 && rule_range.0 < deque_range.1) 
                                                        || (deque_range.0 < rule_range.1 && rule_range.1 < deque_range.1) {

                                                        if deque_range.0 < rule_range.0 && rule_range.0 < deque_range.1 {
                                                            act_on_range.0 = rule_range.0;
                                                            deque_range.1 = rule_range.0;
                                                        }
    
                                                        if deque_range.0 < rule_range.1 && rule_range.1 < deque_range.1 {
                                                            act_on_range.1 = rule_range.1 - 1;
                                                            deque_range.0 = rule_range.1 - 1;
                                                        }
    
                                                        Some(&rule.action)
                                                    } else {
                                                        None
                                                    }
                                                },
                                            }
                                        };

            match action {
                Some(action) => {
                    match action.as_str() {
                        "R" => {},
                        "A" => {
                            sum +=    (act_on_ranges.get("x").unwrap().1 - act_on_ranges.get("x").unwrap().0)
                                    * (act_on_ranges.get("m").unwrap().1 - act_on_ranges.get("m").unwrap().0)
                                    * (act_on_ranges.get("a").unwrap().1 - act_on_ranges.get("a").unwrap().0)
                                    * (act_on_ranges.get("s").unwrap().1 - act_on_ranges.get("s").unwrap().0);
                            println!("sum: {} * {} * {} * {} = {}" 
                                , (act_on_ranges.get("x").unwrap().1 - act_on_ranges.get("x").unwrap().0)
                                , (act_on_ranges.get("m").unwrap().1 - act_on_ranges.get("m").unwrap().0)
                                , (act_on_ranges.get("a").unwrap().1 - act_on_ranges.get("a").unwrap().0)
                                , (act_on_ranges.get("s").unwrap().1 - act_on_ranges.get("s").unwrap().0)
                                , sum);
                        },
                        _ => {
                            deque.push_back((action.to_string(), Ranges {xmas: act_on_ranges.clone()} ));
                        }
                    }
                },
                None => {},
            }
        }
    }

    sum
}

fn main() {
    // println!("Part1: {}", part1("data/prod"));
    println!("Part2: {}", part2("data/prod"));
}
