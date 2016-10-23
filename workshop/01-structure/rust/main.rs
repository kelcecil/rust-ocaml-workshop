use std::cmp::max;

enum Tree<T>{
    Empty,
    Node(T, Box<Tree<T>>, Box<Tree<T>>)
}

fn main() {
    let tree = Tree::Empty
}