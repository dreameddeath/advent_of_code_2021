use crate::utils::Part;
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
enum Cave {
    Start,
    Big(String),
    Small(String),
    End,
}

impl Cave {
    fn to_str(&self) -> &str {
        return match self {
            Cave::Big(s) => s,
            Cave::Small(s) => s,
            Cave::Start => "start",
            Cave::End => "end",
        };
    }
}

struct Link {
    source: Cave,
    target: Cave,
}

type Graph = Vec<Link>;

fn is_big_cave(line: &str) -> bool {
    return line
        .chars()
        .nth(0)
        .map(|c| c.is_uppercase())
        .unwrap_or(false);
}

fn to_cave(line: &str) -> Cave {
    return match line {
        "start" => Cave::Start,
        "end" => Cave::End,
        _ => {
            if is_big_cave(line) {
                Cave::Big(String::from(line))
            } else {
                Cave::Small(String::from(line))
            }
        }
    };
}

fn parse_line(line: &str) -> Option<Link> {
    let parts: Vec<&str> = line.split("-").collect();
    let source = parts.get(0).map(|val| to_cave(val))?;
    let target = parts.get(1).map(|val| to_cave(val))?;
    return Some(Link {
        source: source,
        target: target,
    });
}

fn parse(lines: &Vec<String>) -> Graph {
    return lines
        .into_iter()
        .filter_map(|line| parse_line(&line))
        .flat_map(|link| {
            let source = link.source.clone();
            let target = link.target.clone();
            vec![
                link,
                Link {
                    source: target,
                    target: source,
                },
            ]
        })
        .collect();
}

type Path<'a> = Vec<&'a Cave>;
type CompileGraph<'a> = HashMap<&'a str, Vec<&'a Cave>>;

fn build_map<'a>(rules: &'a Graph) -> CompileGraph {
    let mut map: HashMap<&'a str, Vec<&'a Cave>> = HashMap::new();
    rules.into_iter().for_each(|rule| {
        if let Some(vals) = map.get_mut(rule.source.to_str()) {
            vals.push(&rule.target)
        } else {
            map.insert(rule.source.to_str(), vec![&rule.target]);
        }
    });
    return map;
}

fn find_visitable_caves<'a>(
    potential_node: &Vec<&'a Cave>,
    curr_path: &Path<'a>,
    allow_duplicate_small_cave: bool,
) -> Vec<(bool, &'a Cave)> {
    return potential_node
        .iter()
        .filter_map(|cave| match cave {
            Cave::Small(_) => {
                let already_exist = curr_path.contains(cave);
                if allow_duplicate_small_cave || !already_exist {
                    Some((allow_duplicate_small_cave && !already_exist, *cave))
                } else {
                    None
                }
            }
            Cave::Start => None,
            _ => Some((allow_duplicate_small_cave, cave)),
        })
        .collect();
}

fn find_all_paths<'a>(
    curr_cave: &Cave,
    graph: &CompileGraph<'a>,
    curr_path: &mut Path<'a>,
    allow_duplicate_cave: bool,
    result: &mut Vec<Path<'a>>,
) {
    if let Cave::End = curr_cave {
        result.push(curr_path.clone());
        return;
    }
    if let Some(potential_caves) = graph.get(curr_cave.to_str()) {
        let next_cave_info = find_visitable_caves(potential_caves, curr_path, allow_duplicate_cave);
        for (new_allow_duplicate_cave, cave) in next_cave_info {
            curr_path.push(cave);
            find_all_paths(cave, graph, curr_path, new_allow_duplicate_cave, result);
            curr_path.pop();
        }
    }
}

pub fn puzzle(part: &Part, lines: &Vec<String>) {
    let links = parse(lines);
    let compiled_graph = build_map(&links);
    match part {
        Part::Part1 => {
            let mut result: Vec<Path> = Vec::new();
            find_all_paths(
                &Cave::Start,
                &compiled_graph,
                &mut vec![&Cave::Start],
                false,
                &mut result,
            );

            println!("Result {}", result.len())
        }
        Part::Part2 => {
            let mut result: Vec<Path> = Vec::new();
            find_all_paths(
                &Cave::Start,
                &compiled_graph,
                &mut vec![&Cave::Start],
                true,
                &mut result,
            );

            println!("Result {}", result.len())
        }
    }
}
