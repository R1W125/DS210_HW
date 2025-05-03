use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::mem;

#[derive(Debug)]
pub struct Graph{
    pub map: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            map: HashMap::new(),
        }
    }
    pub fn add_edge(&mut self, current: usize, next: usize) {
        self.map.entry(current).or_insert(vec![]).push(next);

    }
}

pub fn data_to_graph(filename: &str) -> Graph {
    let file: File = File::open(filename).expect("Failed to read the file.");
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut graph = Graph::new();
    
    for line in reader.lines().skip(1){
        if let Ok(line) = line{
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    graph.add_edge(a,b);
                }
            }
        }
    }
    graph
}

#[derive(Debug)]
pub struct Vertex {
    pub node: usize,
    pub terminations: i32,
    pub rank: f32,
}

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            node: 0,
            terminations: 0,
            rank: 0.0,
        }
    }
    pub fn swap_with(&mut self, other: &mut Vertex) {
        mem::swap(self, other);
    }
}