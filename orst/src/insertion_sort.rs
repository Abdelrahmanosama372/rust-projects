use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort
{
    fn sort<T: Ord>(slice: &mut [T]) {
        let mut idx = 0;
        while idx < slice.len() {
            for i in idx..slice.len()
            {
                if slice[i] < slice[idx]
                {
                    slice[idx..=i].rotate_right(1);
                }
            }
            idx += 1;
        }
        
    }
}
