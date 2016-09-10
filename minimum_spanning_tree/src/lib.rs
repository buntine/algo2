pub struct Graph {
    nodes: Vec<Vertex>,
}

pub struct Vertex {
    length: u32,
    label: i32,
    explored: bool,
    adjacent: Vec<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut g = Graph{nodes: vec![]};
        let mut v1 = Vertex{label: 0, length: 1, explored: false, adjacent: vec![]};
        let mut v2 = Vertex{label: 1, length: 1, explored: false, adjacent: vec![]};

        v1.adjacent.push(1);
        v2.adjacent.push(0);

        g.nodes.push(v1);
        g.nodes.push(v2);
    }
}
