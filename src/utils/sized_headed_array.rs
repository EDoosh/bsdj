use std::convert::TryInto;

pub struct SizedHeadedArray<T: PartialEq + Clone + std::fmt::Debug, const S: usize> {
    arr: [Box<Option<T>>; S],
    head: usize,
    len: usize,
}

impl<T: PartialEq + Clone + std::fmt::Debug, const S: usize> SizedHeadedArray<T, S> {
    pub fn new() -> SizedHeadedArray<T, S> {
        let arr = vec![Box::new(None); S];
        let arr: [Box<Option<T>>; S] = arr.try_into().unwrap();

        SizedHeadedArray {
            arr,
            head: 0,
            len: S,
        }
    }

    /// Add an element to the array. If the amount of items exceeds the size,
    /// the array will replace the oldest element.
    ///
    /// # Example
    ///
    /// ```
    /// use utils::sized_headed_array::SizedHeadedArray;
    ///
    /// let mut sha = SizedHeadedArray::<i32, 3>::new();
    /// sha.add(1);
    /// sha.add(3);
    /// sha.add(15);
    /// // Array at this point will be [1, 3, 15]
    /// sha.add(6)
    /// // Array at this point will be [3, 15, 6]
    /// ```
    pub fn add(&mut self, item: T) {
        self.arr[self.head] = Box::new(Some(item));
        self.head = (self.head + 1) % self.len;
    }

    /// Check whether the previously input items match the values in `items`
    ///
    /// # Example
    ///
    /// ```
    /// use utils::sized_headed_array::SizedHeadedArray;
    ///
    /// let mut sha = SizedHeadedArray::<i32, 3>::new();
    /// sha.add(1);
    /// sha.add(2);
    /// assert!(sha.check_previous_items_match(&[2, 1]));
    /// sha.add(3);
    /// sha.add(4);
    /// assert!(sha.check_previous_items_match(&[4, 3, 2]));
    /// ```
    pub fn check_previous_items_match(&self, items: &[T]) -> bool {
        for (idx, check_item) in items.iter().enumerate() {
            // -1 from it as self.head points to the next element to be inserted.
            // Do all these conversions so we dont get an underflow
            let arr_idx =
                ((self.head as isize) - (idx as isize) - 1).rem_euclid(self.len as isize) as usize;
            let arr_item = &self.arr[arr_idx];
            // &**arr_item => dereference arr_item, dereference the arr_item box,
            // reference the arr_item value. Wtf
            if let Some(arr_item) = &**arr_item {
                if arr_item != check_item {
                    return false;
                }
            } else {
                // arr_item is none, meaning there is no boxed item.
                return false;
            }
        }

        true
    }
}
