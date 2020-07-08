struct Solution {}

impl Solution {
    fn bit_flips(n: i32) -> i32 {
        let mut n0 = n;
        let mut n1 = n + 1;
        let mut count = 0;
        while n0 != n1 {
            count += 1;
            n0 /= 2;
            n1 /= 2;
        }
        count
    }

    fn sequence(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..2i32.pow(n as u32) {
            result.push(Solution::bit_flips(i));
        }
        result
    }

    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        let mut i = start;
        let mut result = Vec::new();
        for bit in Self::sequence(n) {
            result.push(i);
            i ^= 1 << (bit - 1);
        }
        result
    }
}

fn main() {
    println!("{:?}", Solution::circular_permutation(2, 3));
    println!("{:?}", Solution::circular_permutation(3, 2));
}
