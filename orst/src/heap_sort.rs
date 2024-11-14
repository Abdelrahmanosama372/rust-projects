use super::Sorter;

pub struct HeapSort;

impl Sorter for HeapSort 
{
    fn sort<T: Ord>(slice: &mut [T]) 
    {
        build_max_heap(slice);
        for i in (0..slice.len()).rev() {
            slice.swap(0, i);
            heapify_down(&mut slice[..i], 0);
        }
    }
}

fn build_max_heap<T: Ord>(slice: &mut [T])
{
    for i in (0..slice.len()/2 - 1).rev() {
        heapify_down(slice, i) 
    }
}

fn heapify_down<T: Ord>(slice: &mut[T], idx: usize)
{
    if slice.len() <= 1 {
        return;
    }

    let mut new_idx = idx;
    let left_idx = idx*2 + 1;
    let right_idx = idx*2 + 2;

    if left_idx < slice.len() && slice[new_idx] < slice[left_idx] {
        new_idx = left_idx;
    }

    if right_idx < slice.len() && slice[new_idx] < slice[right_idx] {
        new_idx = right_idx;
    }

    if idx != new_idx {
        slice.swap(idx, new_idx);
        heapify_down(slice, new_idx);
    } 
}
