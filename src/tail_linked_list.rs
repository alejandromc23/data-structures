use std::ptr::NonNull;

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

pub struct TailLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
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
        let new_node = Box::new(Node {
            value,
            next: self.head,
        });

        let node = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        if self.tail.is_none() {
            self.tail = Some(node); 
        }

        self.head = Some(node); 
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
        let node = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        unsafe { self.tail.unwrap().as_mut().next = Some(node) };
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let node = self.head.take().unwrap();
        unsafe { self.head = node.as_ref().next; }

        if self.head.as_mut().is_none() {
            self.tail = None;
        }

        self.len -= 1;
        Some(unsafe { Box::from_raw(node.as_ptr()).value })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if unsafe { self.head.unwrap().as_ref().next.is_none() } {
            return self.pop_front();
        }
    
        let mut current = self.head;
        while unsafe { current.unwrap().as_ref().next.unwrap().as_ref().next.is_some() } {
            current = unsafe { current.unwrap().as_ref().next };
        }
            
        let old_node = unsafe { current.unwrap().as_mut().next.take().unwrap() };
        let old_tail = unsafe { Box::from_raw(old_node.as_ptr()) };
        self.tail = current;
        self.len -= 1;

        Some(old_tail.value)
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(unsafe { &self.head.unwrap().as_ref().value })
    }

    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(unsafe { &self.tail.unwrap().as_ref().value })
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

        let mut current = self.head.unwrap();
        let mut n = 0;

        while n + 1 < position {
            n += 1;
            current = unsafe { current.as_ref().next.unwrap() };
        }

        let node = unsafe { current.as_mut().next.take().unwrap() };
        unsafe { current.as_mut().next = node.as_ref().next };
        let old_node = unsafe { Box::from_raw(node.as_ptr()) };
        self.len -= 1;
        old_node.value 
    }
}

#[cfg(test)]
mod tests {
   use super::*; 

    #[test]
    fn test_new() {
        let list: TailLinkedList<i32> = TailLinkedList::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn test_push_front() {
        let mut list = TailLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(unsafe { list.tail.unwrap().as_ref().value }, 1);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.front(), Some(&3));
        assert_eq!(list.back(), Some(&1));
    }

    #[test]
    fn test_push_back() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.len(), 3);
        assert_eq!(unsafe { list.tail.unwrap().as_ref().value }, 3);
        assert_eq!(list.is_empty(), false);
        assert_eq!(list.front(), Some(&1));
        assert_eq!(list.back(), Some(&3));
    }

    #[test]
    fn test_pop_front() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(unsafe { list.head.unwrap().as_ref().value }, 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_pop_back() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(unsafe { list.tail.unwrap().as_ref().value }, 2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_remove() {
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
    fn test_remove_out_of_bounds() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.remove(3);
    }

    #[test]
    fn test_remove_front() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.remove(0), 1);
        assert_eq!(list.remove(0), 2);
        assert_eq!(list.remove(0), 3);
    }

    #[test]
    fn test_remove_back() {
        let mut list = TailLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.remove(2), 3);
        assert_eq!(list.remove(1), 2);
        assert_eq!(list.remove(0), 1);
    }

    #[test]
    fn test_front() {
        let mut list = TailLinkedList::new();
        assert_eq!(list.front(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.front(), Some(&1));
    }

    #[test]
    fn test_back() {
        let mut list = TailLinkedList::new();
        assert_eq!(list.back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.back(), Some(&3));
    }
}
