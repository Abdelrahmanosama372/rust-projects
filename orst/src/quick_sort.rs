
use super::Sorter;

pub struct QuickSort;

impl Sorter for QuickSort
{
    fn sort<T: Ord>(slice: &mut [T]) {
        if slice.len() <= 1 {
            return;
        }else if slice.len() == 2 {
            if slice[0] > slice[1]
            {
                slice.swap(0, 1);
            }
            return;
        }

        let pivot = 0;
        let mut left = 1;
        let mut right = slice.len() - 1;

        while left <= right {
            if slice[left] > slice[pivot] && slice[right] < slice[pivot] {
                slice.swap(left, right);
            }

            if slice[left] < slice[pivot] {
                left += 1;
            }

            if slice[right] > slice[pivot] {
                right -= 1;
            }
        }

        slice.swap(pivot, right); 

        QuickSort::sort(&mut slice[..right]); 
        QuickSort::sort(&mut slice[right..]); 
    }
}
