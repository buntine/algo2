use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::num;

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
    cost: f32,
    tail: usize, // This should be a borrow of another Vertex but I couldn't work it out.
}

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
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

    fn from_plot(path: &Path) -> Result<Graph, std::io::Error> {
        let mut g = Graph{vertices: vec![]};
        let file = try!(File::open(path));
        let mut buffer = BufReader::new(&file);
        let mut first_line = String::new();
        let mut points: Vec<Point> = vec![];
        
        try!(buffer.read_line(&mut first_line));

        let vertice_count = first_line.trim().parse::<i32>().ok().expect("Invalid vertice count");

        g.build(vertice_count);

        // Collect points into a vector.
        for l in buffer.lines() {
            match l {
                Ok(parts) => {
                    let details = Graph::split_line::<f32>(&parts[..]);

                    points.push(Point{x: details[0], y: details[1]});
                }
                Err(e) => return Err(e),
            }
        }

        // Build complete graph form Euclidian distances of points.
        for i in 0..points.len() {
            let ref p1 = points[i];

            for n in (i+1)..points.len() {
                let ref p2 = points[n];
                let cost = ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt();

                g.vertices[i].edges.push(Edge{tail: n, cost: cost});
                g.vertices[n].edges.push(Edge{tail: i, cost: cost});
            }
        }

        Ok(g)
    }

    fn tsp(g: &mut Graph) -> i32 {
        10
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
        g.vertices[0].edges.push(Edge{cost: 10.0, tail: 1});
        g.vertices[1].edges.push(Edge{cost: 10.0, tail: 0});

        assert_eq!(g.vertices[1].label, 1);

        let ref e = g.vertices[0].edges[0];
        assert_eq!(e.cost, 10.0);
        assert_eq!(g.vertices[e.tail].label, 1);
    }

    #[test]
    fn representation() {
        let p = Path::new("tsp_simple1.txt");
        let mut g = Graph::from_plot(p).ok().unwrap();

        assert_eq!(g.vertices[0].label, 0);

        assert_eq!(g.vertices[0].edges.len(), 3);
        assert_eq!(g.vertices[1].edges.len(), 3);
        assert_eq!(g.vertices[2].edges.len(), 3);
        assert_eq!(g.vertices[3].edges.len(), 3);

        let ref e1 = g.vertices[0].edges[0];
        assert_eq!(format!("{:.3}", e1.cost), "2.236");

        let ref e1 = g.vertices[1].edges[0];
        assert_eq!(format!("{:.3}", e1.cost), "2.236");

        let ref e3 = g.vertices[0].edges[2];
        assert_eq!(format!("{:.3}", e3.cost), "3.162");

    }
}
