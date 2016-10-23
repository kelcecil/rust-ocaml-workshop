open OUnit

class ['a] queue init = object
    val mutable q: 'a list = init

    method enqueue item = 
        q <- q @ item

    method dequeue =
        match q with
        | head :: tail ->
          q <- tail;
          Some head
        | [] -> None

    method peek =
        match q with
        | head :: _ ->
          Some head
        | [] -> None
end;;

let suite = "Queue Tests" >::: [
    "test_enqueue_one" >:: (fun _ ->
        let q = new queue [] in
          q#enqueue ["First"];
          match q#dequeue with
          | Some x -> assert_equal x "First"
          | None -> assert_failure "dequeue was none"
    );
    "test_peek" >:: (fun _ -> 
        let q = new queue [] in 
            q#enqueue ["Look!"];
                let peek_value = match q#peek with
                | Some x -> x
                | None -> assert_failure "peek was none"
            in
                let dequeue_value = match q#dequeue with
                | Some x -> x
                | None -> assert_failure "dequeue was none"
            in
            assert_equal peek_value dequeue_value
    )
]

let _ =
  run_test_tt_main suite