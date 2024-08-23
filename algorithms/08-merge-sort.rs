fn merge_sort(arr: &mut[i32], start: usize, end: usize) {
    if start >= end {
        return;
    }

    let mid = (start + end) / 2;
    merge_sort(arr, start, mid);
    merge_sort(arr, mid + 1, end);
    merge(arr, start, mid, end);
}

fn merge(arr: &mut[i32], start: usize, mid: usize, end: usize) {
    let (mut i, mut j, mut k) = (0,0,0);

    let left_length = mid - start + 1;
    let right_length = end - mid;

    let left_array = arr[start..=mid].to_vec();
    let right_array = arr[mid+1..=end].to_vec();

    while i < left_length && j < right_length {
        if left_array[i] <= right_array[j] {
            arr[start+k] = left_array[i];
            i += 1;
        }else {
            arr[start+k] = right_array[j];
            j += 1
        }
        k += 1;
    }

    while i < left_length {
        arr[start+k] = left_array[i];
        i += 1;
        k += 1;
    }

    while j < right_length {
        arr[start+k] = right_array[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = [5,8,2,15,4,9,24,1,0,13];

    let arr_length = arr.len();

    merge_sort(&mut arr,0, arr_length - 1);

    println!("{:?}", arr);
}