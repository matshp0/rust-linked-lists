use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Debug, Eq, PartialEq)]
struct Node<T> {
    val: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug, Eq, PartialEq)]
struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

struct IntoIter<T>(List<T>);

impl<T> Node<T> {
    pub fn new(val: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            val,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(node) = cur {
            node.borrow_mut().prev = None;
            cur = node.borrow_mut().next.take();
        }
        self.tail = None;
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.tail.take() {
            Some(link) => {
                new_node.clone().borrow_mut().prev = Some(link.clone());
                link.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node.clone());
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
        };
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Node::new(val);
        match self.head.take() {
            Some(link) => {
                new_node.clone().borrow_mut().next = Some(link.clone());
                link.borrow_mut().prev = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node.clone());
            }
        };
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|cur_head| {
            match cur_head.borrow_mut().next.take() {
                Some(next_head) => {
                    next_head.borrow_mut().prev = None;
                    self.head = Some(next_head.clone());
                }
                None => {
                    self.tail = None;
                    self.head = None
                }
            }
            Rc::try_unwrap(cur_head).ok().unwrap().into_inner().val
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|cur_tail| {
            match cur_tail.borrow_mut().prev.take() {
                Some(next_tail) => {
                    next_tail.borrow_mut().next = None;
                    self.tail = Some(next_tail.clone());
                }
                None => {
                    self.tail = None;
                    self.head = None
                }
            }
            Rc::try_unwrap(cur_tail).ok().unwrap().into_inner().val
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|el| Ref::map(el.borrow(), |node| &node.val))
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|el| Ref::map(el.borrow(), |node| &node.val))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.0.pop_back()
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
        assert_eq!(list.pop_front(), Some(1));
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
        let v: Vec<i32> = list.into_iter().collect();
        assert_eq!(v, vec![1, 2, 3, 4]);
    }

    #[test]
    #[ntest_timeout::timeout(100)]
    fn iterates_backwards() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let v: Vec<i32> = list.into_iter().rev().collect();
        assert_eq!(v, vec![4, 3, 2, 1]);
    }

    #[test]
    #[ntest_timeout::timeout(100)]
    fn drop_does_not_leak() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let middle = list.head.as_ref().unwrap().borrow().next.clone().unwrap();
        assert_eq!(Rc::strong_count(&middle), 3);

        drop(list);

        assert_eq!(Rc::strong_count(&middle), 1);
    }
}
