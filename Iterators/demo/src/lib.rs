pub struct Counter {
    count: i32,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: -1}
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6  {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);    
    }

    #[test]
    fn iterator_test() {
        let mut counter = super::Counter::new();
        while counter.next() != None {
            assert_eq!(1, 1);
        } {}
    }
}
