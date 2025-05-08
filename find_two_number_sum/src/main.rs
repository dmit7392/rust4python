fn find_two_number_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for (i, &num1) in nums.iter().enumerate() {
        for (j, &num2) in nums.iter().enumerate().skip(i) {
            if num1 + num2 == target {
                return Some((i, j));
            }
        }
    }
    None
}

fn main() {} // do not add code

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_two_number_sum_no_solution() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(find_two_number_sum(&nums, 10), None);
    }

    #[test]
    fn find_two_number_sum_one_solution() {
        let nums = vec![1, 2, 3, 9, 5];
        assert_eq!(find_two_number_sum(&nums, 6), Some((0, 4)));
    }

    #[test]
    fn find_two_number_sum_no_solution_large() {
        let mut nums = [0; 50000];
        nums[49998] = 1;
        nums[49999] = 2;
        assert_eq!(find_two_number_sum(&nums, 4), None);
    }

    #[test]
    fn find_two_number_sum_one_solution_large() {
        let mut nums = [0; 50000];
        nums[49998] = 1;
        nums[49999] = 2;
        assert_eq!(find_two_number_sum(&nums, 3), Some((49998, 49999)));
    }
}
