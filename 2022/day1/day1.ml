(*  *)
let file = "input.txt"

let () =
    (* Read file and display the first line *)
    let ic = open_in file in
    try
        let input = Std.input_list ic in
        print_endline input

        (* close the input channel *)
        close_in ic
    with e ->
        (* some unexpected exception occurs *)
        close_in_noerr ic;
        (* emergency closing *)
        raise e
