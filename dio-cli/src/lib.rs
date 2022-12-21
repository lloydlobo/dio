use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub enum StoreCount {
    Facts = 12isize,
    Principles = 14isize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DioFacts {
    pub facts: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DioPrinciples {
    pub principles: Vec<String>,
}
