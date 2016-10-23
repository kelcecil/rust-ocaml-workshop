open OUnit

let test_curry _ =
    let add x y = x + y in
    let add_10 = add 10 in
    assert_equal (add 10 5) (add_10 5)

let suite = "Curry Tests" >::: ["test_curry" >:: test_curry]

let _ =
  run_test_tt_main suite