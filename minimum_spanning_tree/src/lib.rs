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
    cost: u32,
    tail: i32, // This should be a borrow of another Vertex but I couldn't work it out.
}

impl Graph {
    fn build(&mut self, size: i32) {
        for i in 0..size {
            self.nodes.push(Vertex{label: i, explored: false, adjacent: vec![]});
        }
    }

    fn split_line<T: std::str::FromStr>(line: &str) -> Vec<T> {
        line.split(" ")
            .map(|d| d.parse::<T>().ok().unwrap())
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
                    let details = Graph::split_line::<u32>(&parts[..]);
                    let ref mut node = g.nodes[details[0] as usize];

                    node.adjacent.push(Edge{tail: details[1] as i32, cost: details[2]});
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
