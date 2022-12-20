use std::{
    cell::RefCell,
    collections::LinkedList,
    fmt::Display,
    rc::{Rc, Weak},
};

type Linked<T> = Option<Rc<RefCell<Node<T>>>>;
/// 双向链表节点
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Linked<T>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct DoubleLinkedList<T> {
    head: Linked<T>,
    tail: Linked<T>,
    len: usize,
}

impl<T: Copy> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            prev: None,
        }
    }
}

impl<T: Copy> DoubleLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    fn gen_linked_node(&self, node: Node<T>) -> Linked<T> {
        Some(Rc::new(RefCell::new(node)))
    }
    /// 在链表头部插入节点(头插法push front)
    fn push_front_node(&mut self, mut node: Node<T>) {
        let cur_head = self.head.take();

        if let Some(head_node) = cur_head {
            node.next = Some(head_node.clone());
            head_node.borrow_mut().prev = Some(Rc::downgrade(&head_node));
            self.head = self.gen_linked_node(node);
        } else {
            self.head = self.gen_linked_node(node);
            self.tail = self.head.clone();
        }
        self.len += 1;
    }
    pub fn push_front(&mut self, data: T) -> &mut Self {
        self.push_front_node(Node::new(data));
        self
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map_or(None, |node| -> Option<T> {
            self.head = node.borrow().next.clone();

            self.len -= 1;
            Some(node.borrow().data)
        })
    }

    fn push_back_node(&mut self, mut node: Node<T>) {
        let linked_node = self.gen_linked_node(node);
        if let Some(head_node) = self.head.clone() {
            self.tail.take().unwrap().borrow_mut().next = linked_node.clone();
            self.tail = linked_node.clone();
        } else {
            self.head = linked_node;
            self.tail = self.head.clone();
        }
        self.len += 1;
    }

    // 尾插
    pub fn push_back(&mut self, data: T) -> &mut Self {
        self.push_back_node(Node::new(data));
        self
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head {
            None => None,
            Some(ref head_node) => {
                let tail_node = self.tail.take().unwrap();
                let x = Some(tail_node.borrow().data);
                let prev = &tail_node.borrow_mut().prev;

                match prev {
                    None => {
                        self.tail = None;
                        self.head = None
                    }
                    Some(prev_node) => {
                        let mut new_tail_node = prev_node.upgrade();
                        new_tail_node.as_mut().unwrap().borrow_mut().next = None;
                        self.tail = new_tail_node;
                    }
                }

                self.len -= 1;
                x
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head.clone().map(|node| node),
            len: self.len,
        }
    }
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn contains(&self, data: &T) -> bool
    where
        T: PartialEq<T>,
    {
        match &self.head {
            None => false,
            Some(current) => {
                let mut cur_node = current.clone();
                if cur_node.borrow().data == *data {
                    return true;
                }
                while cur_node.borrow().next.is_some() {
                    let temp = cur_node.borrow().next.clone();
                    cur_node = temp.unwrap();

                    if cur_node.borrow().data == *data {
                        return true;
                    }
                }
                return false;
            }
        }
    }
    fn front(&self) -> Option<T> {
        return self.head.as_ref().map(|node| {
            let data = node.borrow().data;
            data
        });
    }
    // fn front_mut(&mut self) -> Option<&mut T> {
    //     return self.head.as_mut().map(|node| {
    //         let data = Ref::map(node.borrow(),|node_mut|&node_mut.data);
    //         data
    //     });
    // }
    fn back(&self) -> Option<T> {
        self.tail.as_ref().map(|tail_node| tail_node.borrow().data)
    }

    fn get_node_by_index(&mut self, index: usize) -> Linked<T> {
        assert!(index < self.len, "get_node_by_index out of range");
        let mut current = self.head.clone();
        let mut i = index;
        while i > 0 {
            current
                .clone()
                .map(|node| current = node.borrow().next.clone());
            i -= 1;
        }
        current
    }
    //     fn remove_v1(&mut self, at: usize) -> T {
    //         let len = self.len;
    //         assert!(
    //             at < len,
    //             "Cannot remove at an index outside of the list bounds"
    //         );

    //         // self.head.take().unwrap().data
    //         if at == 0 {
    //             let mut temp = self.head.take().unwrap();
    //             self.head = temp.next;
    //             return temp.data;
    //         } else {
    //             let mut prev = self.get_node_by_index(at - 1);
    //             let cur = prev.as_mut().and_then(|node| node.next.take());
    //             let temp = cur.unwrap();
    //             prev.unwrap().next = temp.next;
    //             return temp.data;
    //         }
    //     }
    //     // fn remove(&mut self, at: usize) -> T {
    //     //    let offset_from_end = len - at - 1;
    //     // if at <= offset_from_end {
    //     //     let mut cursor = self.cursor_front_mut();
    //     //     for _ in 0..at {
    //     //         cursor.move_next();
    //     //     }
    //     //     cursor.remove_current().unwrap()
    //     // } else {
    //     //     let mut cursor = self.cursor_back_mut();
    //     //     for _ in 0..offset_from_end {
    //     //         cursor.move_prev();
    //     //     }
    //     //     cursor.remove_current().unwrap()
    //     // }
    //     // }
    //     fn back_mut() {}
    //     fn split_off() {}
    //     fn drain_filter() {}
    fn from_iter(&mut self, vec: Vec<T>) -> &mut Self {
        for data in vec {
            self.push_back(data);
        }
        self
    }
    fn append(&mut self, other: &mut DoubleLinkedList<T>) -> &mut Self {
        self.tail.as_mut().unwrap().borrow_mut().next = other.head.clone();
        self.tail = other.tail.clone();
        self.len = self.len + other.len;
        self
    }
}

struct Iter<T: Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    len: usize,
}

impl<T: Copy> Iter<T> {
    fn next(&mut self) -> Option<T> {
        let head = self.head.take();
        match head {
            None => None,
            Some(ref head_node) => {
                self.head = head_node.borrow().next.clone();
                self.len -= 1;
                Some(head_node.borrow().data)
            }
        }
    }
}

impl<T: Copy, const N: usize> From<[T; N]> for DoubleLinkedList<T> {
    fn from(arr: [T; N]) -> Self {
        let mut ll: DoubleLinkedList<T> = DoubleLinkedList::new();
        ll.from_iter(Vec::from_iter(arr));
        ll
    }
}

impl<T: Display> Display for DoubleLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "None\n")?;
        } else {
            let mut next = self.head.clone();
            while let Some(node) = next.clone() {
                write!(f, "{} -> ", node.borrow().data)?;
                next = node.borrow().next.clone();
            }
            write!(f, "None\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    fn init_linked_list() -> DoubleLinkedList<usize> {
        DoubleLinkedList::from([1, 2, 3]) // 1->2->3->none
    }
    #[test]
    fn standardLinkedList() {
        let mut l = LinkedList::from([1, 2, 3]);
        let mut l2 = LinkedList::from([4, 5, 6]);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        l.front_mut();
        l.front();
        l.push_back(1);
        l.back();
        l.append(&mut l2);
    }
    #[test]
    fn push_front() {
        let mut ll = DoubleLinkedList::new();
        ll.push_front(3).push_front(2).push_front(1); // 1->2->3->none
        assert_eq!(ll.len, 3);
    }
    #[test]
    fn tail() {
        let ll = init_linked_list();
        let data = ll.tail.unwrap().borrow().data;
        assert_eq!(data, 3);
    }
    #[test]
    fn append() {
        let mut ll = DoubleLinkedList::from([1, 2, 3]);
        let mut ll2 = DoubleLinkedList::from([4, 5, 6]);
        ll.append(&mut ll2);
        assert_eq!(ll.len, 6);
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn pop_front() {
        let mut ll = DoubleLinkedList::new();
        ll.push_back(1).push_back(2).push_back(3); // 1->2->3->none
        assert_eq!(ll.pop_front().unwrap(), 1);
    }

    #[test]
    fn push_back() {
        let mut ll = DoubleLinkedList::new();
        ll.push_back(1).push_back(2).push_back(3);
        print!("{}", ll);
    }

    #[test]
    fn pop_back() {
        let mut ll = DoubleLinkedList::new();
        assert!(ll.pop_back().is_none());
        ll.push_back(1); // 1->none

        assert_eq!(ll.pop_back().unwrap(), 1);

        ll.push_back(1).push_back(2).push_back(3); // 1->2->3->none
        assert_eq!(ll.pop_back().unwrap(), 3);
    }
    #[test]
    fn back() {
        let mut ll = DoubleLinkedList::new();
        assert!(ll.back().is_none());
        ll.push_front(3).push_front(2).push_front(1); // 1->2->3->none
        assert_eq!(ll.back().unwrap(), 3)
    }
    #[test]
    fn contains() {
        let ll = init_linked_list();
        assert!(ll.contains(&1));
        assert!(ll.contains(&2));
        assert!(ll.contains(&3));
    }

    #[test]
    fn from() {
        let ll = DoubleLinkedList::from([1, 2, 3]);
        print!("{}", ll);
        assert_eq!(ll.len, 3)
    }
    #[test]
    fn clear() {
        let mut ll = init_linked_list();
        ll.clear();
        assert!(ll.is_empty());
    }
    #[test]
    fn iter() {
        let l = DoubleLinkedList::from([1, 2, 3]);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len, 0);

        let mut iter2 = l.iter();
        assert_eq!(iter2.next(), Some(1));
        assert_eq!(l.len(), 3);
        assert_eq!(iter2.len, 2);
    }
    #[test]
    fn front() {
        let l = DoubleLinkedList::from([1, 2, 3]);
        assert_eq!(l.front(), Some(1));
    }
    // #[test]
    // fn front_mut() {
    //     let mut l = DoubleLinkedList::from([1, 2, 3]);
    //     // assert_eq!(*l.front_mut().unwrap(), 1);
    //     // *l.front_mut().unwrap() = 2;
    //     // assert_eq!(*l.front_mut().unwrap(), 2);
    // }

    #[test]
    fn get_node_by_index() {
        let mut l = DoubleLinkedList::from([1, 2, 3]);
        assert_eq!(l.get_node_by_index(0).unwrap().borrow().data, 1);
        assert_eq!(l.get_node_by_index(1).unwrap().borrow().data, 2);
        assert_eq!(l.get_node_by_index(2).unwrap().borrow().data, 3);
    }
    #[test]
    #[should_panic(expected = "get_node_by_index out of range")]
    fn get_node_by_index_panic() {
        let mut l = DoubleLinkedList::from([1, 2, 3]);
        assert_eq!(l.get_node_by_index(4).unwrap().borrow().data, 3);
    }
    // #[test]
    // fn remove_v1() {
    //     let mut l = DoubleLinkedList::from([1, 2, 3]);
    //     assert_eq!(l.remove_v1(1), 2);
    //     assert_eq!(l.remove_v1(0), 1);
    //     assert_eq!(l.remove_v1(0), 3);
    // }
}
