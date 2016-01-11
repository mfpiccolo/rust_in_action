cd ~/code/first_server
cargo run
#    Compiling first_server v0.1.0 (file:///~/code/first_server)
# src/main.rs:10:9: 12:16 error: mismatched types:   #A
#  expected `&[u8]`,                                 #B
#     found `&'static str`                           #C
# (expected slice,
#     found str) [E0308]
# src/main.rs:10         "<div>
# src/main.rs:11            <img src=\"https://www.rust-lang.org/logos/rust-logo-blk.svg\">
# src/main.rs:12         </div>"
# src/main.rs:10:9: 12:16 help: run `rustc --explain E0308` to see a detailed explanation
# error: aborting due to previous error
# Could not compile `first_server`.
# #A The compiler telling us that it found a type mismatch
# #B The type that the compiler was expecting
# #C The type that the compiler found
