use std::iter::FromIterator;

struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
          head: None,
        }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, element: T) {
      let next = self.head.take();
        let node = Node {
          data: element,
          next,
        };
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {
          let Node { data, next } = *n;
          self.head = next;
          data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
      <Self as Into<Vec<T>>>::into(self).into_iter().rev().collect()
    }

    fn iter<'a>(&'a self) -> SimpleLinkedListIterator<T> {
      SimpleLinkedListIterator::new(self)
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for i in iter {
          list.push(i)
        }

        list
    }
}

struct SimpleLinkedListIterator<'a, T> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> SimpleLinkedListIterator<'a, T> {
  fn new(list: &SimpleLinkedList<T>) -> SimpleLinkedListIterator<T> {
    SimpleLinkedListIterator {
      next: list.head.as_ref().map(|n| n.as_ref()),
    }
  }
}

impl<'a, T> Iterator for SimpleLinkedListIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|n| {
      let Node { data, next } = n;
      self.next = next.as_ref().map(|n| n.as_ref());
      data
    })
  }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
      let mut vec = Vec::new();

      while let Some(data) = self.pop() {
        vec.insert(0, data);
      }

      vec
    }
}
