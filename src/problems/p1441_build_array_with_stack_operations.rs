pub struct Solution;

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut target_ptr: usize = 0;
        let mut result: Vec<String> = vec![];

        for x in 1..n+1 {
            if target[target_ptr] == x {
                target_ptr += 1;
                result.push("Push".to_string());
            } else {
                result.push("Push".to_string());
                result.push("Pop".to_string());
            }
            if target_ptr == target.len() {
                break;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vec_string(x: Vec<&str>) -> Vec<String> {
      x.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::build_array(vec![1,3], 3), vec_string(vec!["Push", "Push", "Pop", "Push"]));
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::build_array(vec![1,2,3], 3), vec_string(vec!["Push", "Push", "Push"]));
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::build_array(vec![1,2], 4), vec_string(vec!["Push", "Push"]));
    }

}
