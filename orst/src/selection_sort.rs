use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort 
{
    fn sort<T: Ord>(slice: &mut [T]) {
        for i in 0..slice.len()
        {
            let (min_idx, _) = slice[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v).expect("slice is not empty");

            slice.swap(i, i + min_idx);
        }

    }
}
