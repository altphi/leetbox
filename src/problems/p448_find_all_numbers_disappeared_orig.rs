pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut counts: Vec<i32> = vec![0; nums.len()];

        for n in nums.iter() {
            let i = (*n-1) as usize;
            counts[i] += 1;
        }

        let mut result: Vec<i32> = vec![];
        for (i, n) in counts.iter().enumerate() {
            if *n == 0 {
              result.push((i+1) as i32);
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
        assert_eq!(Solution::find_disappeared_numbers(vec![4,3,2,7,8,2,3,1]), vec![5,6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1,1]), vec![2]);
    }
}

