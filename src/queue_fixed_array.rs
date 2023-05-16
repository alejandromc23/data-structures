pub struct Queue<T, const N: usize> {
    data: [Option<T>; N],
    front: usize,
    rear: usize,
    size: usize,
}

impl<T: std::marker::Copy, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Self {
            data: [None; N],
            front: 0,
            rear: 0,
            size: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.size == N
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn enqueue(&mut self, value: T) {
        if self.is_full() {
            panic!("Queue is full");
        }

        self.data[self.rear] = Some(value);
        self.rear += 1;
        self.size += 1;

        if self.rear == N {
            self.rear = 0;
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            panic!("Queue is empty");
        }

        let value = self.data[self.front].take();
        self.front += 1;
        self.size -= 1;

        if self.front == N {
            self.front = 0;
        }

        value
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        let queue = super::Queue::<i32, 3>::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue() {
        let mut queue = super::Queue::<i32, 3>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_dequeue() {
        let mut queue = super::Queue::<i32, 3>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    #[should_panic]
    fn test_enqueue_panic() {
        let mut queue = super::Queue::<i32, 3>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
    }

    #[test]
    #[should_panic]
    fn test_dequeue_panic() {
        let mut queue = super::Queue::<i32, 3>::new();
        queue.dequeue();
    }

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = super::Queue::<i32, 3>::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        assert_eq!(queue.dequeue(), Some(1));
        queue.enqueue(4);
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
    }
}
