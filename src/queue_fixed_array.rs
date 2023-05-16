
pub struct Queue<T> {
    data: [T; 3],
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: [],
        }
    }
}
