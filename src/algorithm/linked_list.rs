/// 单链表节点
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// 单链表
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Self { data: data, next: None }
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }
}