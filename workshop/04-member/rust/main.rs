use std::cmp::max;

enum Tree<T>{
    Empty,
    Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

impl<T> Tree<T>
    where T: PartialOrd
{
    fn depth(&self) -> usize {
        match *self {
            Tree::Empty => 0,
            Tree::Node(_, ref left, ref right) =>
                1 + max(left.depth(), right.depth())
        }
    }

    fn insert(&mut self, value: T) -> &mut Self {
        match *self {
            Tree::Empty => {
                *self = Tree::Node(value, Box::new(Tree::Empty), Box::new(Tree::Empty));
            },
            Tree::Node(ref old_value, ref mut left, ref mut right) => {
                if &value < old_value {
                    left.insert(value);
                } else {
                    right.insert(value);
                }
            }
        }

        self
    }

    // You'd probably want to return the value instead of the key
    // normally, but the stored value is omitted for simplicity.
    fn member(self, search_key: T) -> Option<T> {
        match self {
            Tree::Empty => None,
            Tree::Node(node_key, left, right) => {
                if search_key == node_key {
                    Some(node_key)
                } else if search_key < node_key {
                    left.member(search_key)
                } else {
                    right.member(search_key)
                }
            }
        }
    }
}

#[test]
fn depth_test() {
    let tree = Tree::Node(5, 
        Box::new(
        Tree::Node(4, Box::new(Tree::Empty), Box::new(Tree::Empty))),
        Box::new(Tree::Empty));
    assert!(tree.depth() == 2)
}

#[test]
fn insert_test() {
    let mut tree = Tree::Empty;
    tree.insert(5);
    match tree {
        Tree::Node(x, _, _) => assert_eq!(x, 5),
        Tree::Empty => assert!(false)
    }
}

#[test]
fn member_test() {
    let mut tree = Tree::Empty;
    tree.insert(5);
    let node = tree.member(5);
    match node {
        Some(x) => assert_eq!(x, 5),
        None => assert!(false)
    }
}

fn main() {
    let mut tree = Tree::Empty;
    tree
        .insert(5)
        .insert(4)
        .insert(3)
        .insert(6)
        .insert(2);

    println!("Depth is: {}", tree.depth());
}
