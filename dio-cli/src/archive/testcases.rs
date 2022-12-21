use dio::StoreCount;
use std::{collections::HashMap, error::Error, io::ErrorKind, ops::Range};

pub fn get_principles_testcases(index: u8) -> Result<String, Box<dyn Error>> {
    let mut cache: HashMap<u8, String> = HashMap::new();
    let range: Range<u8> = 1..StoreCount::Principles as u8 + 1;
    (range).for_each(|i| {
        let principle = format!("principle {}:", i);
        cache.insert(i, principle);
    });
    let principle = cache.get(&index);
    match principle {
        Some(principle) => Ok(principle.to_string()),
        None => Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound,
            "Principle not found",
        ))),
    }
}
pub fn get_facts_testcase(index: u8) -> Result<String, Box<dyn Error>> {
    let mut cache: HashMap<u8, String> = HashMap::new();
    let range: Range<u8> = 1..StoreCount::Facts as u8 + 1;
    for key in range {
        let fact = format!("fact {}:", key);
        cache.insert(key, fact);
    }
    let fact = cache.get(&index);
    match fact {
        Some(fact) => Ok(fact.to_string()),
        None => Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound,
            "Fact not found",
        ))),
    }
}
