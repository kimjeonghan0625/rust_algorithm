pub fn binary_search_r<T: Ord + Copy>(array: &Vec<T>, start: usize, end: usize, target: T) -> i32 {
    if start > end {
        -1
    } else {
        let mid = (start + end) / 2;
        match &array[mid].cmp(&target) {
            std::cmp::Ordering::Less => binary_search_r(array, mid+1, end, target),
            std::cmp::Ordering::Greater => binary_search_r(array, start, mid - 1, target),
            std::cmp::Ordering::Equal => mid as i32,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bsr() {
        let array = vec![2, 3, 9, 10, 17, 28, 31, 45];
        assert_eq!(binary_search_r(&array, 0, &array.len()-1, 31), 6);
        assert_eq!(binary_search_r(&array, 0, &array.len()-1, 9), 2);
        assert_eq!(binary_search_r(&array, 0, &array.len()-1, 40), -1);
    }
}
