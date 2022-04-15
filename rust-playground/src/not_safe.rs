#[cfg(test)]
mod not_safe {
    #[test]
    fn unsafe_access_test() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    #[test]
    fn arbitrary_mem_access_test() {
        let address = 0 as u64;
        let raw_pointer = address as *const u64;

        // DO NOT DO THIS
        // unsafe {
        //     println!("r1 is: {}", *raw_pointer);
        // }
    }

    #[test]
    fn split_at_mut_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &v[..];
        let (a, b) = r.split_at(3);
        println!("a: {:?}", a);
        println!("a: {:?}", b);
    }

    fn my_split_at_mut(slice: &mut [i32], mid: usize)
                       -> (&mut [i32], &mut [i32]) {
        use std::slice;
        let len = slice.len();
        assert!(mid <= len);
        let ptr = slice.as_mut_ptr();

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    #[test]
    fn my_split_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let (a, b) = my_split_at_mut(&mut v[..], 3);
        println!("{:?}", a);
        println!("{:?}", b);
    }

    fn my_divide_at_mut(slice: &mut [i32], la: usize, ra: usize, lb: usize, rb: usize)
                        -> (&mut [i32], &mut [i32]) {
        use std::slice;
        assert!(ra >= la);
        assert!(rb >= lb);
        let ptr = slice.as_mut_ptr();

        unsafe {
            (
                slice::from_raw_parts_mut(ptr.add(la), ra - la),
                slice::from_raw_parts_mut(ptr.add(lb), rb - lb),
            )
        }
    }

    #[test]
    fn my_split_overlapped_test() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let (a, b) = my_divide_at_mut(&mut v[..], 0, 4, 2, 6);
        assert_eq!(a, [1, 2, 3, 4]);
        assert_eq!(b, [3, 4, 5, 6]);
        a[2] = 11;
        b[1] = 22;
        assert_eq!(a, [1, 2, 11, 22]);
        assert_eq!(b, [11, 22, 5, 6]);
    }
}