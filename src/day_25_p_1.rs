// https://www.cs.tau.ac.il/~zwick/grad-algo-08/gmc.pdf

use std::collections::{HashMap, HashSet};
// use std::collections::BinaryHeap;
use priority_queue::PriorityQueue;

// use std::cmp::Reverse;

use std::fs::File;
use std::io::{self, BufRead};
// use crate::io::Error;


type Graph =  HashMap<String, HashMap<String, i32>>;


fn st_min_cut<'a>(graph: &'a Graph, next_key: &'a String) -> (&'a String, &'a String, i32) {
  let mut pq = PriorityQueue::new();

  let next_neighbors = graph.get(next_key).expect("");
  next_neighbors.iter().for_each(|nn| { pq.push(nn.0, *nn.1); } );

  let mut a_set: HashSet<&String> = HashSet::new();
  let mut s: (&String, i32);
  let mut t: (&String, i32) = (next_key, 1);
  a_set.insert(next_key);
  loop {
      s = pq.pop().expect("");
      a_set.insert(s.0);
      graph.get(s.0).expect("").iter().for_each(|n| {
        let is_n_in_a = a_set.contains(n.0);
        if is_n_in_a {
          pq.remove(n.0);
        }
        // TODO(maryam): Check if priority chane eq is correct.
        let change_priority_result = pq.change_priority_by(&n.0, |p| *p += n.1);
        if !is_n_in_a && !change_priority_result {
          pq.push(&n.0, *n.1);
        }
      });
    if pq.is_empty() { break; }
    t = s;
    // println!("a_set size: {:?}, pq size: {:?}", a_set.len(), pq.len());
  }
  println!("Size of a_set: {}", a_set.len());
  (s.0, t.0, t.1)
}


fn merge_nodes(a_graph: &Graph, s: &String, t: &String) -> Graph {
  let mut graph = a_graph.clone();
  println!("Removing s = {:?}, t = {:?}", s, t);
  let mut new_key: String = s.clone();
  new_key.push_str(t);

  let mut new_edges: HashMap<String, i32> = HashMap::new();

  graph.get(s).expect("").iter().for_each(|n| {
    new_edges.insert(n.0.clone(), *n.1);  
  });
  graph.get(t).expect("").iter().for_each(|n| {
    new_edges.entry(n.0.clone()).and_modify(|v| *v += n.1).or_insert(*n.1);
  });
  graph.remove(s);
  graph.remove(t);
  new_edges.remove(s);
  new_edges.remove(t);

  new_edges.iter().for_each(|n| {
    // println!("Attempting to remove n = {:?}", n);
    let node_n: &mut HashMap<String, i32> = graph.get_mut(n.0).expect("");
    node_n.remove(s);
    node_n.remove(t);
  });
  graph.insert(new_key, new_edges);
  graph
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

  let mut adjacency_list: Graph = HashMap::new();

  reader.lines().for_each(|line| {
    let line = line.expect("");
    let line_v: Vec<&str> = line.split(&[' ', ':']).filter(|word| !word.is_empty()).collect();
    let node = line_v[0];
    line_v[1..].iter().for_each(|w| {
      adjacency_list.entry(node.to_string()).or_default().insert(w.to_string(), 1);
      adjacency_list.entry(w.to_string()).or_default().insert(node.to_string(), 1);
    });
  });
  adjacency_list
}




fn main() {
  let a_graph = get_graph("data/d25.txt");
  println!("{:?}", a_graph.iter().next());
  // println!("{:?}", a_graph.len());
  // let next_key: &String = a_graph.keys().next().expect("");
  // let min_cut = st_min_cut(&a_graph, next_key);
  // println!("{:?}", min_cut);
}