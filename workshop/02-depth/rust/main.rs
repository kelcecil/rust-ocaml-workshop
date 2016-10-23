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
}

#[test]
fn depth_test() {
    let tree = Tree::Node(5, 
        Box::new(
        Tree::Node(4, Box::new(Tree::Empty), Box::new(Tree::Empty))),
        Box::new(Tree::Empty));
    assert!(tree.depth() == 2)
}

fn main() {
    let tree = Tree::Empty;
    println!("Depth is: {}", tree.depth())
}