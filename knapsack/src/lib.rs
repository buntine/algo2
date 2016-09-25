
#[derive(Debug)]
pub struct Item {
    value: i32,
    weight: i32,
}

impl Item {
    fn from_line(l: &str) -> Item {
        let ns: Vec<i32> = l.split(" ")
                            .take(2)
                            .map(|n| n.parse::<i32>().expect("Invalid input"))
                            .collect();

        Item{value: ns[0], weight: ns[1]}
    }
}

pub fn knapsack(items: Vec<Item>, w: i32) -> i32 {
    2
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
}
