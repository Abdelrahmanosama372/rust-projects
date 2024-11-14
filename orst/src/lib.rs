pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;
pub mod quick_sort;
pub mod heap_sort;


trait Sorter {
    fn sort<T: Ord>(slice: &mut [T]);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bubble_sort_works() {
       let mut v = [3,5,2,1];
       bubble_sort::BubbleSort::sort(&mut v);
       assert_eq!(v, [1,2,3,5]);
    }
    #[test]
    fn selection_sort_works() {
       let mut v = [3,5,2,1];
       selection_sort::SelectionSort::sort(&mut v);
       assert_eq!(v, [1,2,3,5]);
    }
    #[test]
    fn insertion_sort_works() {
       let mut v = [3,5,2,1];
       insertion_sort::InsertionSort::sort(&mut v);
       assert_eq!(v, [1,2,3,5]);
    }
    #[test]
    fn quick_sort_works() {
       let mut v = [3,5,2,1];
       quick_sort::QuickSort::sort(&mut v);
       assert_eq!(v, [1,2,3,5]);
    }
    #[test]
    fn heap_sort_works() {
       let mut v = [3,5,2,1];
       heap_sort::HeapSort::sort(&mut v);
       assert_eq!(v, [1,2,3,5]);
    }
}
