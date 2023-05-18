#[derive(Clone)]
struct KeyValue<T> {
    key: String,
    value: T,
    is_deleted: bool,
}

struct HashTable<T> {
    data: Vec<Option<KeyValue<T>>>,
}

impl<T: PartialEq + Copy + Clone> HashTable<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::from([None]),
        }
    }

    // approach to the djb2 hash function
    fn hash(&self, key: &str) -> usize {
        let mut hash: usize = 5381;

        for c in key.chars() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(c as usize);
        }

        hash % self.data.len()
    }

    pub fn add(&mut self, key: String, value: T) {
        let mut index = self.hash(&key);
        let mut probes = 0;

        while self.data[index].is_some() && !self.data[index].as_ref().unwrap().is_deleted && self.data[index].as_ref().unwrap().key != key && probes < self.data.len() {
            index = (index + 1) % self.data.len();
            probes += 1;
        }

        if probes == self.data.len() {
            self.data.push(None);
            index = self.data.len() - 1;
        }

        self.data[index] = Some(KeyValue {
            key,
            value,
            is_deleted: false,
        });
    }

    pub fn exists(&self, key: &str) -> bool {
        let mut index = self.hash(key);
        let mut probes = 0;

        while self.data[index].is_some() && !self.data[index].as_ref().unwrap().is_deleted && probes < self.data.len()  {
            let key_value = self.data[index].as_ref().unwrap();
            if !key_value.is_deleted && &key_value.key == key {
                return true;
            }

            index = (index + 1) % self.data.len();
            probes += 1;
        }

        false
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let mut index = self.hash(key);
        let mut probes = 0;

        while self.data[index].is_some() && probes < self.data.len() {
            let key_value = self.data[index].as_ref().unwrap();
            if !key_value.is_deleted && &key_value.key == key {
                return Some(key_value.value);
            }

            index = (index + 1) % self.data.len();
            probes += 1;
        }

        None
    }

    pub fn remove(&mut self, key: &str) -> Option<T> {
        let mut index = self.hash(key);
        let mut probes = 0;

        while self.data[index].is_some() && probes < self.data.capacity() {
            let key_value = self.data[index].as_mut().unwrap();
            if !key_value.is_deleted && &key_value.key == key {
                key_value.is_deleted = true;
                return Some(key_value.value);
            }

            index = (index + 1) % self.data.len();
            probes += 1;
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to populate a hash table
    fn populate_hash_table(hash_table: &mut HashTable<i32>, from: i32, to: i32) {
        for i in from..to {
            let key = format!("key{}", i);
            hash_table.add(key.clone(), i);
        }
    }

    #[test]
    fn new() {
        let hash_table: HashTable<i32> = HashTable::new();
        assert_eq!(hash_table.data.capacity(), 1);
        assert_eq!(hash_table.data.len(), 1);
    }

    #[test]
    fn add() {
        let mut hash_table: HashTable<i32> = HashTable::new();
        populate_hash_table(&mut hash_table, 0, 3);

        assert_eq!(hash_table.data.len(), 3);
    }

    #[test]
    fn exists() {
        let mut hash_table: HashTable<i32> = HashTable::new();
        populate_hash_table(&mut hash_table, 0, 3);

        for i in 0..3 {
            let key = format!("key{}", i);
            assert!(hash_table.exists(&key));
        }

        assert!(!hash_table.exists("key10"));
    }

    #[test]
    fn get() {
        let mut hash_table: HashTable<i32> = HashTable::new();
        populate_hash_table(&mut hash_table, 0, 3);

        for i in 0..3 {
            let key = format!("key{}", i);
            assert_eq!(hash_table.get(&key), Some(i));
        }

        assert_eq!(hash_table.get("key10"), None);
    }

    #[test]
    fn remove() {
        let mut hash_table: HashTable<i32> = HashTable::new();
        populate_hash_table(&mut hash_table, 0, 3);

        for i in 0..3 {
            let key = format!("key{}", i);
            assert_eq!(hash_table.remove(&key), Some(i));
        }

        assert_eq!(hash_table.remove("key10"), None);
    }

    #[test]
    fn remove_and_add() {
        let mut hash_table: HashTable<i32> = HashTable::new();
        populate_hash_table(&mut hash_table, 0, 3);

        for i in 0..3 {
            let key = format!("key{}", i);
            assert_eq!(hash_table.remove(&key), Some(i));
        }

        populate_hash_table(&mut hash_table, 0, 3);

        for i in 0..3 {
            let key = format!("key{}", i);
            assert_eq!(hash_table.get(&key), Some(i));
        }
    }
}

