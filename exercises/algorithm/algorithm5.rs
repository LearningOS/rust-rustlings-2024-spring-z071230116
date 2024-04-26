/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		//TODO
        let mut _visited_status = vec![0;self.adj.len()];
        let mut visit_order = vec![]; 
        let mut _start = start;
        let mut _index = 0usize;
        visit_order.push(_start);
        _visited_status[_start] = 1;
        
        loop {
            println!("the len is {}",self.adj.len());
            let mut adj_vec_iter = self.adj[_start].iter();
            loop {
                match adj_vec_iter.next(){
                    Some(p) =>{
                        if _visited_status[*p] != 1{
                            visit_order.push(*p);
                            _visited_status[*p] = 1;
                        }
                    },
                    None =>{
                        break;
                    }
                }                
            }
            if _index < self.adj.len(){
                _index +=1;
                match visit_order.get(_index){
                    Some(p) =>{
                        println!("the len is {}",_start);
                        _start = *p;
                    },
                    None =>{
                        return visit_order;
                    }
                }
            }            
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

