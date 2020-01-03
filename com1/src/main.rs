// cargo test -- --nocapture
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
}

fn mod1(elements: Vec<i32>) -> Result<Vec<i32>, String> {
    let mut m = vec![];
    let mut n: i32 = 0;
    for e in elements.iter() {
        n += e;
        m.push(n);
    }
    Ok(m)
}

fn birthday_cake_candles(elements: Vec<i32>) -> Result<Vec<i32>, String> {
    let mut n: HashMap<&i32, i32> = HashMap::new();
    for e in &elements {
        match n.get(&e) {
            Some(&v) => {
                n.insert(&e, v + 1);
            }
            _ => {
                n.insert(&e, 1);
            }
        }
    }
    println!("{:?}", n);
    Ok(elements)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod1() -> Result<(), String> {
        #[derive(Debug)]
        struct StTestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }
        let case1 = StTestCase {
            input: vec![1, 2, 3, 4, 5],
            expected: vec![1, 3, 6, 10, 15],
        };
        assert_eq!(mod1(case1.input.to_vec())?, case1.expected);
        Ok(())
    }

    #[test]
    fn test_mod2() -> Result<(), String> {
        #[derive(Debug)]
        struct StTestCase {
            input: Vec<i32>,
            expected: Vec<i32>,
        }
        let case2 = StTestCase {
            input: vec![1, 2, 1],
            expected: vec![1, 3, 6, 10, 15],
        };
        assert_eq!(birthday_cake_candles(case2.input.to_vec())?, case2.expected);
        Ok(())
    }
}
