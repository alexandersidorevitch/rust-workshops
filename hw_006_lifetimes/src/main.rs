extern crate core;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Cache<'a> {
    map: HashMap<&'a str, (&'a usize, Instant)>,
    ttl: BTreeMap<Instant, HashSet<&'a str>>,
}

impl<'a> Cache<'a> {
    pub fn new() -> Self {
        Self { map: HashMap::new(), ttl: BTreeMap::new() }
    }
}

impl<'a> Cache<'a> {
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl<'a> Cache<'a> {
    pub fn insert(&mut self, key: &'a str, value: &'a usize, ttl: Duration) -> Option<(&usize, Instant)> {
        let expire_time = Instant::now() + ttl;
        let prev_value = self.map.insert(key, (value, expire_time));
        if let Some(value) = prev_value {
            self.ttl.entry(value.1).and_modify(|x| {
                x.remove(key);
            });
            self.ttl.entry(expire_time).and_modify(|x| {
                x.insert(key);
            }).or_insert(
                HashSet::from([key])
            );
        } else {
            self.ttl.entry(expire_time).or_insert(
                HashSet::from([key])
            );
        }
        prev_value
    }

    pub fn remove(&mut self, key: &str) -> Option<(&usize, Instant)> {
        let deleted_value = self.map.remove(key);
        if let Some(value) = deleted_value {
            self.ttl.entry(value.1).and_modify(
                |x| {
                    x.remove(key);
                }
            );
        }
        deleted_value
    }

    pub fn get(&self, key: &str) -> Option<(&usize, Instant)> {
        self.map.get(key).map(|x| *x)
    }

    pub fn expire(&mut self) {
        self.ttl.retain(|key, value| {
            let delete_value = *key <= Instant::now();
            if delete_value {
                value.iter().for_each(|x| {
                    self.map.remove(*x);
                })
            }
            !delete_value
        })
    }
}

fn main() {
    let data = vec![
        (String::from("one"), Box::new(1), Duration::from_secs(5)),
        (String::from("two"), Box::new(2), Duration::from_millis(5)),
        (String::from("three"), Box::new(3), Duration::from_secs(5)),
    ];

    let mut cache = Cache::new();
    data.iter().for_each(|item| {
        cache.insert(&item.0, &item.1, item.2);
    });
    cache.insert("two", &2, Duration::from_secs(1));

    std::thread::sleep(Duration::from_secs(2));
    cache.expire();

    println!("{:#?}", cache);
}
