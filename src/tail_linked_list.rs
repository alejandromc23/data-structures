struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct TailLinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> TailLinkedList<T> {
    pub fn new() -> TailLinkedList<T> {
        Self { 
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        let head = self.head.take();

        let mut node = Box::new(Node {
            value,
            next: head,
        });

        self.head = Some(node);

        if self.tail.is_none() {
            self.tail = Some(node);
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        if self.is_empty() {
            self.push_front(value);
            return;
        }

        let new_node = Box::new(Node {
            value,
            next: None,
        });

        self.tail.as_mut().unwrap().next = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let node = self.head.take().unwrap();
        self.head = node.next;

        if self.head.is_none() {
            self.tail = None;
        }

        self.len -= 1;

        Some(node.value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }

        let mut current = self.head.as_mut();
        while current.unwrap().next.as_ref().unwrap().next.is_some() {
            current = current.unwrap().next.as_mut();
        }

        let last_node = current.unwrap().next.take();
        self.tail = current;
        self.len -= 1;

        // If last_node is Some, unwrap it and return the value
        last_node.map(|node| node.value)}

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn back(&self) -> Option<&T> {
        self.tail.as_ref().map(|node| &node.value)
    }

    pub fn remove(&mut self, position: usize) -> T {
        if position >= self.len {
            panic!("Index out of bounds");
        }

        if position == 0 {
            return self.pop_front().unwrap();
        } else if position == self.len - 1 {
            return self.pop_back().unwrap();
        }

        let mut current = self.head.as_mut().unwrap();
        let mut n = 0;

        while n + 1 < position {
            n += 1;
            current = current.next.as_mut().unwrap();
        }

        let node = current.next.take().unwrap();
        current.next = node.next;
        self.len -= 1;
        node.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let list: TailLinkedList<i32> = TailLinkedList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn push_front() {
        let mut list = TailLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.back(), Some(&1));
    }

    #[test]
    fn push_back() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
    }

    #[test]
    fn pop_front() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn pop_back() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn remove() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.remove(1), 2);
        assert_eq!(list.remove(0), 1);
        assert_eq!(list.remove(0), 3);
    }

    #[test]
    #[should_panic]
    fn remove_out_of_bounds() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.remove(3);
    }

    #[test]
    fn front() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.front(), Some(&1));
    }

    #[test]
    fn back() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.back(), Some(&3));
    }

    #[test]
    fn is_empty() {
        let mut list = TailLinkedList::new();
        assert!(list.is_empty());

        list.push_back(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn len() {
        let mut list = TailLinkedList::new();
        assert_eq!(list.len(), 0);

        list.push_back(1);
        assert_eq!(list.len(), 1);

        list.push_back(2);
        assert_eq!(list.len(), 2);

        list.push_back(3);
        assert_eq!(list.len(), 3);
    }
}
