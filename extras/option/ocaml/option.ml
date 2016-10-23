open OUnit

let greeting first_name =
    let name = match first_name with
    | Some x -> x
    | None -> "New User" in
    "Hello, " ^ name ^ "!"

let suite = "Greeting Tests" >::: [
    "test_named_greeting" >:: (fun _ ->
        assert_equal (greeting (Some "Kel")) "Hello, Kel!"
    );
    "test_anonymous_greeting" >:: (fun _ ->
        assert_equal (greeting None) "Hello, New User!"
    )
]

let _ =
  run_test_tt_main suite