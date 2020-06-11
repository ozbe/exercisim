use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
  let mut stack = Vec::new();
  let brackets = {
    let mut map = HashMap::new();
    map.insert(']', '[');
    map.insert('}', '{');
    map.insert(')', '(');
    map
  };

  for c in string.chars() {
    if brackets.values().any(|&k| k == c) {
      stack.push(c);
    } if let Some(&b) = brackets.get(&c) {
      if stack.pop() != Some(b) {
        return false
      }
    }
  }

  stack.is_empty()
}