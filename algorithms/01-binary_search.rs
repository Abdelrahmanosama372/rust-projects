

fn main() {
    let v = vec![1,2,3,4,5,6,12,56,78,99,102];
    
    assert_eq!(binary_search(&v, 1), Some(0));
    assert_eq!(binary_search(&v, 2), Some(1));
    assert_eq!(binary_search(&v, 3), Some(2));
    assert_eq!(binary_search(&v, 4), Some(3));
    assert_eq!(binary_search(&v, 5), Some(4));
    assert_eq!(binary_search(&v, 6), Some(5));
    assert_eq!(binary_search(&v, 12), Some(6));
    assert_eq!(binary_search(&v, 56), Some(7));
    assert_eq!(binary_search(&v, 78), Some(8));
    assert_eq!(binary_search(&v, 99), Some(9));
    assert_eq!(binary_search(&v, 102), Some(10));

    assert_eq!(binary_search(&v, 103), None);
    assert_eq!(binary_search(&v, 0), None);
    assert_eq!(binary_search(&v, -20), None);
}

fn binary_search(arr: &Vec<i32>, val: i32) -> Option<usize> {

    let mut low = 0 as usize;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        if arr[mid] == val {
            return Some(mid);
        }else {
            if arr[mid] < val {
                low = mid + 1;
            }else {
                if mid == 0 { break; }
                high = mid - 1;
            }
        }
    }

    None
}
