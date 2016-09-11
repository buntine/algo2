use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct Graph {
    nodes: Vec<Vertex>,
}

pub struct Vertex {
    label: i32,
    explored: bool,
    adjacent: Vec<Edge>,
}

pub struct Edge {
    cost: i32,
    tail: usize, // This should be a borrow of another Vertex but I couldn't work it out.
}

impl Graph {
    fn build(&mut self, size: i32) {
        for i in 0..size {
            self.nodes.push(Vertex{label: i, explored: false, adjacent: vec![]});
        }
    }

    fn split_line<T: std::str::FromStr>(line: &str) -> Vec<T> {
        line.split(" ")
            .map(|d| { 
                let e = d.trim().parse::<T>();
                match e {
                    Ok(v) => v,
                    Err(ee) => panic!("Invalid: {:?}", d),
                }
            })
            .collect()
    }

    fn from_file(path: &Path) -> Result<Graph, std::io::Error> {
        let mut g = Graph{nodes: vec![]};
        let file = try!(File::open(path));
        let mut buffer = BufReader::new(&file);
        let mut first_line = String::new();
        
        try!(buffer.read_line(&mut first_line));

        let details = Graph::split_line::<i32>(&first_line[..]);

        g.build(details[0]);

        for l in buffer.lines().skip(1) {
            match l {
                Ok(parts) => {
                    let details = Graph::split_line::<i32>(&parts[..]);
                    let ref mut node = g.nodes[details[0] as usize];

                    node.adjacent.push(Edge{tail: details[1] as usize, cost: details[2]});
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
        let mut g = Graph{nodes: vec![]};

        g.build(2);
        g.nodes[0].adjacent.push(Edge{cost: 10, tail: 1});
        g.nodes[1].adjacent.push(Edge{cost: 12, tail: 0});

        assert_eq!(g.nodes[1].label, 1);

        let ref e = g.nodes[0].adjacent[0];
        assert_eq!(e.cost, 10);
        assert_eq!(g.nodes[e.tail].label, 1);
    }

    #[test]
    fn test2() {
        let p = Path::new("edges.txt");

        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(g.nodes[5].label, 5);
        assert_eq!(g.nodes[30].label, 30);

        let ref e = g.nodes[10].adjacent[0];
        assert_eq!(e.cost, 7353);
        assert_eq!(g.nodes[e.tail].label, 11);
    }
}
