pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // The count of numbers smaller than the current number is the same as the number's index
        // after the vector is sorted. Duplicates are bigger than their leftmost-index count.

        use std::collections::HashMap;
        let mut sorted_positions: HashMap<i32, i32> = HashMap::new();
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();
        for (i, n) in sorted_nums.iter().enumerate() {
            sorted_positions.entry(*n).or_insert(i as i32);
        }

        let mut result: Vec<i32> = vec![];
        for n in nums {
            let smaller_count = sorted_positions.get(&n).unwrap();
            result.push(*smaller_count);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
            vec![0, 0, 0, 0]
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        );
    }
}
