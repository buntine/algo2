use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub struct Vertex {
    label: i32,
    explored: bool,
    leader: usize,
    edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Edge {
    cost: i32,
    head: usize,
    tail: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge { }

impl Graph {
    fn new() -> Graph {
        Graph{vertices: vec![]}
    }

    fn build(&mut self, size: i32) {
        for i in 0..size {
            self.vertices.push(Vertex{label: i, explored: false, leader: i as usize, edges: vec![]});
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

                    g.vertices[head].edges.push(Edge{head: head, tail: tail, cost: details[2]});
                    g.vertices[tail].edges.push(Edge{head: tail, tail: head, cost: details[2]});
                },
                Err(e) => return Err(e),
            }
        }
 
        Ok(g)
    }
}

// Inefficient O(n) implementation. Recursive implementation proved troublesome.
fn update_leaders(g: &mut Graph, start: usize, leader: usize) {
    for mut v in &mut g.vertices {
        if v.leader == start {
            v.leader = leader;
        }
    }
}

pub fn cluster_spacing(groups: usize, g: &mut Graph) -> i32 {
    let mut t = Graph::new();
    let mut edges: Vec<Edge> = vec![];
    let mut clusters = g.vertices.len();
    let mut sizes: Vec<i32> = vec![];

    t.build(clusters as i32);

    for v in &g.vertices {
        sizes.push(1);

        for e in &v.edges {
            edges.push(Edge{head: e.head, tail: e.tail, cost: e.cost});
        }
    }

    edges.sort_by(|a, b| a.cmp(b));

    for e in &edges {
        let hleader = t.vertices[e.head].leader;
        let tleader = t.vertices[e.tail].leader;

        if hleader != tleader {
            if clusters <= groups {
                return e.cost;
            }

            if sizes[hleader] >= sizes[tleader] {
                update_leaders(&mut t, tleader, hleader);
                sizes[hleader] += sizes[tleader];
                sizes[tleader] = 0;
            } else {
                update_leaders(&mut t, hleader, tleader);
                sizes[tleader] += sizes[hleader];
                sizes[hleader] = 0;
            }

            t.vertices[e.head].edges.push(Edge{head: e.head, tail: e.tail, cost: e.cost});
            t.vertices[e.tail].edges.push(Edge{head: e.tail, tail: e.head, cost: e.cost});

            clusters -= 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_works() {
        let mut g = Graph::new();

        g.build(2);
        g.vertices[0].edges.push(Edge{head: 0, cost: 10, tail: 1});
        g.vertices[1].edges.push(Edge{head: 1, cost: 10, tail: 0});

        assert_eq!(g.vertices[1].label, 1);
        assert_eq!(g.vertices[1].leader, 1);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.cost, 10);
        assert_eq!(g.vertices[e.tail].label, 1);
    }

    #[test]
    fn representation() {
        let p = Path::new("clustering1.txt");

        let g = Graph::from_file(p).ok().unwrap();

        assert_eq!(g.vertices[0].label, 0);
        assert_eq!(g.vertices[5].label, 5);
        assert_eq!(g.vertices[30].label, 30);

        assert_eq!(g.vertices[0].leader, 0);
        assert_eq!(g.vertices[1].leader, 1);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.cost, 6808);
        assert_eq!(e.tail, 1);
        assert_eq!(g.vertices[e.tail].label, 1);

        let ref e = g.vertices[0].edges[3];
        assert_eq!(e.cost, 3659);
    }

    #[test]
    fn execute() {
        let p = Path::new("clustering1.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(cluster_spacing(4, &mut g), 106)
    }

    #[test]
    fn simple1() {
        let p = Path::new("clustering_small1.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(cluster_spacing(4, &mut g), 2)
    }

    #[test]
    fn simple2() {
        let p = Path::new("clustering_small2.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(cluster_spacing(4, &mut g), 8)
    }

    #[test]
    fn simple3() {
        let p = Path::new("clustering_small3.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(cluster_spacing(4, &mut g), 7)
    }

    #[test]
    fn simple4() {
        let p = Path::new("clustering_small4.txt");
        let mut g = Graph::from_file(p).ok().unwrap();

        assert_eq!(cluster_spacing(4, &mut g), 17)
    }

}
