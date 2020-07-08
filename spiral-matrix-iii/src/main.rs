struct Solution {}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn spiral_matrix_iii(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let result_size = (r * c) as usize;
        result.reserve(result_size);
        let mut double_step = 1;
        let mut dir = Direction::Right;
        let mut r0 = r0;
        let mut c0 = c0;
        while result.len() < result_size {
            double_step += 1;
            for _ in 0..(double_step / 2) {
                if r0 >= 0 && r0 < r && c0 >= 0 && c0 < c {
                    result.push(vec![r0, c0]);
                }
                match dir {
                    Direction::Right => c0 += 1,
                    Direction::Down => r0 += 1,
                    Direction::Left => c0 -= 1,
                    Direction::Up => r0 -= 1,
                }
            }
            dir = match dir {
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Up => Direction::Right,
            }
        }
        result
    }
}

fn main() {
    let v = Solution::spiral_matrix_iii(5, 6, 1, 4);
    println!("{:?}", v);
}
