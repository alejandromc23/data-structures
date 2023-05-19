fn binary_search(arr: &[usize], target: usize) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        let guess = arr[mid];

        if guess == target {
            return Some(mid);
        } else if guess > target {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        use super::binary_search;

        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 1..10 {
            assert_eq!(binary_search(&arr, i), Some(i - 1));
        }
    }
}
