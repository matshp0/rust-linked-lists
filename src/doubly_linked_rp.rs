use std::{io::Cursor, marker::PhantomData};

#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    val: T,
    next: Option<*mut Node<T>>,
    prev: Option<*mut Node<T>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct List<T> {
    head: Option<*mut Node<T>>,
    tail: Option<*mut Node<T>>,
}

struct Iter<'a, T> {
    cur: Option<*mut Node<T>>,
    _marker: PhantomData<&'a T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Box::new(Node::new(val));
        let raw_pointer = Box::into_raw(new_node);
        match self.tail {
            Some(tail) => unsafe {
                (*tail).next = Some(raw_pointer);
                (*raw_pointer).prev = Some(tail);
            },
            None => {
                self.head = Some(raw_pointer);
            }
        };
        self.tail = Some(raw_pointer);
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Box::new(Node::new(val));
        let raw_pointer = Box::into_raw(new_node);
        match self.head {
            Some(head) => unsafe {
                (*head).prev = Some(raw_pointer);
                (*raw_pointer).next = Some(head);
            },
            None => {
                self.tail = Some(raw_pointer);
            }
        };
        self.head = Some(raw_pointer);
    }

    pub fn peek_back(&self) -> Option<&T> {
        self.tail.map(|el| unsafe { &(*el).val })
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.map(|el| unsafe { &(*el).val })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(cur_head) = self.head {
            unsafe {
                self.head = (*cur_head).next;
                if let Some(next_head) = self.head {
                    (*next_head).prev = None;
                } else {
                    self.tail = None;
                }
                Some(Box::from_raw(cur_head).val)
            }
        } else {
            None
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(cur_tail) = self.tail {
            unsafe {
                self.tail = (*cur_tail).prev;
                if let Some(next_tail) = self.tail {
                    (*next_tail).next = None;
                } else {
                    self.head = None;
                }
                Some(Box::from_raw(cur_tail).val)
            }
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            cur: self.head,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Some(cur) => unsafe {
                self.cur = (*cur).next;
                Some(&(*cur).val)
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ntest_timeout::timeout(100)]
    fn creates_new() {
        let list: List<i32> = List::new();
        assert_eq!(
            list,
            List {
                head: None,
                tail: None
            }
        );
    }

    #[test]
    #[ntest_timeout::timeout(100)]
    fn peeks() {
        let mut list: List<i32> = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(*list.peek_front().unwrap(), 1);
        assert_eq!(*list.peek_back().unwrap(), 3);
    }

    #[test]
    #[ntest_timeout::timeout(100)]
    fn pops() {
        let mut list: List<i32> = List::new();
        assert!(list.pop_front().is_none());
        assert!(list.pop_back().is_none());

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let v: Vec<i32> = list.iter().cloned().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
        assert_eq!(list.pop_front(), Some(1));

        let v: Vec<i32> = list.iter().cloned().collect();
        assert_eq!(v, vec![2, 3, 4]);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));

        assert_eq!(list, List::new());
    }

    #[test]
    #[ntest_timeout::timeout(100)]
    fn iterates() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let v: Vec<i32> = list.iter().cloned().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
    // #[test]
    // fn iterates_backwards() {
    //     let mut list = List::new();
    //     list.push_back(1);
    //     list.push_back(2);
    //     list.push_back(3);
    //     list.push_back(4);
    //     let v: Vec<i32> = list.into_iter().rev().collect();
    //     assert_eq!(v, vec![4, 3, 2, 1]);
    // }
}
