struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = nums.len() - 1;
        loop {
            if a + 1 == b && nums[a] <= 0 && nums[b] <= 0 {
                return std::cmp::max(nums[a], nums[b])
            }
            if nums[a] <= 0 && a < b {
                a += 1;
                continue;
            }
            if nums[b] <= 0 && a < b {
                b -= 1;
                continue;
            }
            if a + 1 < b && nums[a] + nums[a + 1] <= 0 {
                a += 2;
                continue;
            }
            if a + 1 < b && nums[b] + nums[b - 1] <= 0 {
                b -= 2;
                continue;
            }
            break;
        }
        nums[a..=b].iter().sum()
    }
}

fn main() {
    assert_eq!(187, Solution::max_sub_array(vec![31,-41,59,26,-53,58,97,-93,-23,84]));
    assert_eq!(0, Solution::max_sub_array(vec![-1,0,-2]));
    assert_eq!(6, Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]));
}
