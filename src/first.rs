use std::mem;

pub struct List {
    head : Link
}

pub enum Link {
    Empty,
    More(Box<Node>),
}

pub struct Node {
    elem : i32,
    next : Link
}

impl List {
    pub fn new() -> Self {
        Self { head : Link::Empty}
    }

    pub fn push(&mut self , elem : i32) {
        let new_node = Node {
            elem : elem,
            next : mem::replace(&mut self.head, Link::Empty)
        };

        self.head = Link::More(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<i32> {
        let head_link = mem::replace(&mut self.head, Link::Empty);

        match head_link {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_head = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = curr_head {
            curr_head = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}