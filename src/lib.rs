pub mod const_utils {
    pub const fn split_left_at<T>(mut slice: &[T], at: usize) -> &[T] {
        while slice.len() != at {
            let [s @ .., _] = slice else { unreachable!() };
            slice = s;
        }
        slice
    }
    
    pub const fn split_right_at<T>(mut slice: &[T], at: usize) -> &[T] {
        let at = slice.len() - at - 1;
        while slice.len() != at {
            let [_, s @ ..] = slice else { unreachable!() };
            slice = s;
        }
        slice
    }

    pub const fn split_at<T>(slice: &[T], at: usize) -> (&[T], &[T]) {
        (split_left_at(slice, at), split_right_at(slice, at))
    }
    
    pub const fn split_right_u8(mut right: &[u8], value: u8) -> Option<&[u8]> {
        while let &[v, ref r @ ..] = right {
            if v == value {
                return Some(r);
            }
            right = r;
        }
        None
    }
    
    pub const fn index_of_u8(slice: &[u8], value: u8) -> Option<usize> {
        let mut window = slice;
        while let &[v, ref w @ ..] = window {
            if v == value {
                return Some(slice.len() - window.len());
            }
            window = w;
        }
        None
    }
}
