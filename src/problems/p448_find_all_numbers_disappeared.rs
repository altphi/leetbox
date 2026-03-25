pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        #[allow(clippy::needless_range_loop)]
        for i in 0..nums.len() {
            let idx = (nums[i].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        }

        let mut result = vec![];
        for (i, n) in nums.iter().enumerate() {
            if *n > 0 {
                result.push((i + 1) as i32);
            }
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
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
    }
}
