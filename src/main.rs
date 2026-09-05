#[derive(Debug, PartialEq, Clone)]
struct Node {
    next: Option<Box<Node>>,
    val: i32,
}

#[derive(Debug, PartialEq, Clone)]
struct List {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
}

impl List {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }
    pub fn to_vec(&self) -> Vec<i32> {
        let mut cur = self.head.as_ref();
        let mut v = Vec::new();
        while let Some(node) = cur {
            v.push(node.val);
            cur = node.next.as_ref();
        }
        v
    }

    pub fn push_back(&mut self, val: i32) {
        let mut tail = self.tail.as_mut();
        let new_node = Some(Box::new(Node { next: None, val }));
        match tail {
            Some(node) => node.next = new_node,
            None => {
                self.head = new_node;
            }
        };
    }

    // pub fn tail_mut(&mut self) -> &mut Node {
    //     let mut cur = self;
    //     while cur.next.is_some() {
    //         cur = cur.next.as_mut().unwrap();
    //     }
    //     cur
    // }
    //
    //
    // pub fn reverse(self) -> Node {
    //     let mut prev: Option<Box<Node>> = None;
    //     let mut cur: Option<Box<Node>> = Some(Box::new(self));
    //     while let Some(mut node) = cur {
    //         cur = node.next;
    //         node.next = prev;
    //         prev = Some(node);
    //     }
    //     *prev.unwrap()
    // }
    //
    // pub fn remove_by_value(&mut self) -> Node {}
    //
    // pub fn iter(&self) -> Iter {
    //     Iter { cur: Some(self) }
    // }
}

impl Node {
    pub fn new(val: i32) -> Self {
        Node { next: None, val }
    }
    pub fn to_vec(&self) -> Vec<i32> {
        let mut cur = self;
        let mut v = Vec::new();
        v.push(cur.val);
        while let Some(next) = &cur.next {
            cur = next;
            v.push(cur.val);
        }
        v
    }

    pub fn tail_mut(&mut self) -> &mut Node {
        let mut cur = self;
        while cur.next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }
        cur
    }

    pub fn push_back(&mut self, val: i32) {
        let mut tail = self.tail_mut();
        tail.next = Some(Box::new(Node::new(val)));
    }

    pub fn reverse(self) -> Node {
        let mut prev: Option<Box<Node>> = None;
        let mut cur: Option<Box<Node>> = Some(Box::new(self));
        while let Some(mut node) = cur {
            cur = node.next;
            node.next = prev;
            prev = Some(node);
        }
        *prev.unwrap()
    }

    pub fn iter(&self) -> Iter {
        Iter { cur: Some(self) }
    }
}

struct Iter<'a> {
    cur: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur {
            Some(node) => {
                self.cur = node.next.as_deref();
                Some(node.val)
            }
            None => None,
        }
    }
}

fn main() {
    let mut a = Node::new(5);
    a.next = Some(Box::new(Node::new(10)));
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_new() {
        let node = Node::new(5);
        assert_eq!(node, Node { next: None, val: 5 });
    }

    #[test]
    fn creates_new_list() {
        let list = List::new();
        assert_eq!(list, List { head: None });
    }

    #[test]
    fn pushes_to_back() {
        let mut head1 = Node::new(1);
        head1.push_back(2);
        head1.push_back(3);
        let head2 = Node {
            next: Some(Box::new(Node {
                next: Some(Box::new(Node::new(3))),
                val: 2,
            })),
            val: 1,
        };
        assert_eq!(head1, head2)
    }

    #[test]
    fn converts_to_vec() {
        let mut head = Node::new(1);
        head.push_back(2);
        head.push_back(3);
        let v = head.to_vec();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn reverses_list() {
        let mut head = Node::new(1);
        head.push_back(2);
        head.push_back(3);
        let reversed_head = head.reverse();
        assert_eq!(reversed_head.to_vec(), vec![3, 2, 1]);
    }

    #[test]
    fn can_be_iterated_over() {
        let mut head = Node::new(1);
        head.push_back(2);
        head.push_back(3);
        let v: Vec<i32> = head.iter().collect();
        assert_eq!(v, vec![1, 2, 3]);
    }
}
