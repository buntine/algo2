use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub struct Vertex {
    label: i32,
    explored: bool,
    edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Edge {
    cost: i32,
    tail: usize, // This should be a borrow of another Vertex but I couldn't work it out.
}

impl Graph {
    fn new() -> Graph {
        Graph{vertices: vec![]}
    }

    fn build(&mut self, size: i32) {
        for i in 0..size {
            self.vertices.push(Vertex{label: i, explored: false, edges: vec![]});
        }
    }

    fn split_line<T: std::str::FromStr>(line: &str) -> Vec<T> {
        line.split(" ")
            .map(|d| { 
                let e = d.trim().parse::<T>();
                match e {
                    Ok(v) => v,
                    Err(_) => panic!("Invalid: {:?}", d),
                }
            })
            .collect()
    }

    fn from_file(path: &Path) -> Result<Graph, std::io::Error> {
        let mut g = Graph{vertices: vec![]};
        let file = try!(File::open(path));
        let mut buffer = BufReader::new(&file);
        let mut first_line = String::new();
        
        try!(buffer.read_line(&mut first_line));

        let details = Graph::split_line::<i32>(&first_line[..]);

        g.build(details[0]);

        for l in buffer.lines() {
            match l {
                Ok(parts) => {
                    let details = Graph::split_line::<i32>(&parts[..]);
                    let head = (details[0] - 1) as usize;
                    let tail = (details[1] - 1) as usize;

                    g.vertices[head].edges.push(Edge{tail: tail, cost: details[2]});
                    g.vertices[tail].edges.push(Edge{tail: head, cost: details[2]});
                },
                Err(e) => return Err(e),
            }
        }
 
        Ok(g)
    }
}

pub fn mst(g: &mut Graph) -> i32 {
    let mut t = Graph::new();
    let mut e = Edge{tail: 0, cost: 0};
    let mut total_cost = 0;

    t.build(g.vertices.len() as i32);

    g.vertices[0].explored = true;
    t.vertices[0].explored = true;

    for _ in 0..(g.vertices.len()-1) {
        e.cost = 999999;

        for vertex in t.vertices.iter().filter(|v| v.explored) {
            for edge in &g.vertices[vertex.label as usize].edges {
                if edge.cost < e.cost && g.vertices[edge.tail].explored == false {
                    e = Edge{tail: edge.tail, cost: edge.cost};
                }
            }
        }

        total_cost += e.cost;
        g.vertices[e.tail as usize].explored = true;
        t.vertices[e.tail as usize].explored = true;
    }

    total_cost
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let mut g = Graph::new();

        g.build(2);
        g.vertices[0].edges.push(Edge{cost: 10, tail: 1});
        g.vertices[1].edges.push(Edge{cost: 10, tail: 0});

        assert_eq!(g.vertices[1].label, 1);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.cost, 10);
        assert_eq!(g.vertices[e.tail].label, 1);
    }

    #[test]
    fn representation() {
        let p = Path::new("edges.txt");

        let g = Graph::from_file(p).ok().unwrap();

        assert_eq!(g.vertices[0].label, 0);
        assert_eq!(g.vertices[5].label, 5);
        assert_eq!(g.vertices[30].label, 30);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.cost, 6807);
        assert_eq!(e.tail, 1);
        assert_eq!(g.vertices[e.tail].label, 1);

        let ref e = g.vertices[18].edges[1];
        assert_eq!(e.cost, 7674);
    }

    #[test]
    fn run() {
        let p = Path::new("edges.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(mst(&mut g), -3612829)
    }

    #[test]
    fn simple1() {
        let p = Path::new("edges_simple1.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(mst(&mut g), 4)
    }

    #[test]
    fn simple2() {
        let p = Path::new("edges_simple2.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(mst(&mut g), 16)
    }

    #[test]
    fn simple3() {
        let p = Path::new("edges_simple3.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(mst(&mut g), -236)
    }
}
