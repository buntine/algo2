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
        let mut diff_jobs: Vec<Job> = jobs_from_file(&p, JobStrategy::Diff).ok().unwrap();
        let diff_s = wsct(&mut diff_jobs);

        let mut ratio_jobs: Vec<Job> = jobs_from_file(&p, JobStrategy::Ratio).ok().unwrap();
        let ratio_s = wsct(&mut ratio_jobs);

        assert_eq!(diff_s, 69119377652);
        assert_eq!(ratio_s, 67311454237);
    }

    #[test]
    fn test2() {
        let details = vec![[8, 50], [74, 59], [31, 73],
                           [45, 79], [10, 10], [41, 66]];
        let mut diff_jobs = details.iter().map(|d| Job{weight: d[0], length: d[1], strategy: JobStrategy::Diff}).collect();
        let diff_s = wsct(&mut diff_jobs);

        let mut ratio_jobs = details.iter().map(|d| Job{weight: d[0], length: d[1], strategy: JobStrategy::Ratio}).collect();
        let ratio_s = wsct(&mut ratio_jobs);

        assert_eq!(diff_s, 31814);
        assert_eq!(ratio_s, 31814);
    }

    #[test]
    fn test3() {
        let details = vec![[8, 50], [74, 59], [31, 73], [45, 79], [24, 10],
                           [41, 66], [93, 43], [88, 4], [28, 30], [41, 13]];
        let mut diff_jobs = details.iter().map(|d| Job{weight: d[0], length: d[1], strategy: JobStrategy::Diff}).collect();
        let diff_s = wsct(&mut diff_jobs);

        let mut ratio_jobs = details.iter().map(|d| Job{weight: d[0], length: d[1], strategy: JobStrategy::Ratio}).collect();
        let ratio_s = wsct(&mut ratio_jobs);

        assert_eq!(diff_s, 61545);
        assert_eq!(ratio_s, 60213);
    }

    #[test]
    fn test4() {
        let details = vec![
            [1, 37], [79, 39], [94, 16], [16, 73], [48, 44], [52, 40], [96, 27], [15, 86], [20, 81],
            [99, 57], [10, 90], [46, 66], [77, 52], [42, 74], [16, 45], [47, 4], [84, 41], [34, 54],
            [87, 53], [13, 69], [83, 88], [69, 63], [5, 97], [13, 65], [10, 46], [17, 10], [62, 79],
            [62, 32], [13, 12], [57, 61], [100, 98], [43, 7]];
        let mut diff_jobs = details.iter().map(|d| Job{weight: d[0], length: d[1], strategy: JobStrategy::Diff}).collect();
        let diff_s = wsct(&mut diff_jobs);

        let mut ratio_jobs = details.iter().map(|d| Job{weight: d[0], length: d[1], strategy: JobStrategy::Ratio}).collect();
        let ratio_s = wsct(&mut ratio_jobs);

        assert_eq!(diff_s, 688647);
        assert_eq!(ratio_s, 674634);
    }
}
