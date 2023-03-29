use std::{collections::HashMap, f32::NEG_INFINITY};

pub fn mean_imperative(v: &Vec<i32>) -> i32 {
    // add all up and divide by length
    let mut sum: i32 = 0;
    for i in v {
        sum += i;
    }
    sum / v.len() as i32
}

pub fn mean_declarative(v: &Vec<i32>) -> i32 {
    let sum: i32 = v.iter().sum();
    sum / v.len() as i32
}

pub fn median(v: &Vec<i32>) -> i32 {
    let mut v_copy = v.clone();
    v_copy.sort_unstable();
    let mid = v.len() / 2;
    v_copy[mid]
}

pub fn mode_imperative(v: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    // count occurrences
    for i in v {
        let count = map.entry(*i).or_insert(1);
        *count += 1;
    }

    // get mode
    let mut max_key = NEG_INFINITY as i32;
    let mut max_val = NEG_INFINITY as i32;
    for (key, val) in map.iter() {
        if max_val < *val {
            max_val = *val;
            max_key = *key;
        }
    }
    max_key
}

pub fn mode_declarative(v: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    // count occurrences
    for i in v {
        let count = map.entry(*i).or_insert(1);
        *count += 1;
    }

    *map.iter()
        .max_by(|x, y| x.1.cmp(y.1))
        .map(|(k, _)| k)
        .unwrap_or(&(0 as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_mean_imperatively() {
        let list = vec![2, 4, 3, 1];
        let result = mean_imperative(&list);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_calculates_mean_declaratively() {
        let list = vec![2, 4, 3, 1];
        let result = mean_declarative(&list);
        assert_eq!(result, 2);
    }

    #[test]
    fn it_calculates_median() {
        let list = vec![2, 4, 3, 1];
        let result = median(&list);
        assert_eq!(result, 3)
    }

    #[test]
    fn it_calculates_mode_imperatively() {
        let list = vec![2, 4, 1, 3, 1];
        let result = mode_imperative(&list);
        assert_eq!(result, 1)
    }

    #[test]
    fn it_calculates_mode_declaratively() {
        let list = vec![2, 4, 1, 3, 1, 2, 2, 3];
        let result = mode_declarative(&list);
        assert_eq!(result, 2)
    }
}
