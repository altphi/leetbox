pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut v: Vec<i32> = vec![0; nums.len()];
        let mut extra: i32 = -9;

        for n in nums.iter() {
            let i = (n - 1) as usize;
            v[i] += 1;
            if v[i] == 2 {
                extra = i as i32;
            }
        }

        let missing: i32 = v.iter().position(|x| *x == 0).unwrap() as i32;

        vec![extra + 1, missing + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
