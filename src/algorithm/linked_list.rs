use std::{collections::LinkedList, fmt::Display};

type Linked<T> = Option<Box<Node<T>>>;
/// 单链表节点
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Linked<T>,
}

/// 单链表
#[derive(Debug)]
struct LinkedListMock<T> {
    head: Linked<T>,
    tail: Linked<T>,
    len: usize,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> LinkedListMock<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /// 在链表头部插入节点(头插法push front)
    fn push_front(&mut self, data: T) -> &mut Self {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));

        self
    }
    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map_or(None, |node| -> Option<T> {
            self.head = node.next;

            Some(node.data)
        })
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.data)
        //     }
        // }
    }

    // 尾插
    fn push_back(&mut self, data: T) -> &mut Self {
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
        self
    }

    fn pop_back(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(mut current) => {
                while current.next.is_some() && current.next.as_ref().unwrap().next.is_some() {
                    current = current.next.as_mut().unwrap()
                }

                match current.next {
                    Some(_) => Some(current.next.take().unwrap().data), // link length >1
                    None => Some(self.head.take().unwrap().data),       // link length = 1
                }
            }
        }
    }

    fn reverse(&mut self) {
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

    fn len(&self) -> usize {
        let mut next = &self.head;
        let mut len = 0;
        while let Some(node) = next {
            len = len + 1;
            next = &node.next;
        }
        len
    }

    fn head() {}
    fn iter() {}
    fn clear() {}
    fn contains(&self, data: &T) -> bool
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
    fn front() {}
    fn front_mut() {}
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
    fn back_mut() {}
    fn split_off() {}
    fn remove() {}
    fn drain_filter() {}
    fn from_iter(&mut self, vec:Vec<T>) -> &mut Self {
        // vec.into_iter().for_each(move |ele| self.push_back(ele) );
        for data in vec {
            self.push_back(data);
        }
        self
    }
}



impl<T, const N: usize> From<[T; N]> for LinkedListMock<T> {
    fn from(arr: [T; N]) -> Self {
        let mut ll: LinkedListMock<T> = LinkedListMock::<T>::new();
        ll.from_iter(Vec::from_iter(arr));
        // LinkedList::contains(&self, x)
        // LinkedList::from([1,2,3]);
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
        let mut ll = LinkedListMock::new();
        // ll.from([1, 2, 3]);
        // LinkedList::from([1, 2, 3]);
        ll.push_back(1).push_back(2).push_back(3); // 1->2->3->none
        ll
    }
    #[test]
    fn push_front() {
        let mut ll = LinkedListMock::new();
        ll.push_front(3).push_front(2).push_front(1); // 1->2->3->none
        assert_eq!(ll.len(), 3);
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
    fn from(){
        let ll = LinkedListMock::from([1,2,3]);
        print!("{}", ll);
        assert_eq!(ll.len(),3)
    }
}
