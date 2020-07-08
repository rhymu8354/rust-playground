struct CachingXor<'a> {
    arr: &'a [i32],
    cache: std::collections::HashMap<(usize, usize), i32>,
}

impl<'a> CachingXor<'a> {
    fn new(arr: &'a [i32]) -> Self {
        CachingXor {
            arr,
            cache: std::collections::HashMap::new(),
        }
    }

    fn xor(&mut self, i: usize, j: usize) -> i32 {
        let key = (i, j);
        let arr = self.arr;
        match self.cache.get(&key) {
            Some(result) => *result,
            None => {
                let prefix = if i < j {
                    let prefix = (i, j-1);
                    match self.cache.get(&prefix) {
                        Some(result) => Some(*result),
                        None => None,
                    }
                } else {
                    None
                };
                match prefix {
                    Some(result) => result ^ arr[j],
                    None => {
                        let mut result = 0;
                        for n in arr.iter().take(j+1).skip(i) {
                            result ^= n;
                        }
                        result
                    },
                }
            }
        }
    }
}

struct Solution {}

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let n = arr.len();
        let mut caching_xor = CachingXor::new(&arr);
        for i in 0..n {
            for j in i+1..n {
                for k in j..n {
                    if caching_xor.xor(i, j - 1) == caching_xor.xor(j, k) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let arr = vec![2,3,1,6,7];
    println!("{:?} has {} triplets.", arr, Solution::count_triplets(arr.clone()));
    let arr = vec![1,1,1,1,1];
    println!("{:?} has {} triplets.", arr, Solution::count_triplets(arr.clone()));
    let arr = vec![2,3];
    println!("{:?} has {} triplets.", arr, Solution::count_triplets(arr.clone()));
    let arr = vec![1,3,5,7,9];
    println!("{:?} has {} triplets.", arr, Solution::count_triplets(arr.clone()));
    let arr = vec![7,11,12,9,5,2,7,17,22];
    println!("{:?} has {} triplets.", arr, Solution::count_triplets(arr.clone()));
}
