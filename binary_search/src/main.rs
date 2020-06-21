fn find<T>(mut v: &[T], key: T) -> Option<usize>
    where T: Ord
{
    let mut offset = 0;
    loop {
        if v.is_empty() {
            return None;
        }
        let mid = v.len() / 2;
        match v[mid].cmp(&key) {
            std::cmp::Ordering::Less => {
                offset += mid + 1;
                v = &v[mid+1..];
            },
            std::cmp::Ordering::Equal => {
                return Some(mid + offset);
            },
            std::cmp::Ordering::Greater => {
                v = &v[..mid];
            }
        }
    }
}

fn main() {
    let data = vec![1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634];
    let value = 144;
    let index = find(&data, value).unwrap();
    println!("Index of {} in {:?} is {}", value, data, index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let data = vec![1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634];
        assert_eq!(Some(9), find(&data, 144));
    }

}
