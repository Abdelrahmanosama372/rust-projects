#[derive(Clone)]
pub struct KeyValuePair <T: Clone, U: Clone> {
    pub key: T,
    pub value: U
}

impl<T: Clone, U: Clone> KeyValuePair <T, U> {
    pub fn new(key: T, value: U) -> Self {
        KeyValuePair { key, value }
    }
}