use std::fmt::Display;

type Linked<T> = Option<Box<Node<T>>>;
/// 单链表节点
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Linked<T>,
}

/// 单链表
#[derive(Debug)]
struct LinkedList<T> {
    head: Linked<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }
    /// 在链表头部插入节点(头插法push front)
    fn prepend(&mut self, data: T) -> &mut Self {
        self.head = Some(Box::new(Node {
            data,
            next: self.head.take(),
        }));

        self
    }

    fn tail(&self) -> &Linked<T> {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.next.is_none() {
                break;
            }
            current = &node.next;
        }
        current
    }   

    // 尾插
    fn append(&mut self, data: T) -> &mut Self {
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
    fn reverse() {}
    fn delete() {}
    fn head() {}
    fn lenth(&mut self) -> usize {
        let mut next = &self.head;
        let mut lenth = 0;
        while let Some(node) = next {
            lenth = lenth + 1;
            next = &node.next;
        }
        lenth
    }
    fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }
    fn pop_end(&mut self) -> Option<T> {
        match self.head.as_mut() {
            None => None,
            Some(mut current) => {
                while current.next.is_some() && current.next.as_ref().unwrap().next.is_some() {
                    current = current.next.as_mut().unwrap()
                }

                match current.next {
                    Some(_) => Some(current.next.take().unwrap().data),// link length >1
                    None => Some(self.head.take().unwrap().data), // link length = 1
                }
            }
        }
    }
}

impl<T: Display> Display for LinkedList<T> {
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
    #[test]
    fn test_prepend() {
        let mut ll = LinkedList::new();
        ll.prepend(3).prepend(2).prepend(1); // 1->2->3->none
        assert_eq!(ll.lenth(), 3);
    }
    #[test]
    fn test_tail() {
        let mut ll = LinkedList::new();
        assert!(ll.tail().is_none());
        ll.prepend(3).prepend(2).prepend(1); // 1->2->3->none
                                             // print!("{:?}", ll.tail());
        assert_eq!(ll.tail().as_ref().unwrap().data, 3)
    }
    #[test]
    fn test_append() {
        let mut ll = LinkedList::new();
        ll.append(1).append(2).append(3);
        print!("{}", ll);
    }
    #[test]
    fn test_pop_front() {
        let mut ll = LinkedList::new();
        ll.append(1).append(2).append(3); // 1->2->3->none
        assert_eq!(ll.pop_front().unwrap(), 1);
    }
    #[test]
    fn test_pop_end() {
        let mut ll = LinkedList::new();
        assert!(ll.pop_end().is_none());
        ll.append(1); // 1->none

        assert_eq!(ll.pop_end().unwrap(),1);
        ll.append(1).append(2).append(3); // 1->2->3->none
        assert_eq!(ll.pop_end().unwrap(), 3);
    }
}
