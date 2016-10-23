open OUnit

type 'a tree =
    | Empty
    | Node of 'a * 'a tree * 'a tree
;;

let rec depth = function
    | Empty -> 0
    | Node(_, left, right) -> 1 + max (depth left) (depth right)

let rec insert x = function
    | Empty -> Node(x, Empty, Empty)
    | Node(value, left, right) ->
        if x < value then Node (value, insert x left, right)
        else Node (value, left, insert x right)

let rec member x = function
    | Empty -> Empty
    | Node(value, left, right) ->
        if x == value then Node(value, left, right)
        else if x < value then member x left
        else member x right

(* Unit tests *) 

let empty_tree = Empty
let one_sided_tree = Node(4, Node(3, Node(2, Empty, Empty), Empty), Empty)
let nicely_split_tree = Node(5, Node(3, Empty, Empty), Node(7, Empty, Empty))

let tree_depth _ =
    assert_equal 0 (depth empty_tree);
    assert_equal 3 (depth one_sided_tree);
    assert_equal 2 (depth nicely_split_tree)

let tree_insert _ =
    let new_tree = insert 4 Empty |> insert 3 |> insert 2 in
    assert_equal new_tree one_sided_tree;
    let new_tree = insert 5 Empty |> insert 3 |> insert 7 in
    assert_equal new_tree nicely_split_tree

let tree_member _ =
    let target_node = Node(3, Empty, Empty) in
    assert_equal target_node (member 3 nicely_split_tree);
    let target_node = Node(2, Empty, Empty) in
    assert_equal target_node (member 2 one_sided_tree)

let suite = "Binary Tree Tests" >::: ["tree_depth" >:: tree_depth; 
                                      "tree_insert" >:: tree_insert;
                                      "tree_member" >:: tree_member]

let _ =
  run_test_tt_main suite
