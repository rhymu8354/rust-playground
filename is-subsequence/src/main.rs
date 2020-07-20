struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let mut subs: Vec<usize> = Vec::new();
        for c in t {
            for sub in &mut subs {
                let k = *sub + 1;
                if c == s[k] {
                    if k + 1 == s.len() {
                        return true;
                    }
                    *sub = k;
                }
            }
            if c == s[0] {
                subs.push(0);
            }
        }
        false
    }
}

fn main() {
    assert_eq!(true, Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")));
    assert_eq!(false, Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")));
}
