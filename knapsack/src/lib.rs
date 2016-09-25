use std::cmp;

#[derive(Debug)]
pub struct Item {
    value: i32,
    weight: i32,
}

impl Item {
    fn new(v: i32, w: i32) -> Item {
        Item{value: v, weight: w}
    }

    fn from_line(l: &str) -> Item {
        let ns: Vec<i32> = l.split(" ")
                            .take(2)
                            .map(|n| n.parse::<i32>().expect("Invalid input"))
                            .collect();

        Item{value: ns[0], weight: ns[1]}
    }
}

pub fn knapsack(items: Vec<Item>, weight: i32) -> i32 {
    let mut res: Vec<Vec<i32>> = vec![];

    for i in 0..items.len() {
        res.push(vec![]);
    }

    for i in 0..weight {
        res[0].push(0);
    }

    for i in 1..items.len() {
        for w in 0..weight {
            let a = res[(i-1)][w as usize];

            if items[i].weight > w {
                res[i].push(a);
            } else {
                let b = res[(i-1)][(w-items[i].weight) as usize] + items[i].value;

                res[i].push(cmp::max(a, b));
            }
        }
    }

    res[(items.len()-1)][(weight-1) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let items: Vec<Item> = vec![
            Item::new(4, 1),
            Item::new(2, 1),
            Item::new(6, 2),
            Item::new(7, 3),
        ];

        assert_eq!(knapsack(items, 5), 14);
    }
}
