use crate::tail_linked_list::TailLinkedList;

pub struct Queue<T> {
   data: TailLinkedList<T>,
}

impl<T> Queue<T> {
   pub fn new() -> Self {
      Self {
         data: TailLinkedList::new(),
      }
   }

   pub fn enqueue(&mut self, value: T) {
      self.data.push_front(value);
   }

   pub fn dequeue(&mut self) -> Option<T> {
      self.data.pop_back()
   }

   pub fn is_empty(&self) -> bool {
      self.data.is_empty()
   }
}

#[cfg(test)]
mod tests {

   #[test]
   fn test_new() {
      let queue = super::Queue::<i32>::new();
      assert!(queue.is_empty());
   }

   #[test]
   fn test_enqueue() {
      let mut queue = super::Queue::<i32>::new();
      queue.enqueue(1);
      queue.enqueue(2);
      queue.enqueue(3);
      assert!(!queue.is_empty());
   }

   #[test]
   fn test_dequeue() {
      let mut queue = super::Queue::<i32>::new();
      queue.enqueue(1);
      queue.enqueue(2);
      queue.enqueue(3);
      assert_eq!(queue.dequeue(), Some(1));
      assert_eq!(queue.dequeue(), Some(2));
      assert_eq!(queue.dequeue(), Some(3));
      assert_eq!(queue.dequeue(), None);
   }
}
