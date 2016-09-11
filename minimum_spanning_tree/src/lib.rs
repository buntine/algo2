pub struct Graph {
    nodes: Vec<Vertex>,
}

pub struct Vertex {
    label: i32,
    explored: bool,
    adjacent: Vec<Edge>,
}

pub struct Edge {
    cost: u32,
    tail: i32,
}

impl Graph {
    fn build(&mut self, size: i32) {
        for i in 0..size {
            self.nodes.push(Vertex{label: i, explored: false, adjacent: vec![]});
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut g = Graph{nodes: vec![]};

        g.build(2);
        g.nodes[0].adjacent.push(Edge{cost: 10, tail: 1});
        g.nodes[1].adjacent.push(Edge{cost: 12, tail: 0});
    }

    #[test]
    fn test2() {
        
    }
}
