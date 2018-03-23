extern crate rand;

use rand::{thread_rng, Rng};

pub fn binary_search<T: PartialOrd>(list: &[T], x: T) -> bool {
    let mut low = 0;
    let mut high = list.len() -1;

    while low <= high {
        let mid = (low + high) / 2;
        let elem = &list[mid];
        if x > *elem {
            low = mid + 1;
        } else if x < *elem {
            high = mid - 1;
        } else {
            return true;
        }
    }

    return false
}

pub fn min<T: PartialOrd>(list: &[T]) -> usize {
    let mut smallest = &list[0];
    let mut smallest_index = 0;

    for i in 1..list.len() {
        if smallest > &list[i] {
            smallest = &list[i];
            smallest_index = i;
        }
    }
    
    smallest_index
}

pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    for i in 0..list.len() {
        let min_index = i + min(&list[i..]);
        if i != min_index {
            list.swap(i, min_index);
            //let temp = list[i];
            //list[i] = list[min_index];
            //list[min_index] = temp;
        }
    }
}

fn partition<T: Ord + Copy>(slice: &mut [T]) -> usize {
    
    let mut rng = thread_rng();
    let index = rng.gen_range(0, slice.len());

    let p = slice[index];

    let mut v1 = Vec::<T>::new();
    let mut v2 = Vec::<T>::new();

    for i in 0..slice.len() {
        if slice[i] < p {
            v1.push(slice[i]);
        } else if slice[i] > p {
            v2.push(slice[i]);
        }
    }

    for i in 0..v1.len() {
        slice[i] = v1[i];
    }

    slice[v1.len()] = p;

    for i in 0..v2.len() {
        slice[i + v1.len() + 1] = v2[i];  
    }

    v1.len()
}

pub fn quicksort<T: Ord + Copy>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return; // Nothing to sort
    }

    let pivot_index = partition(slice);

    quicksort(&mut slice[.. pivot_index]);

    quicksort(&mut slice[pivot_index + 1 ..]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn binary_search_test() {
        let a = [1, 5, 6, 9, 11];
        assert_eq!(::binary_search(&a[..], 6), true);
        let b = [1, 5, 7, 8];
        assert_eq!(::binary_search(&b[..], 7), true);
        assert_eq!(::binary_search(&b[..], 6), false);
        let c = vec![1, 5, 7, 8];
        assert_eq!(::binary_search(&c[..], 7), true);
    }

    #[test]
    fn min_test() {
        let a = [6, 1, -5, 7];
        assert_eq!(::min(&a[..]), 2);
    }

    #[test]
    fn selection_sort_test() {
        let mut a = [6, 1, -5, 7];
        ::selection_sort(&mut a[..]);
        assert_eq!(&a[..], [-5, 1, 6, 7]);
        let mut b = ["ssas", "asasa", "dsds"];
        ::selection_sort(&mut b[..]);
        assert_eq!(&b[..], ["asasa", "dsds", "ssas"]);
        let mut v = vec!["ssas".to_string(), "asasa".to_string(), "dsds".to_string()];
        ::selection_sort(&mut v);
        assert_eq!(v, vec!["asasa".to_string(), "dsds".to_string(), "ssas".to_string()]);
    }

    #[test]
    fn quicksort_test() {
        let mut a = [6, 1, -5, 7];
        ::quicksort(&mut a);
        assert_eq!(&a[..], [-5, 1, 6, 7]);
        let mut b = ["ssas", "asasa", "dsds"];
        ::quicksort(&mut b[..]);
        assert_eq!(&b[..], ["asasa", "dsds", "ssas"]);
    }
}
