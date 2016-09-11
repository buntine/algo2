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
    23
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let mut g = Graph{vertices: vec![]};

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

        let mut g = Graph::from_file(p).ok().unwrap();

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
    fn simple1() {
        let p = Path::new("edges_simple1.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(mst(&mut g), 5)
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
