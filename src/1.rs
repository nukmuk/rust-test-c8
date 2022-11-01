use std::{collections::HashMap, vec};

fn main() {
    let mut list = vec![6, 7, 7, 7, 8, 8, 9, 9, 10, 10, 10, 5, 4, 4, 4];
    list.sort_unstable();

    let median = median(&list);
    let mode = mode(&list);

    println!("list: {:?}", &list);
    println!("median: {:?}", &median);
    println!("mode: {:?}", &mode);
}

fn mode(list: &Vec<i32>) -> Vec<i32> {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();
    let mut highest_count = 0;

    for i in list {
        let count = occurrences.entry(*i).or_insert(0);
        *count += 1;

        if *count > highest_count {
            highest_count = *count;
        };        
    };

    let mut mode = Vec::new();

    for kv in occurrences {
        if kv.1 == highest_count {
            mode.push(kv.0);
        }
    }

    mode
}

fn median(list: &Vec<i32>) -> f64 {
    let length = list.len();

    if length % 2 == 0 {
        let first_index = (length - 1) / 2;
        let sec_index = (length - 1) / 2 + 1;

        let first = list[first_index];
        let sec = list[sec_index];

        (first as f64 + sec as f64) / 2.0
    } else {
        list[(length - 1) / 2] as f64
    }
}