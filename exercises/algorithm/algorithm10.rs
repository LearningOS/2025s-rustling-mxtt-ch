/*
	graph
	This problem requires you to implement a basic graph functio
*/


use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        let (from_node, to_node, weight) = edge;
        
        // 确保两个节点都存在
        self.add_node(from_node);
        self.add_node(to_node);
        
        // 添加边
        let from_node_neighbors = self.adjacency_table_mutable().get_mut(from_node).unwrap();
        from_node_neighbors.push((to_node.to_string(), weight));
        
        // 对于无向图，还需要添加反向边
        let to_node_neighbors = self.adjacency_table_mutable().get_mut(to_node).unwrap();
        to_node_neighbors.push((from_node.to_string(), weight));
    }
    
    fn shortest_path(&self, start: &str, end: &str) -> Option<(Vec<String>, i32)> {
        // 检查起点和终点是否存在于图中
        if !self.contains(start) || !self.contains(end) {
            return None;
        }
        
        // 如果起点和终点相同，直接返回
        if start == end {
            return Some((vec![start.to_string()], 0));
        }
        
        // 初始化距离表和前驱节点表
        let mut distances: HashMap<String, i32> = HashMap::new();
        let mut predecessors: HashMap<String, String> = HashMap::new();
        let mut unvisited: HashSet<String> = HashSet::new();
        
        // 初始化所有节点的距离为无穷大
        for node in self.nodes() {
            distances.insert(node.clone(), i32::MAX);
            unvisited.insert(node.clone());
        }
        
        // 设置起点距离为0
        distances.insert(start.to_string(), 0);
        
        // Dijkstra算法主循环
        while !unvisited.is_empty() {
            // 找到未访问节点中距离最小的节点
            let current = unvisited.iter()
                .min_by_key(|node| distances.get(*node).unwrap_or(&i32::MAX))
                .map(|node| node.clone())
                .unwrap();
            
            // 如果当前节点是终点，构建路径并返回
            if current == end {
                let mut path = Vec::new();
                let mut current_node = current.clone();
                path.push(current_node.clone());
                
                while let Some(predecessor) = predecessors.get(&current_node) {
                    path.push(predecessor.clone());
                    current_node = predecessor.clone();
                }
                
                path.reverse();
                return Some((path, *distances.get(&end.to_string()).unwrap()));
            }
            
            // 如果当前节点的距离是无穷大，说明无法到达更多节点
            if distances.get(&current).unwrap() == &i32::MAX {
                break;
            }
            
            // 从未访问集合中移除当前节点
            unvisited.remove(&current);
            
            // 更新邻居节点的距离
            if let Some(neighbors) = self.adjacency_table().get(&current) {
                for (neighbor, weight) in neighbors {
                    if unvisited.contains(neighbor) {
                        let new_distance = distances.get(&current).unwrap() + weight;
                        if new_distance < *distances.get(neighbor).unwrap_or(&i32::MAX) {
                            distances.insert(neighbor.clone(), new_distance);
                            predecessors.insert(neighbor.clone(), current.clone());
                        }
                    }
                }
            }
        }
        
        // 如果无法到达终点，返回None
        None
    }
}
pub trait Graph {
    fn new() -> Self;
    fn shortest_path(&self, start: &str, end: &str) -> Option<(Vec<String>, i32)>;
    fn add_edge(&mut self, edge: (&str, &str, i32));  // 添加这一行

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        if self.contains(node) {
            // 节点已存在，返回false
            false
        } else {
            // 添加新节点
            self.adjacency_table_mutable().insert(node.to_string(), Vec::new());
            true
        }
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}