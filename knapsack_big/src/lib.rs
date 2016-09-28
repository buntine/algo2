use std::cmp;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

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

pub fn knapsack(items: &[Item], weight: usize, store: &mut [HashMap<usize, i32>]) -> i32 {
    if items.len() <= 0 {
        return 0;
    } else if store[items.len() - 1].get(&weight).is_some() {
        return *store[items.len() - 1].get(&weight).unwrap();
    } else if items.len() == 1 {
        if items[0].weight <= weight {
            return items[0].value;
        } else {
            return 0;
        }
    } else {
        let butlast = &items[..(items.len() - 1)];
        let last = items.last().unwrap();

        let result = match last.weight > weight {
            true => knapsack(butlast, weight, store),
            false => cmp::max(knapsack(butlast, weight, store),
                              knapsack(butlast, weight - last.weight, store) + last.value),
        };

        store[items.len() - 1].insert(weight, result);

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::collections::HashMap;

    #[test]
    fn representation() {
        let item1 = Item::from_line("10 12");
        let item2 = Item::from_line("923 2006");

        assert_eq!(item1.value, 10);
        assert_eq!(item1.weight, 12);

        assert_eq!(item2.value, 923);
        assert_eq!(item2.weight, 2006);
    }

 //   #[test]
  //  fn knapsack1() {
//        let items: Vec<Item> =
//            vec![(4, 1), (2, 1), (6, 2), (7, 3)].iter()
//                                                .map(|&(v, w)| Item::new(v, w))
//                                                .collect();
//
//    let mut store: Vec<Vec<i32>> = 
//        [vec![-1; 6]].iter()
//                             .cycle()
//                             .take(items.len() + 1)
//                             .cloned()
//                             .collect();
//
//        assert_eq!(knapsack(&items[..], 5, &mut store[..]), 13);
//    }

//    #[test]
 //   fn knapsack2() {
  //      let items: Vec<Item> =
   //         vec![(7, 5), (2, 5), (7, 7), (9, 3),
   //              (5, 9), (4, 1), (2, 8), (6, 8),
    //             (1, 9), (7, 7)].iter()
   //                             .map(|&(v, w)| Item::new(v, w))
   //                             .collect();
//
//        assert_eq!(knapsack(&items[..], 40), 45);
//    }

    #[test]
    fn knapsack3() {
        let p = Path::new("knapsack1.txt");
        let items: Vec<Item> = items_from_file(p).ok().unwrap();
    let mut store: Vec<HashMap<usize, i32>> = 
        [HashMap::new()].iter()
                        .cycle()
                        .take(items.len() + 1)
                        .cloned()
                        .collect();

        assert_eq!(knapsack(&items[..], 10000, &mut store[..]), 2493893);
    }

    #[test]
    fn knapsack4() {
        let p = Path::new("knapsack2.txt");
        let items: Vec<Item> = items_from_file(p).ok().unwrap();
    let mut store: Vec<HashMap<usize, i32>> = 
        [HashMap::new()].iter()
                             .cycle()
                             .take(items.len() + 1)
                             .cloned()
                             .collect();

        assert_eq!(knapsack(&items[..], 2000000, &mut store[..]), 4243395);
    }
}
