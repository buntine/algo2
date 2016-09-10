use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::Ordering;

pub struct Job {
    weight: i32,
    length: i32,
}

impl Job {
    fn ratio(&self) -> f32 {
        self.weight as f32 / self.length as f32
    }

    fn diff(&self) -> f32 {
        self.weight as f32 - self.length as f32
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.diff()
            .partial_cmp(&other.diff())
            .unwrap_or(Ordering::Equal)
            .reverse()
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        (self.weight, self.length) == (other.weight, other.length)
    }
}

impl Eq for Job { }

pub fn wsct(jobs: &mut Vec<Job>) -> i32 {
    jobs.sort_by(|a, b| a.cmp(b));
    return jobs.len() as i32;
}

pub fn jobs_from_file(path: &Path) -> Result<Vec<Job>, std::io::Error> {
    let file = try!(File::open(path));
    let buffer = BufReader::new(&file);
    let parse_job = |l: std::io::Result<String>| -> Job {
        let line = l.ok().unwrap();
        let details: Vec<i32> = line.split(" ")
                                     .map(|d| d.parse::<i32>().ok().unwrap())
                                     .collect();

        Job{weight: details[0], length: details[1]}
    };

    let jobs = buffer.lines()
                     .skip(1)
                     .map(parse_job)
                     .collect();

    return Ok(jobs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test1() {
        let p = Path::new("jobs.txt");
        let mut jobs: Vec<Job> = jobs_from_file(&p).ok().unwrap();
        let s = wsct(&mut jobs);

        assert_eq!(s, 10000);
    }

    #[test]
    fn test2() {
        let mut jobs = vec![
            Job{weight: 8, length: 50},
            Job{weight: 74, length: 59},
            Job{weight: 31, length: 73},
            Job{weight: 45, length: 79},
            Job{weight: 10, length: 10},
            Job{weight: 41, length: 66},
        ];
        let s = wsct(&mut jobs);

        assert_eq!(s, 31814)
    }
}
