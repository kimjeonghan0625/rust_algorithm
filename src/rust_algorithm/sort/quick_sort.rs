pub fn quick_sort<T: PartialOrd>(array: &mut [T], first: usize, last: usize) {
    if first < last {
        let pivot = partition(array, first, last);
        quick_sort(array, first, pivot - 1);
        quick_sort(array, pivot + 1, last);
    }
}

fn partition<T: PartialOrd>(array: &mut [T], first: usize, last: usize) -> usize {
    let pivot = first;
    let mut left = first + 1;
    let mut right = last;
    loop {
        while left <= last && array[pivot] > array[left] {
            left += 1;
        }
        while array[pivot] < array[right] {
            right -= 1;
        }
        if left <= right {
            array.swap(left, right);
            continue;
        }
        break;
    }
    array.swap(pivot, right);
    right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_1() {
        let mut v = vec![3, 1, 5, 6, 2];
        let v_len = v.len();
        quick_sort(&mut v, 0, v_len - 1);
        assert_eq!(v, vec![1, 2, 3, 5, 6]);
    }

    #[test]
    fn test_quick_sort_2() {
        let mut v = vec![3.0, 1.0, 5.0, 6.0, 2.0];
        let v_len = v.len();
        quick_sort(&mut v, 0, v_len - 1);
        assert_eq!(v, vec![1.0, 2.0, 3.0, 5.0, 6.0]);
    }
}
