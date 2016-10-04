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
    weight: i32,
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

                    g.vertices[head].edges.push(Edge{tail: tail, weight: details[2]});
                    g.vertices[tail].edges.push(Edge{tail: head, weight: details[2]});
                },
                Err(e) => return Err(e),
            }
        }
 
        Ok(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let mut g = Graph::new();

        g.build(2);
        g.vertices[0].edges.push(Edge{weight: 10, tail: 1});
        g.vertices[1].edges.push(Edge{weight: 10, tail: 0});

        assert_eq!(g.vertices[1].label, 1);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.weight, 10);
        assert_eq!(g.vertices[e.tail].label, 1);
    }

    #[test]
    fn representation() {
        let p = Path::new("g1.txt");

        let g = Graph::from_file(p).ok().unwrap();

        assert_eq!(g.vertices[0].label, 0);
        assert_eq!(g.vertices[5].label, 5);
        assert_eq!(g.vertices[30].label, 30);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.weight, 6);
        assert_eq!(e.tail, 13);
        assert_eq!(g.vertices[e.tail].label, 13);

        let ref e = g.vertices[4].edges[1];
        assert_eq!(e.weight, 16);
        assert_eq!(e.tail, 17);
    }
}
