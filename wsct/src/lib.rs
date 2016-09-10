use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::error::Error;
use std::fs::File;

pub struct Job {
    weight: i32,
    length: i32,
}

pub fn wsct(jobs: &[Job]) -> i32 {
    return jobs.len() as i32;
}

pub fn jobs_from_file(path: &Path) -> Result<Vec<Job>, std::io::Error> {
    let mut file = try!(File::open(path));
    let mut buffer = BufReader::new(&file);
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
        let s = wsct(&jobs[..]);

        assert_eq!(s, 10000);
    }
}
