fn binary_search(arr: &[usize], target: &usize, low: usize, high: usize) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid = (low + high) / 2;
    let guess = &arr[mid];

    if guess > target {
        return binary_search(arr, target, low, mid - 1);
    } else if guess < target {
        return binary_search(arr, target, mid + 1, high);
    } 

    Some(mid)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        use super::binary_search;

        let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 1..10 {
            assert_eq!(binary_search(&arr, &i, 0, arr.len()-1), Some(i - 1));
        }
    }
}

