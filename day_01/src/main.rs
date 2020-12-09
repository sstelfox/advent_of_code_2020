#![feature(iterator_fold_self)]

use std::fs::File;
use std::io::Read;

fn search_for_recursive_sum_target(list: &[usize], target: usize, depth: usize) -> Option<Vec<usize>> {
    for (pos, num) in list.iter().enumerate() {
        if *num > target {
            continue;
        }

        if depth == 1 {
            if target == *num {
                return Some(vec![*num]);
            }
        } else {
            if let Some(mut result) = search_for_recursive_sum_target(&list[(pos + 1)..], target - num, depth - 1) {
                result.push(*num);
                return Some(result);
            }
        }
    }

    None
}

fn main() {
    let mut in_dat_fh = File::open("./data/input1.txt").unwrap();
    let mut in_dat =String::new();

    in_dat_fh.read_to_string(&mut in_dat).unwrap();

    let input_entries: Vec<usize> = in_dat.lines().map(|i| i.parse::<usize>().unwrap()).collect();
    match search_for_recursive_sum_target(&input_entries, 2020, 2) {
        Some(res) => {
            println!("Found matching numbers ({:?}) in data set", res);
            let product: usize = res.into_iter().fold_first(|acc, x| acc * x).unwrap();
            println!("The product of the result is: {}", product);
        }
        None => {
            println!("Found no match for the first request");
        }
    }

    match search_for_recursive_sum_target(&input_entries, 2020, 3) {
        Some(res) => {
            println!("Found matching numbers ({:?}) in data set", res);
            let product: usize = res.into_iter().fold_first(|acc, x| acc * x).unwrap();
            println!("The product of the result is: {}", product);
        }
        None => {
            println!("Found no match for the second request");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // From the sample data given in the problem
    #[test]
    fn test_search_for_recursive_sum_target() {
        let matches = search_for_recursive_sum_target(&[1721, 979, 366, 299, 675, 1456], 2020, 2);
        assert!(matches.is_some());

        let results = matches.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results, vec![299, 1721]);
    }

    #[test]
    fn test_search_for_paired_sum_target_uses_unique_entries() {
        let matches = search_for_recursive_sum_target(&[50, 70], 100, 2);
        assert!(matches.is_none());
    }
}
