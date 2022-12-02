pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_box_list(){
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    }
}