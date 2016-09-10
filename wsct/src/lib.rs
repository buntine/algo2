pub struct Job {
    weight: i32,
    length: i32,
}

pub fn wsct(jobs: &[Job]) -> i32 {
    return 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let size = 10;
        let jobs: Vec<Job> = vec![];
        let s = wsct(&jobs[..]);

        assert_eq!(1, 1)
    }
}
