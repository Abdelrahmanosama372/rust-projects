use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort
{
    fn sort<T: Ord>(slice: &mut [T]) {
        let mut not_sorted = true;
        while not_sorted {
            not_sorted = false;
            for i in 0..slice.len() - 1 {
                if slice[i+1] < slice[i] {
                    slice.swap(i, i+1);
                    not_sorted = true;
                }
            }
        }
    }
}
