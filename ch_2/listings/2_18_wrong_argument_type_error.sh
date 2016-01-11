cargo run
# src/main.rs:2:35: 2:47 error: mismatched types:
#  expected `collections::string::String`,
#     found `&'static str`
# (expected struct `collections::string::String`,
#     found &-ptr) [E0308]
# src/main.rs:2   let rev_string = reverse_string("Reverse Me");
#                                                 ^~~~~~~~~~~~
# src/main.rs:2:35: 2:47 help: run `rustc --explain E0308` to see a detailed explanation
# error: aborting due to previous error
# Could not compile `reverse_string`.
