use std::cmp;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Debug)]
pub struct Item {
    value: i32,
    weight: usize,
}

impl Item {
    fn new(v: i32, w: usize) -> Item {
        Item{value: v, weight: w}
    }

    fn from_line(l: &str) -> Item {
        let ns: Vec<i32> = l.split(" ")
                            .take(2)
                            .map(|n| n.parse::<i32>().expect("Invalid input"))
                            .collect();

        Item{value: ns[0], weight: ns[1] as usize}
    }
}

pub fn items_from_file(path: &Path) -> Result<Vec<Item>, std::io::Error> {
    let file = try!(File::open(path));
    let buffer = BufReader::new(&file);
    let items = buffer.lines()
                      .skip(1)
                      .map(|l| {
                          match l {
                              Ok(parts) => Item::from_line(&parts[..]),
                              Err(e) => panic!("Invalid input")
                          }
                      })
                      .collect();
 
    Ok(items)
}

pub fn knapsack(items: &[Item], weight: usize) -> i32 {
    let mut res: Vec<Vec<i32>> = 
        [vec![0; weight + 1]].iter()
                             .cycle()
                             .take(items.len() + 1)
                             .cloned()
                             .collect();

    for i in 0..items.len() {
        for w in 0..weight+1 {
            let a = res[i][w];

            if items[i].weight > w {
                res[i+1][w] = a;
            } else {
                let b = res[i][(w-items[i].weight)] + items[i].value;

                res[i+1][w] = cmp::max(a, b);
            }
        }
    }

    res[items.len()][weight]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn representation() {
        let item1 = Item::from_line("10 12");
        let item2 = Item::from_line("923 2006");

        assert_eq!(item1.value, 10);
        assert_eq!(item1.weight, 12);

        assert_eq!(item2.value, 923);
        assert_eq!(item2.weight, 2006);
    }

    #[test]
    fn knapsack1() {
        let items: Vec<Item> =
            vec![(4, 1), (2, 1), (6, 2), (7, 3)].iter()
                                                .map(|&(v, w)| Item::new(v, w))
                                                .collect();

        assert_eq!(knapsack(&items[..], 5), 13);
    }

    #[test]
    fn knapsack2() {
        let items: Vec<Item> =
            vec![(7, 5), (2, 5), (7, 7), (9, 3),
                 (5, 9), (4, 1), (2, 8), (6, 8),
                 (1, 9), (7, 7)].iter()
                                .map(|&(v, w)| Item::new(v, w))
                                .collect();

        assert_eq!(knapsack(&items[..], 40), 45);
    }

    #[test]
    fn knapsack3() {
        let p = Path::new("knapsack1.txt");
        let items: Vec<Item> = items_from_file(p).ok().unwrap();

        assert_eq!(knapsack(&items[..], 10000), 2493893);
    }
}
