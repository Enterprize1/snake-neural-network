
#[derive(Debug, Clone)]
pub struct RingBuffer<T> where T: PartialEq {
    buffer: Vec<T>,
    start: usize,
    end: usize,
    size: usize,
}

impl<T> RingBuffer<T> where T: PartialEq {
    pub fn new(preallocate_capacity: usize, size: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(preallocate_capacity),
            start: 0,
            end: 0,
            size
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() < self.size {
            self.buffer.push(item);
        } else {
            self.buffer[self.end] = item;
        }

        self.end = (self.end + 1) % self.size;
        if self.end == self.start {
            self.start = (self.start + 1) % self.size;
        }
    }

    pub fn extend(&mut self, item: T) {
        self.size += 1;
        self.buffer.insert(self.end, item);
    }

    pub fn first(&self) -> Option<&T> {
        if self.buffer.is_empty() {
            None
        } else if self.buffer.len() < self.size {
            Some(&self.buffer[self.end - 1])
        } else {
            Some(&self.buffer[(self.end + self.size - 1) % self.size])
        }
    }

    pub fn last(&self) -> Option<&T> {
        if self.buffer.is_empty() {
            None
        } else if self.buffer.len() < self.size {
            Some(&self.buffer[self.start])
        } else {
            Some(&self.buffer[self.end])
        }
    }

    pub fn contains(&self, item: &T) -> bool {
        return self.buffer.contains(item);
    }
}

#[cfg(test)]
mod tests {
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn can_push_items() {
        let mut result = RingBuffer::new(10, 3);

        result.push(1);
        result.push(2);
        result.push(3);

        contains_all(&result, vec![1, 2, 3]);
    }

    #[test]
    fn can_push_more_items_than_size() {
        let mut result = RingBuffer::new(10, 3);

        result.push(1);
        result.push(2);
        result.push(3);
        result.push(4);

        contains_all(&result, vec![2, 3, 4]);
    }

    #[test]
    fn can_expand_more_items_than_size() {
        let mut result = RingBuffer::new(10, 2);

        result.push(1);
        result.push(2);

        contains_all(&result, vec![1, 2]);

        result.extend(3);
        result.extend(4);

        result.push(5);

        result.extend(6);

        println!("{:?}", result);

        contains_all(&result, vec![1,2,3,5,6]);
    }

    #[test]
    fn correctly_calculates_first() {
        let mut result = RingBuffer::new(10, 3);

        result.push(1);
        assert_eq!(result.first(), Some(&1));

        result.push(2);
        assert_eq!(result.first(), Some(&2));

        result.push(3);
        assert_eq!(result.first(), Some(&3));

        result.push(4);
        assert_eq!(result.first(), Some(&4));

        result.push(5);
        assert_eq!(result.first(), Some(&5));

        result.push(6);
        assert_eq!(result.first(), Some(&6));

        result.extend(7);
        assert_eq!(result.first(), Some(&6));
    }

    #[test]
    fn correctly_calculates_last() {
        let mut result = RingBuffer::new(10, 3);

        result.push(1);

        assert_eq!(result.last(), Some(&1));

        result.push(2);

        assert_eq!(result.last(), Some(&1));

        result.push(3);

        assert_eq!(result.last(), Some(&1));

        result.push(4);

        assert_eq!(result.last(), Some(&2));

        result.push(5);

        assert_eq!(result.last(), Some(&3));

        result.push(6);

        assert_eq!(result.last(), Some(&4));

        result.extend(7);

        assert_eq!(result.last(), Some(&7));
    }

    fn contains_all(buffer: &RingBuffer<i32>, expected: Vec<i32>) {
        assert_eq!(buffer.size, expected.len());

        let mut min = expected.first().unwrap().clone();
        let mut max = expected.first().unwrap().clone();

        for x in expected {
            assert!(buffer.contains(&x));

            if x < min {
                min = x.clone();
            }

            if x > max {
                max = x.clone();
            }
        }

        assert!(!buffer.contains(&(min - 1)));
        assert!(!buffer.contains(&(max + 1)));
    }
}
