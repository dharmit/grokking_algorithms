use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::hash::{BuildHasher, Hash, Hasher};

// ModHasher is a custom hasher that hashes the bytes of a key using a simple module operation.
// Default modulus is 3 which can be overridden when using ::new()
pub struct ModHasher {
    // number which we want to use for modulo operation
    modulo: u64,
    // track the state of the hasher
    state: u64,
}

impl ModHasher {
    pub fn new(modulo: u64) -> ModHasher {
        let modulo = if modulo == 0 { 1 } else { modulo };

        ModHasher { modulo, state: 0 }
    }

    fn default() -> Self {
        // Default for performing modulo is 3
        Self::new(3)
    }
}

// Implement the Hasher trait for ModHasher
impl Hasher for ModHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_add(byte as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.state % self.modulo
    }
}

pub struct ModBuildHasher {
    modulo: u64,
}

impl ModBuildHasher {
    pub fn new(modulo: u64) -> Self {
        ModBuildHasher { modulo }
    }

    fn default() -> Self {
        Self::new(3)
    }
}

impl BuildHasher for ModBuildHasher {
    type Hasher = ModHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ModHasher::new(self.modulo)
    }
}

pub struct Map<K, V> {
    build_hasher: ModBuildHasher,
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> Map<K, V> {
    pub fn with_modulo(m: u64) -> Self {
        Self {
            build_hasher: ModBuildHasher::new(m),
            buckets: Vec::new(),
            len: 0,
        }
    }

    pub fn new() -> Self {
        Self::with_modulo(3)
    }
}

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    // Return a bucket for the given key
    fn bucket_for<Q>(&self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Hash + ?Sized,
    {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);

        (hasher.finish() as usize) % self.buckets.len()
    }

    // Grow the map by doubling its capacity
    fn grow(&mut self) {
        let old_bucket_count = self.buckets.len();
        let new_bucket_count = (self.buckets.len() * 2).max(4);
        self.buckets.resize_with(new_bucket_count, Vec::new);

        // Rehash and move pairs around
        for check_bucket in 0..old_bucket_count {
            let mut index = 0;
            while index < self.buckets[check_bucket].len() {
                let correct_bucket = self.bucket_for(&self.buckets[check_bucket][index].0);
                if check_bucket != correct_bucket {
                    let pair = self.buckets[check_bucket].swap_remove(index);
                    self.buckets[correct_bucket].push(pair);
                } else {
                    index += 1;
                }
            }
        }
    }

    const BUCKET_SIZE: usize = 8;

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // When the map is empty TODO: figure a way to initialize a non-empty map
        if self.buckets.is_empty() {
            self.grow();

            let bucket = self.bucket_for(&key);
            self.buckets[bucket].push((key, value));
            self.len += 1;
            return None;
        }
        let bucket = self.bucket_for(&key);
        let pair = self.buckets[bucket].iter_mut().find(|(k, _)| *k == key);
        match pair {
            Some((_, v)) => Some(std::mem::replace(v, value)),
            None => {
                self.buckets[bucket].push((key, value));
                self.len += 1;

                // Check if we have more than BUCKET_SIZE elements per bucket; grow if that's the case
                if Self::BUCKET_SIZE * self.buckets.len() < self.len {
                    self.grow();
                }
                None
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let bucket = self.bucket_for(&key);
        self.buckets[bucket]
            .iter()
            .find_map(|pair| (key == pair.0.borrow()).then_some(&pair.1))
    }
}

impl<K, V> Debug for Map<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_map();
        for pair in self.buckets.iter().flat_map(|b| b.iter()) {
            d.entry(&pair.0, &pair.1);
        }
        d.finish()
    }
}

fn main() {
    println!("1. Basic hashing\n");

    let mut hasher1 = ModHasher::default();
    std::hash::Hash::hash(&"hello", &mut hasher1);
    let hash1 = hasher1.finish();
    println!("    Hash of 'hello': {}", hash1);

    let mut hasher2 = ModHasher::new(29);
    std::hash::Hash::hash(&"world", &mut hasher2);
    let hash2 = hasher2.finish();
    println!("    Hash of 'world': {}", hash2);

    println!("\n2. Different modulo values\n");
    let test_string = "test string";
    for modulo in [3, 7, 10, 13, 15] {
        let mut hasher = ModHasher::new(modulo);
        std::hash::Hash::hash(&test_string, &mut hasher);
        let hash = hasher.finish();
        println!(
            "    Hash of '{}' for module value {} is {}",
            test_string, modulo, hash
        );
    }

    println!("\n3. Using our custom hasher with a HashMap");
    let build_hasher = ModBuildHasher::new(3);
    let mut map: HashMap<&str, i32, ModBuildHasher> = HashMap::with_hasher(build_hasher);

    map.insert("apple", 5);
    map.insert("banana", 2);
    map.insert("papaya", 1);
    map.insert("mango", 4);

    println!("    HashMap contents");
    for (key, value) in &map {
        let mut hasher = ModHasher::new(5);
        std::hash::Hash::hash(key, &mut hasher);
        let hash = hasher.finish();
        println!("      key: {}, value: {}, hash: {}", key, value, hash);
    }

    let mut map = Map::new();
    map.insert("pi", 31415);
    println!("\n{:?}", map.get("pi"));
    println!("{:?}", map.get("nosuchkey"));
}
