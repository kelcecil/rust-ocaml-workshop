open OUnit

type 'a tree =
    | Empty
    | Node of 'a * 'a tree * 'a tree
;;

let rec depth = function
    | Empty -> 0
    | Node(_, left, right) -> 1 + max (depth left) (depth right)

(* Unit tests *) 

let empty_tree = Empty
let one_sided_tree = Node(4, Node(3, Node(2, Empty, Empty), Empty), Empty)
let nicely_split_tree = Node(5, Node(3, Empty, Empty), Node(7, Empty, Empty))

let tree_depth _ =
    assert_equal 0 (depth empty_tree);
    assert_equal 3 (depth one_sided_tree);
    assert_equal 2 (depth nicely_split_tree)

let suite = "Binary Tree Tests" >::: ["tree_depth" >:: tree_depth]

let _ =
  run_test_tt_main suite