use std::borrow::Borrow;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    from: String,
    to: String,
    pub attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new<F, T>(from: F, to: T) -> Self
    where
        F: ToString,
        T: ToString,
    {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
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
}
