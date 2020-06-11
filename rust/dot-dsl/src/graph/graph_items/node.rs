use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new<T>(name: T) -> Self
    where
        T: ToString,
    {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs<I, K, V>(mut self, attrs: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: ToString,
        V: ToString,
    {
        self.attrs = attrs
            .into_iter()
            .map(|i| {
                let (k, v) = i.borrow();
                (k.to_string(), v.to_string())
            })
            .collect();
        self
    }

    pub fn get_attr<T: ToString>(&self, attr: T) -> Option<&str> {
        self.attrs.get(&attr.to_string()).map(|k| k.as_str())
    }
}
