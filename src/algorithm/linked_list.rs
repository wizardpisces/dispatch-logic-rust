use std::{collections::LinkedList, fmt::Display};

type Linked<T> = Option<Box<Node<T>>>;
/// 双向链表节点
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Linked<T>,
    // prev: Linked<T>,// box 外部可变引用数量受限，需要用 Rc 跟 RefCell 来实现；详情参加 double_linked_list
}

#[derive(Debug)]
struct LinkedListMock<T> {
    head: Linked<T>,
    // tail: Linked<T>,
    len: usize,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            next: None,
            // prev: None,
        }
    }
}

impl<T> LinkedListMock<T> {
    fn new() -> Self {
        Self {
            head: None,
            // tail: None,
            len: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// 在链表头部插入节点(头插法push front)
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        let cur_head = self.head.take();
        node.next = cur_head;
        self.head = Some(node);
        self.len += 1;
    }
    pub fn push_front(&mut self, data: T) -> &mut Self {
        self.push_front_node(Box::new(Node::new(data)));
        self
    }
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map_or(None, |node| -> Option<T> {
            self.head = node.next;

            self.len -= 1;
            Some(node.data)
        })
    }

    // 尾插
    pub fn push_back(&mut self, data: T) -> &mut Self {
        let new_node = Some(Box::new(Node::new(data)));
        match self.head.as_mut() {
            None => self.head = new_node,
            Some(mut current) => {
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = new_node;
            }
        }
        self.len += 1;
        self
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(mut current) => {
                while current.next.is_some() && current.next.as_ref().unwrap().next.is_some() {
                    current = current.next.as_mut().unwrap()
                }
                self.len -= 1;
                match current.next {
                    Some(_) => Some(current.next.take().unwrap().data), // link length >1
                    None => Some(self.head.take().unwrap().data),       // link length = 1
                }
            }
        }
    }

    pub fn reverse(&mut self) {
        if self.is_empty() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }
        // should has length >1
        let mut left = self.head.as_mut().unwrap().next.take();
        while left.is_some() {
            let mut taked_left = left.take().unwrap();
            left = taked_left.next;
            taked_left.next = self.head.take();
            self.head = Some(taked_left);
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head.as_ref().map(|node| node),
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
        match self.head.as_ref() {
            None => false,
            Some(mut current) => {
                if current.data == *data {
                    return true;
                }
                while current.next.is_some() {
                    current = &current.next.as_ref().unwrap();
                    if current.data == *data {
                        return true;
                    }
                }
                return false;
            }
        }
    }
    fn front(&self) -> Option<&T> {
        return self.head.as_ref().map(|node| return &node.as_ref().data);
    }
    fn front_mut(&mut self) -> Option<&mut T> {
        return self.head.as_mut().map(|node| {
            return &mut node.as_mut().data;
        });
    }
    fn back(&self) -> &Linked<T> {
        // optimize to O(1)
        let mut current = &self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                break;
            }
            current = &node.next;
        }
        current
    }

    fn get_node_by_index(&mut self, index: usize) -> Option<&mut Box<Node<T>>> {
        assert!(index < self.len, "get_node_by_index out of range");
        let mut cur = self.head.as_mut();
        let mut i = index.clone();
        while i > 0 {
            // cur = cur.map(|node|node.next).unwrap().as_mut();
            match cur {
                None => panic!("out of range"),
                Some(node) => cur = node.next.as_mut(),
            }
            i -= 1;
        }
        cur
    }
    fn remove_v1(&mut self, at: usize) -> T {
        let len = self.len;
        assert!(
            at < len,
            "Cannot remove at an index outside of the list bounds"
        );

        // self.head.take().unwrap().data
        if at == 0 {
            let mut temp = self.head.take().unwrap();
            self.head = temp.next;
            return temp.data;
        } else {
            let mut prev = self.get_node_by_index(at - 1);
            let cur = prev.as_mut().and_then(|node| node.next.take());
            let temp = cur.unwrap();
            prev.unwrap().next = temp.next;
            return temp.data;
        }
    }
    // fn remove(&mut self, at: usize) -> T {
    //    let offset_from_end = len - at - 1;
    // if at <= offset_from_end {
    //     let mut cursor = self.cursor_front_mut();
    //     for _ in 0..at {
    //         cursor.move_next();
    //     }
    //     cursor.remove_current().unwrap()
    // } else {
    //     let mut cursor = self.cursor_back_mut();
    //     for _ in 0..offset_from_end {
    //         cursor.move_prev();
    //     }
    //     cursor.remove_current().unwrap()
    // }
    // }
    fn back_mut() {}
    fn split_off() {}
    fn drain_filter() {}
    fn from_iter(&mut self, vec: Vec<T>) -> &mut Self {
        // vec.into_iter().for_each(move |ele| {
        //     self.push_back(ele);
        // });
        for data in vec {
            self.push_back(data);
        }

        self
    }
    fn append(&mut self, other: &mut LinkedListMock<T>)->&mut Self {
        // self.back().as_mut().map(|node|{
        //     node.next = other.head.take().unwrap().next
        // });
        self
    }
}

struct Iter<'a, T> {
    head: Option<&'a Box<Node<T>>>,
    len: usize,
}

impl<'a, T> Iter<'a, T> {
    fn next(&mut self) -> Option<&T> {
        self.head.map(|node| {
            self.head = node.next.as_ref();
            self.len -= 1;
            &node.data
        })
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedListMock<T> {
    fn from(arr: [T; N]) -> Self {
        let mut ll: LinkedListMock<T> = LinkedListMock::<T>::new();
        ll.from_iter(Vec::from_iter(arr));
        ll
    }
}

impl<T: Display> Display for LinkedListMock<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.head.is_none() {
            write!(f, "None\n")?;
        } else {
            let mut next = self.head.as_ref();
            while let Some(node) = next {
                write!(f, "{} -> ", node.data)?;
                next = node.next.as_ref();
            }
            write!(f, "None\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    fn init_linked_list() -> LinkedListMock<usize> {
        LinkedListMock::from([1, 2, 3]) // 1->2->3->none
    }
    #[test]
    fn standardLinkedList() {
        let mut l = LinkedList::from([1, 2, 3]);
        let mut l2 = LinkedList::from([4,5,6]);
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
        // assert_eq!(l.len(), 3);
        // l.remove(1);
        // assert_eq!(l.len(), 2);
    }
    #[test]
    fn push_front() {
        let mut ll = LinkedListMock::new();
        ll.push_front(3).push_front(2).push_front(1); // 1->2->3->none
        assert_eq!(ll.len, 3);
    }
    #[test]
    fn pop_front() {
        let mut ll = LinkedListMock::new();
        ll.push_back(1).push_back(2).push_back(3); // 1->2->3->none
        assert_eq!(ll.pop_front().unwrap(), 1);
    }

    #[test]
    fn push_back() {
        let mut ll = LinkedListMock::new();
        ll.push_back(1).push_back(2).push_back(3);
        print!("{}", ll);
    }

    #[test]
    fn pop_back() {
        let mut ll = LinkedListMock::new();
        assert!(ll.pop_back().is_none());
        ll.push_back(1); // 1->none

        assert_eq!(ll.pop_back().unwrap(), 1);
        ll.push_back(1).push_back(2).push_back(3); // 1->2->3->none
        assert_eq!(ll.pop_back().unwrap(), 3);
    }
    #[test]
    fn back() {
        let mut ll = LinkedListMock::new();
        assert!(ll.back().is_none());
        ll.push_front(3).push_front(2).push_front(1); // 1->2->3->none
                                                      // print!("{:?}", ll.tail());
        assert_eq!(ll.back().as_ref().unwrap().data, 3)
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
        let ll = LinkedListMock::from([1, 2, 3]);
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
        let l = LinkedListMock::from([1, 2, 3]);
        let mut iter = l.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len, 0);

        let mut iter2 = l.iter();
        assert_eq!(iter2.next(), Some(&1));
        assert_eq!(l.len(), 3);
        assert_eq!(iter2.len, 2);
    }
    #[test]
    fn front() {
        let l = LinkedListMock::from([1, 2, 3]);
        assert_eq!(l.front(), Some(&1));
    }
    #[test]
    fn front_mut() {
        let mut l = LinkedListMock::from([1, 2, 3]);
        assert_eq!(l.front_mut(), Some(&mut 1));
        *l.front_mut().unwrap() = 2;
        assert_eq!(l.front_mut(), Some(&mut 2));
    }

    #[test]
    fn get_node_by_index() {
        let mut l = LinkedListMock::from([1, 2, 3]);
        assert_eq!(l.get_node_by_index(0).unwrap().data, 1);
        assert_eq!(l.get_node_by_index(1).unwrap().data, 2);
        assert_eq!(l.get_node_by_index(2).unwrap().data, 3);
    }

    #[test]
    fn remove_v1() {
        let mut l = LinkedListMock::from([1, 2, 3]);
        assert_eq!(l.remove_v1(1), 2);
        assert_eq!(l.remove_v1(0), 1);
        assert_eq!(l.remove_v1(0), 3);
    }
}
