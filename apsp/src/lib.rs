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
//                    g.vertices[tail].edges.push(Edge{tail: head, weight: details[2]});
                },
                Err(e) => return Err(e),
            }
        }
 
        Ok(g)
    }
}

pub fn apsp(g: &mut Graph) -> Result<i32, &'static str> {
    let len = g.vertices.len();
    let mut smallest = std::i32::MAX;
    let mut dist: Vec<Vec<i32>> = vec![
                       vec![std::i32::MAX; len];
                       len];

    for i in 0..len {
        dist[i][i] = 0;
    }

    for v in &g.vertices {
        for e in &v.edges {
            dist[v.label as usize][e.tail] = e.weight;
        }
    }

    for k in 0..len {
        if dist[k][k] < 0 {
            return Err("Negative cycle");
        }

        for i in 0..len {
            for j in 0..len {
                if dist[i][k] == std::i32::MAX || dist[k][j] == std::i32::MAX {
                    continue;
                }

                dist[i][j] = std::cmp::min(dist[i][j], dist[i][k] + dist[k][j]);

                if dist[i][j] < smallest {
                    smallest = dist[i][j];
                }
            }
        }
    }

    Ok(smallest)
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

    #[test]
    fn simple1() {
        let p = Path::new("g_simple1.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert!(apsp(&mut g).is_ok());
        assert_eq!(apsp(&mut g).ok().unwrap(), -6);
    }

    #[test]
    fn simple2() {
        let p = Path::new("g_simple2.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert!(apsp(&mut g).is_err());
    }

    #[test]
    fn simple3() {
        let p = Path::new("g_simple3.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert!(apsp(&mut g).is_ok());
        assert_eq!(apsp(&mut g).ok().unwrap(), -10003);
    }

    //#[test]
  //  fn large1() {
 //       let p = Path::new("g1.txt");
 //       let mut g = Graph::from_file(p).ok().unwrap();
//
//        assert!(apsp(&mut g).is_err());
//    }

//    #[test]
//    fn large2() {
//        let p = Path::new("g2.txt");
//        let mut g = Graph::from_file(p).ok().unwrap();
//
//        assert!(apsp(&mut g).is_err());
 //   }

    #[test]
    fn large3() {
        let p = Path::new("g3.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(apsp(&mut g).ok().unwrap(), -19);
    }
}
