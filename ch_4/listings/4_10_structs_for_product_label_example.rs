#![feature(custom_derive)]

#[derive(Debug)]
struct Product {
    id: i32,
    name: &'static str,,
    label: &'static str,,
}

#[derive(Debug)]
struct Label {
    id: i32,
    text: &'static str,,
}
