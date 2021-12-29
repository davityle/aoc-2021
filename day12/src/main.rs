use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn main() {
    let input_re = Regex::new(r#"([a-zA-Z]+)-([a-zA-Z]+)"#).unwrap();

    let node_relationships = INPUT
        .iter()
        .flat_map(|relationship| input_re.captures(relationship).map(regex_capture_to_vec))
        .collect::<Vec<_>>();

    let unique_keys: HashSet<&str> =
        HashSet::from_iter(node_relationships.iter().flatten().map(|&s| s));

    let nodes: Vec<Node> = unique_keys
        .iter()
        .map(|&key| key.to_owned())
        .map(|key| Node {
            small: key.as_str().to_lowercase() == key,
            key,
            visited_paths: HashSet::new(),
        })
        .collect();

    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut reversed_edges: HashMap<String, Vec<String>> = HashMap::new();
    for pair in node_relationships.iter() {
        edges
            .entry(pair[0].to_owned())
            .or_insert_with(Vec::new)
            .push(pair[1].to_owned());
        reversed_edges
            .entry(pair[1].to_owned())
            .or_insert_with(Vec::new)
            .push(pair[0].to_owned());
    }
    let mut graph = Graph {
        nodes: nodes
            .into_iter()
            .map(|n| (n.key.clone(), Rc::new(n)))
            .collect(),
        edges,
        reversed_edges,
        paths: Vec::new(),
    };
    let start_node = graph.nodes.get("start").unwrap();
    // It seems like this could probably be done with math instead ¯\_(ツ)_/¯
    find_all_paths(
        start_node.clone(),
        Path {
            seen_nodes: HashSet::new(),
            key: "start".to_owned(),
            visited_small_twice: false,
        },
        &mut graph,
    );

    println!(
        "{:?}",
        graph
            .paths
            .iter()
            .filter(|p| p.seen_nodes.contains("end"))
            .collect::<Vec<_>>()
            .len()
    );
}

fn find_all_paths(node: Rc<Node>, mut path: Path, graph: &mut Graph) {
    let restrict_visit = node.key == "start" || (node.small && path.visited_small_twice);
    if path.seen_nodes.contains(&node.key)
        && (restrict_visit || node.visited_paths.contains(&path.key))
    {
        return;
    }
    if node.small && path.seen_nodes.contains(&node.key) {
        path.visited_small_twice = true;
    }
    path.seen_nodes.insert(node.key.clone());
    graph.paths.push(path.clone());

    if node.key == "end" {
        return;
    }

    let node_pointers_opt: Option<Vec<Rc<Node>>> = graph.edges.get(&node.key).map(|edges| {
        edges
            .iter()
            .map(|e| graph.nodes.get(e).unwrap().clone())
            .collect()
    });

    if let Some(node_pointers) = node_pointers_opt {
        for node in node_pointers {
            let new_path = Path {
                key: path.key.clone() + "-" + node.key.as_str(),
                seen_nodes: HashSet::from_iter(path.seen_nodes.iter().map(|k| k.clone())),
                visited_small_twice: path.visited_small_twice,
            };
            find_all_paths(node, new_path, graph);
        }
    }

    let reverse_node_pointers_opt: Option<Vec<Rc<Node>>> =
        graph.reversed_edges.get(&node.key).map(|edges| {
            edges
                .iter()
                .map(|e| graph.nodes.get(e).unwrap().clone())
                .collect()
        });

    if let Some(node_pointers) = reverse_node_pointers_opt {
        for node in node_pointers {
            let new_path = Path {
                key: path.key.clone() + "-" + node.key.as_str(),
                seen_nodes: HashSet::from_iter(path.seen_nodes.iter().map(|k| k.clone())),
                visited_small_twice: path.visited_small_twice,
            };
            find_all_paths(node, new_path, graph);
        }
    }
}

fn regex_capture_to_vec(capture: Captures) -> Vec<&str> {
    capture
        .iter()
        .skip(1)
        .flat_map(|c| c)
        .map(|c| c.as_str())
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Node {
    key: String,
    small: bool,
    visited_paths: HashSet<String>,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<String, Rc<Node>>,
    edges: HashMap<String, Vec<String>>,
    reversed_edges: HashMap<String, Vec<String>>,
    paths: Vec<Path>,
}

#[derive(Debug, Clone)]
struct Path {
    seen_nodes: HashSet<String>,
    key: String,
    visited_small_twice: bool,
}

const _: [&str; 7] = ["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];
const _: [&str; 10] = [
    "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa", "kj-HN",
    "kj-dc",
];
const _: [&str; 18] = [
    "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he", "RW-he",
    "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
];
const INPUT: [&str; 23] = [
    "start-YA", "ps-yq", "zt-mu", "JS-yi", "yq-VJ", "QT-ps", "start-yq", "YA-yi", "start-nf",
    "nf-YA", "nf-JS", "JS-ez", "yq-JS", "ps-JS", "ps-yi", "yq-nf", "QT-yi", "end-QT", "nf-yi",
    "zt-QT", "end-ez", "yq-YA", "end-JS",
];
