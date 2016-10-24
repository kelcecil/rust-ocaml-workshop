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