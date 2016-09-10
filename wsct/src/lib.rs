use std::path::Path;

pub struct Job {
    weight: i32,
    length: i32,
}

pub fn wsct(jobs: &[Job]) -> i32 {
    return jobs.len() as i32;
}

pub fn jobs_from_file(path: &Path) -> Result<Vec<Job>, String> {
    return Ok(vec![Job{weight: 1, length: 2}]);
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

        assert_eq!(s, 1)
    }
}
