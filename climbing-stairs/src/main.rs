struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => 1,
            2 => 2,
            _ => {
                let mut n1 = 2;
                let mut n2 = 1;
                for _ in 3..=n {
                    let n0 = n1;
                    n1 += n2;
                    n2 = n0;
                }
                n1
            }
        }
    }
}

fn main() {
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
}
