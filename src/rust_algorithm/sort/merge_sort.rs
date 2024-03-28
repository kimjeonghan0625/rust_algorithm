pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 { return }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut temp_array = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut temp_array[..]);
    arr.copy_from_slice(&temp_array[..])
}

fn merge<T: PartialOrd + Copy>(a: &[T], b: &[T], temp: &mut [T]) {
    let mut a_index = 0;
    let mut b_index = 0;
    let mut temp_index = 0;
    while a_index < a.len() && b_index < b.len() {
        if a[a_index] <= b[b_index] {
            temp[temp_index] = a[a_index];
            temp_index += 1;
            a_index += 1;
        } else {
            temp[temp_index] = b[b_index];
            temp_index += 1;
            b_index += 1;
            
        }
    }
    if a_index < a.len() {
        temp[temp_index..].copy_from_slice(&a[a_index..]);
    }
    if b_index < b.len() {
        temp[temp_index..].copy_from_slice(&b[b_index..]);
    }
}

// pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
//     let mid = arr.len() / 2;
//     if mid == 0 {
//         return;
//     }

//     merge_sort(&mut arr[0..mid]);
//     merge_sort(&mut arr[mid..]);

//     let mut ret = arr.to_vec();

//     merge(&arr[0..mid], &arr[mid..], &mut ret[..]);

//     arr.copy_from_slice(&ret);
// }

// fn merge<T: PartialOrd + Copy>(a: &[T], b: &[T], ret: &mut [T]) {
//     let mut left = 0;
//     let mut right = 0;
//     let mut index = 0;

//     while left < a.len() && right < b.len() {
//         if a[left] <= b[right] {
//             ret[index] = a[left];
//             index += 1;
//             left += 1;
//         } else {
//             ret[index] = b[right];
//             index += 1;
//             right += 1;
//         }
//     }

//     if left < a.len() {
//         ret[index..].copy_from_slice(&a[left..]);
//     }
//     if right < b.len() {
//         ret[index..].copy_from_slice(&b[right..]);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 2, 4, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        let mut arr = [5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        let mut arr = [1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        let mut arr: [i32; 0] = [];
        merge_sort(&mut arr);
        assert_eq!(arr, []);

        let mut arr = [1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1]);
    }
}