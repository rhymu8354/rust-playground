struct Solution {}

impl Solution {
    fn longest(s: &str) -> usize {
        let chars: Vec<(usize, char)> = s.char_indices().collect();
        let mut last = std::collections::HashMap::new();
        let n = chars.len();
        for i in 0..n {
            let j = n - i - 1;
            let c = chars[j];
            last.entry(c.1).or_insert(j);
        }
        let mut left = 0;
        let mut right = *last.get(&chars[left].1).unwrap();
        while left < right {
            right = std::cmp::max(
                right,
                *last.get(&chars[left].1).unwrap()
            );
            left += 1;
        }
        if right + 1 >= chars.len() {
            s.len()
        } else {
            chars[right + 1].0
        }
    }

    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut result = Vec::new();
        let mut left = 0;
        while left < s.len() {
            let next = Solution::longest(&s[left..]);
            result.push(next as i32);
            left += next;
        }
        result
    }
}

fn main() {
    let s = "ababcbacadefegdehijhklij";
    println!("partitions of {} are {:?}", s, Solution::partition_labels(String::from(s)));
    let s = "ababcbacad💩f💩gd💩hijhklij";
    println!("partitions of {} are {:?}", s, Solution::partition_labels(String::from(s)));
}
