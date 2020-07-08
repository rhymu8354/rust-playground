struct Solution {}

impl Solution {
    fn wins(
        piles: &[i32],
        first: usize,
        last: usize,
        us: i32,
        them: i32,
    ) -> bool {
        let left = piles[first];
        let right = piles[last];
        match right - left {
            0 => us > them,
            _ => (
                !Self::wins(piles, first + 1, last, them, us + left)
                || !Self::wins(piles, first, last - 1, them, us + right)
            ),
        }
    }

    pub fn stone_game(piles: Vec<i32>) -> bool {
        Self::wins(&piles, 0, piles.len() - 1, 0, 0)
    }
}

fn main() {
    let piles = vec![5,3,4,5];
    println!("Alex wins for {:?}? {}", piles, Solution::stone_game(piles.clone()));
    let piles = vec![7,8,8,10];
    println!("Alex wins for {:?}? {}", piles, Solution::stone_game(piles.clone()));
    let piles = vec![3,2,10,4];
    println!("Alex wins for {:?}? {}", piles, Solution::stone_game(piles.clone()));
    let piles = vec![3,7,3,2,5,1,6,3,10,7];
    println!("Alex wins for {:?}? {}", piles, Solution::stone_game(piles.clone()));
}
