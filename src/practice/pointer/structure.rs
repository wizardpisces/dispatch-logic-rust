use std::cell::RefCell;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&List> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RCList {
    Cons(i32, RefCell<Box<RCList>>),
    Nil,
}
impl RCList {
    fn tail(&self) -> Option<&RefCell<Box<RCList>>> {
        match self {
            RCList::Cons(_, item) => Some(item),
            RCList::Nil => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_list() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        let node1 = List::Cons(1, Box::new(List::Nil));
        let node2 = List::Cons(2, Box::new(List::Nil));

        // if let Some(node1Item) = node1.tail(){
        //     *node1Item = node2
        // }
    }
    #[test]
    fn test_refcell_list() {
        let node1 = RCList::Cons(1, RefCell::new(Box::new(RCList::Nil)));
        let node2 = RCList::Cons(2, RefCell::new(Box::new(RCList::Nil)));
        println!("node1： {:?}", node1);
        println!("node2： {:?}", node2);
        if let Some(item1) = node1.tail() {
            *item1.borrow_mut() = Box::new(node2.clone())
        }
        if let Some(item2) = node2.tail() {
            *item2.borrow_mut() = Box::new(node1.clone())
        }
        println!("node1： {:?}", node1);
        println!("node2： {:?}", node2);
        println!("node1.tail()： {:?}", node1.tail());
        println!("node2.tail()： {:?}", node2.tail());
    }
}
