mod graph;
use crate::graph::data_to_graph;
use crate::graph::Graph;
use crate::graph::Vertex;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashMap;

fn main() {

    let graph: Graph = data_to_graph("/Users/RiwazShrestha/Desktop/DS210/Homework/HW7/pagerank_data.txt");
    
    let top5 = page_rank(&graph);

    for l in top5{
        println!("Vertex: {:?}, approximate PageRank: {:?}", l.node, l.rank);
    }
}

fn page_rank (graph: &Graph) -> Vec<Vertex> {
    
    let mut top_5_list: Vec<Vertex> = Vec::new();

    let mut top1: Vertex = Vertex::new();
    let mut top2: Vertex = Vertex::new();
    let mut top3: Vertex = Vertex::new();
    let mut top4: Vertex = Vertex::new();
    let mut top5: Vertex = Vertex::new();
    
    let mut termination_map: HashMap<usize, i32> = HashMap::new();
    
    for k in graph.map.keys() {

        let keys: Vec<usize> = graph.map.keys().cloned().collect();

        for _i in 0..90{

            let mut current = k;

            for _j in 0..90{

                let mut rng = rand::thread_rng();
                let num = rng.gen_range(1..=10);

                if num < 3 {

                    let random_key = keys.choose(&mut rng).unwrap();
                    current = random_key;

                } else {

                    let random_node = (graph.map.get(k).unwrap()).choose(&mut rng).unwrap();
                    current = random_node;
                }

            }
            *termination_map.entry(*current).or_insert(1) += 1;
        }
    }

    for k in termination_map.keys() {

        let termination_value = termination_map.get(k).unwrap();

        let rank = (*termination_value as f32) / 90000.00;

        if *termination_value > top5.terminations {

            top5.terminations = *termination_value;

            top5.node = *k;

            top5.rank = rank;
        }
        if top5.terminations > top4.terminations{

            top4.swap_with(&mut top5);

            if top4.terminations > top3.terminations{

                top3.swap_with(&mut top4);

                if top3.terminations > top2.terminations{

                    top2.swap_with(&mut top3);

                    if top2.terminations > top1.terminations{

                        top1.swap_with(&mut top2);
                    }
                }                
            }           
        } 
    }

    top_5_list.push(top1);
    top_5_list.push(top2);
    top_5_list.push(top3);
    top_5_list.push(top4);
    top_5_list.push(top5);

    top_5_list
}

#[test]
fn simple_graph() {
    let mut graph = Graph::new();
    graph.add_edge(0,1);
    graph.add_edge(1,0);
    graph.add_edge(2,0);
    graph.add_edge(0,2);
    let values = page_rank(&graph);
    let result = &values[0].node;
    let expected = 0;

    assert_eq!(*result, expected, "Fail!!");
}
