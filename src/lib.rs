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
}
