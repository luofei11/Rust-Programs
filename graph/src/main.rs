
#[doc="
    Author: Fei Luo.
    NetID: fla414
    This is an implementation of Graph which can read graph information from a text file.
    Graph struct is constructed with a HashMap, where the key is a vertex denoted by a String type
    and the value is a HashSet containing all of the neighbors of the key.
    Usage:
        - cargo run graph.txt
        - [src] [des]
        - To quit, input [quit]
    Comments:
        - The algorithm I used to search for a path is Breadth First Search.
        - With BFS, the path is always the shortest one.
    Assumptions:
        - Self loop in the graph is allowed
        - Keys are only Strings
        - Keys are unique
        - Keys contains only 1 character
        - Any path from source to destination is valid
        - Graph is undirected
        - Graph must be mutable
        - The alert is printed when there is no path from src to des
"]
use std::env;
use std::fs::File;
use std::collections::{HashMap,HashSet};
use std::collections::vec_deque::VecDeque;
use std::io::{self,BufReader,BufRead};
struct Graph{
    edges: HashMap<String, HashSet<String>>
}
impl Graph{
    fn new() -> Graph{
        Graph {edges: HashMap::new()}
    }
    fn add_edge(&mut self, a: &str, b: &str){
        if !self.edges.contains_key(a){
            self.edges.insert(a.to_string(), HashSet::new());
        }
        if !self.edges.contains_key(b){
            self.edges.insert(b.to_string(), HashSet::new());
        }

        if let Some(neighbors_of_a) = self.edges.get_mut(a) {
            neighbors_of_a.insert(b.to_string());
        }
        if let Some(neighbors_of_b) = self.edges.get_mut(b) {
            neighbors_of_b.insert(a.to_string());
        };
    }
    /*
    fn get_neighbors(self, node: String) -> Vec<String>{
        let neighbors = match self.edges.get(&node){
            Some(v) => v,
            None => panic!("Wrong!")
        };
        neighbors
    }
    */
}
fn create_graph(filename: String) -> Graph{
    let f = match File::open(filename){
        Ok(file) => file,
        Err(e) => {
            panic!("{}", e);
        }
    };
    let mut graph = Graph::new();
    let file = BufReader::new(&f);
    for line in file.lines(){
        let l = line.unwrap();
        let nodes:Vec<&str> = l.split(' ').collect();
        let parent = nodes[0];
        let length = nodes.len();
        for i in 1..length{
            graph.add_edge(parent,nodes[i]);
        }
        //debug output
        //println!("{:?}", graph.edges);
    }
    graph
}
/*
Algorithm: BFS
*/
fn find_path(src: String,des: String, graph: &Graph) -> String{
    let mut path_queue = VecDeque::new();
    let mut visited = HashMap::new();
    path_queue.push_back(src.clone());
    visited.insert(src.clone(),true);
    while !path_queue.is_empty(){
        let curr_path = match path_queue.pop_front(){
            Some(path) => path,
            None => panic!("")
        };
        let curr_node = &curr_path[curr_path.len() - 1..];
        let neighbors = match graph.edges.get(curr_node) {
            Some(vec) => vec,
            None => panic!("None")
        };
        for neighbor in neighbors{
            if neighbor.to_string() == des{
                let final_path: String = curr_path.to_string() + &(neighbor.to_string());
                return final_path;
            }
            if !visited.contains_key(neighbor){
                visited.insert(neighbor.to_string(),true);
                path_queue.push_back(curr_path.to_string() + &neighbor);
            }
        }
    }
    "No such path".to_string()
}
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: cargo run graph.txt");
    }
    let filename = args[1].to_string();
    let graph = create_graph(filename);
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
            }
            Err(error) => println!("error: {}", error),
        }
        let query: Vec<&str> = input.split(" ").collect();
            if query.len() == 1{
                if query[0].trim() == "quit"{
                    break;
                }
                println!("{:?}", "Input [src] [des] | quit");
            }
            else if query.len() == 2{
                let src: String = query[0].trim().to_string();
                let des: String = query[1].trim().to_string();
                let path = find_path(src, des,&graph);
                if path == "No such path".to_string(){
                    println!("{}", path);
                }
                else{
                    let mut result = String::new();
                    for c in path.chars(){
                        result = result + &c.to_string() + &" ";
                    }
                    println!("{:?}", result);
                }
            }
            else{
                println!("{:?}","Usage: [src] [des] | quit" );
            }
        }
    }
/*
#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn create_graph() {
        let mut graph = Graph::new();
        graph.add_edge("a","b");
        assert_eq!(vec!["b"], graph.edges.get("a"));
    }
}
*/
