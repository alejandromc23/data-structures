use std::alloc::{self, Layout};
use std::ops::Index;

pub struct Vector<T> {
    len: usize,
    capacity: usize,
    ptr: *mut T,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            capacity: 0,
            ptr: std::ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            let ptr = self.ptr.add(self.len);
            std::ptr::write(ptr, item);
        }

        self.len += 1;
    }

    pub fn insert(&mut self, position: usize, item: T) {
        if position > self.len {
            panic!("Insertion index ({}) is out of bounds len ({})", position, self.len);
        } 

        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            std::ptr::copy(
                self.ptr.add(position),
                self.ptr.add(position + 1),
                self.len - position,
                );

            std::ptr::write(self.ptr.add(position), item);

            self.len += 1;
        }
    }

    pub fn prepend(&mut self, item: T) {
        self.insert(0, item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe {
            Some(std::ptr::read(self.ptr.add(self.len)))
        }
    }

    pub fn remove(&mut self, position: usize) -> T {
        if position >= self.len {
            panic!("Removal index ({}) is out of bounds len ({})", position, self.len);
        }

        unsafe {
            self.len -= 1;
            let item = std::ptr::read(self.ptr.add(position));

            std::ptr::copy(
                self.ptr.add(position + 1),
                self.ptr.add(position),
                self.len - position,
            );
            
            item
        }
    }

    fn grow(&mut self) { 
        let new_capacity = if self.capacity == 0 {
            1
        } else {
            self.capacity * 2
        };

        let new_layout = Layout::array::<T>(new_capacity).unwrap();

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.ptr as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        if new_ptr.is_null() {
            std::alloc::handle_alloc_error(new_layout);
        }

        self.ptr = new_ptr as *mut T;
        self.capacity = new_capacity;
    }
}


impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Index out of bounds, the len is {} but the index is {}", self.len, index);
        }

        unsafe {
            &*self.ptr.add(index)
        }
    }
}
