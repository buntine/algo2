use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::Ordering;

#[derive(Debug)]
#[derive(Clone)]
pub enum JobStrategy {
    Diff,
    Ratio,
}

#[derive(Debug)]
pub struct Job {
    weight: i32,
    length: i32,
    strategy: JobStrategy,
}

impl Job {
    fn ratio(&self) -> f32 {
        self.weight as f32 / self.length as f32
    }

    fn diff(&self) -> f32 {
        self.weight as f32 - self.length as f32
    }

    fn value(&self) -> f32 {
        match self.strategy {
            JobStrategy::Diff => self.diff(),
            JobStrategy::Ratio => self.ratio(),
        }
    }
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = match self.value().partial_cmp(&other.value()).unwrap_or(Ordering::Equal) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                if self.weight > other.weight {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        };

        order.reverse()
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

pub fn wsct(jobs: &mut Vec<Job>) -> i64 {
    jobs.sort_by(|a, b| a.cmp(b));

    let mut sum: i64 = 0;
    let mut len: i64 = 0;

    for j in jobs {
        len += j.length as i64;
        sum += j.weight as i64 * len;
    }

    sum
}

pub fn jobs_from_file(path: &Path, strategy: JobStrategy) -> Result<Vec<Job>, std::io::Error> {
    let file = try!(File::open(path));
    let buffer = BufReader::new(&file);
    let parse_job = |l: std::io::Result<String>| -> Job {
        let line = l.ok().unwrap();
        let details: Vec<i32> = line.split(" ")
                                     .map(|d| d.parse::<i32>().ok().unwrap())
                                     .collect();

        Job{weight: details[0], length: details[1], strategy: strategy.clone()}
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
        let mut jobs: Vec<Job> = jobs_from_file(&p, JobStrategy::Diff).ok().unwrap();
        let s = wsct(&mut jobs);

        assert_eq!(s, 69119377652);
    }

    #[test]
    fn test2() {
        let mut jobs = vec![
            Job{weight: 8, length: 50, strategy: JobStrategy::Diff},
            Job{weight: 74, length: 59, strategy: JobStrategy::Diff},
            Job{weight: 31, length: 73, strategy: JobStrategy::Diff},
            Job{weight: 45, length: 79, strategy: JobStrategy::Diff},
            Job{weight: 10, length: 10, strategy: JobStrategy::Diff},
            Job{weight: 41, length: 66, strategy: JobStrategy::Diff},
        ];
        let s = wsct(&mut jobs);

        assert_eq!(s, 31814);
    }

    #[test]
    fn test3() {


    }
}
