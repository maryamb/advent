// https://www.cs.tau.ac.il/~zwick/grad-algo-08/gmc.pdf

use std::collections::{HashMap, HashSet};
// use std::collections::BinaryHeap;
use std::clone::Clone;
use priority_queue::PriorityQueue;

use std::thread;
use std::time::Duration;

// use std::cmp::Reverse;

use std::fs::File;
use std::io::{self, BufRead};
// use crate::io::Error;

#[derive(Clone, Debug)]
struct Graph {
  nodes: HashSet<String>,
  edges: HashMap<(String, String), i32>,
}

impl Graph {
  fn new(nodes: HashSet<String>, edges: HashMap<(String, String), i32>) -> Self {
    Graph {
      nodes,
      edges,
    }
  }
}

fn st_min_cut(graph: &Graph) -> (String, String, i32) {
  let mut pq = PriorityQueue::new();
  graph.nodes.iter().for_each(|n| { pq.push(n, 0); } );

  let mut a_set: HashSet<String> = graph.nodes.clone();
  let mut s: String = String::from("");
  let mut t: String = String::from("");
  let mut cost = 0;
  while !pq.is_empty() {
    let u = pq.pop().unwrap();
    a_set.remove(u.0);
    s = t;
    t = u.0.clone();
    cost = u.1;

    graph.edges.iter().for_each(|((a, v), w)| {
      if a.eq(u.0) && a_set.contains(v) {
        let key_v = pq.get_priority(v);
        pq.change_priority(v, w + key_v.unwrap());
      }
    });
  }
  // println!("nodes size: {}, cut cost: {}", graph.nodes.len(), cost);
  (s, t, cost)
}


fn merge_nodes(a_graph: &Graph, s: &String, t: &String) -> Graph {
  if a_graph.nodes.len() % 100 == 0 {
    println!("{}", a_graph.nodes.len());
  }
  let mut graph: Graph = a_graph.clone();
  // println!("Removing s = {:?}, t = {:?}", s, t);
  // thread::sleep(Duration::from_secs(1));
  let new_key: String = s.clone() + t;

  graph.nodes.remove(s);
  graph.nodes.remove(t);
  graph.nodes.insert(new_key.clone());

  // TODO(not me): Merge the two lists and add a bit indicating if s and t are the same.
  let mut add_edges: HashMap<(String, String), i32> = HashMap::new();
  let mut remove_edges: Vec<(String, String)> = vec![];
  graph.edges.iter().for_each(|(e, w)| {
    let cond_0 = e.0 == *s || e.0 == *t;  // one side of the edge is in {s, t}
    let cond_1 = e.1 == *s || e.1 == *t;  // Another side of the edge is in {s, t}
    if cond_0 != cond_1 {  // Just one side is in {s, t}
      let u = if cond_0 { new_key.clone() } else { e.0.clone() };
      let v = if cond_1 { new_key.clone() } else { e.1.clone() };
      add_edges.entry((u, v)).and_modify(|w_1| *w_1 += w).or_insert(*w);
    }
    if cond_0 || cond_1 {
      remove_edges.push(e.clone());
    }
  });

  remove_edges.iter().for_each(|e| {
    graph.edges.remove(e);
  });
  add_edges.drain().for_each(|(e, w)| {
    graph.edges.insert(e, w);
  });
  

  graph
}

fn global_min_cut(graph: &Graph) -> (String, String, i32) {
  let mut best = (String::new(), String::new(), i32::MAX);
  let mut graph_stored;
  let empty_graph = Graph::new(HashSet::new(), HashMap::new());
  let mut graph_2 = graph;
  loop {
    if graph_2.nodes.len() < 2 {
      return best;
    }
    if graph_2.nodes.len() < 4 {
      // println!("{:?}", graph_2);
    }
     
    let cut = st_min_cut(graph_2);
    let g = merge_nodes(graph_2, &cut.0, &cut.1);
    graph_2 = &empty_graph;
    graph_stored = g;
    graph_2 = &graph_stored;
    if cut.2 < best.2 {
      best = cut;
    }
  }
}

fn get_graph(file_path: &str) -> Graph {
  // Open the file
  // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&file_path) {
      Err(why) => panic!("couldn't open {}: {}", file_path, why),
      Ok(file) => file,
    };

  // Create a buffered reader to read the file line by line
  let reader = io::BufReader::new(file);

  let mut edges: HashMap<(String, String), i32> = HashMap::new();
  let mut nodes: HashSet<String> = HashSet::new();

  reader.lines().for_each(|line| {
    let line = line.expect("");
    let line_v: Vec<&str> = line.split(&[' ', ':']).filter(|word| !word.is_empty()).collect();
    let v = line_v[0].to_string();
    nodes.insert(v.clone());
    line_v[1..].iter().for_each(|w| {
      let w = w.to_string();
      nodes.insert(w.clone());
      edges.insert((v.clone(), w.clone()), 1);
      edges.insert((w.clone(), v.clone()), 1);
    });
  });
  Graph::new(nodes, edges)
}




fn main() {
  let a_graph = get_graph("data/d25.txt");
  // println!("{:?}", a_graph.len());
  let min_cut = global_min_cut(&a_graph);
  println!("Final global min cut is  \n {:?} \n\n", min_cut);
  // println!("\n{:?}\n", a_graph.nodes);
  println!("\n{}\n", a_graph.nodes.len());
  println!("{}, {}", min_cut.0.len() / 3, min_cut.1.len() / 3);
  let part_one = min_cut.1.len() / 3;
  let part_two = a_graph.nodes.len() - part_one;
  println!("The answer to the puzzle is {}", part_one * part_two);
}