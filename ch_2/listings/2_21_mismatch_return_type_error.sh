cargo run
#    Compiling reverse_string v0.1.0 (file:///path_to_file)
# src/main.rs:7:3: 7:38 error: mismatched types:
#  expected `()`,
#     found `collections::string::String`
# (expected (),
#     found struct `collections::string::String`) [E0308]
# src/main.rs:7   s.chars().rev().collect() as String
#                 ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
