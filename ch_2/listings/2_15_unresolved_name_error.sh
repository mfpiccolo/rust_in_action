cd ~/code/reverse_string
cargo run
# Compiling reverse_string v0.1.0 (file:///path_to_file)
# src/main.rs:2:20: 2:34 error: unresolved name `reverse_string` [E0425]
# src/main.rs:2   let rev_string = reverse_string("Reverse Me");
#                                  ^~~~~~~~~~~~~~
# src/main.rs:2:20: 2:34 help: run `rustc --explain E0425` to see a detailed explanation
# error: aborting due to previous error
# Could not compile `reverse_string`.
