#[test]
fn run_program_products_test() {
    assert_eq!(
        "[Product { id: 1, name: \"Awesome Gaget\", label: \"awesome\" }, Product { id: 2, name: \"Cool Widget\", label: \"cool\" }, Product { id: 3, name: \"Rad Gizmo\", label: \"cool\" }, Product { id: 3, name: \"Lame Thingy\", label: \"lame\" }]".to_string(),
        run_program("ls products".to_string())
    );
}

#[test]
fn run_program_labels_test() {
    assert_eq!(
        "[Label { id: 1, text: \"cool\" }, Label { id: 1, text: \"awesome\" }, Label { id: 1, text: \"lame\" }]".to_string(),
        run_program("ls labels".to_string())
    );
}

#[test]
fn run_program_error_test() {
    assert_eq!(
        "Invalid command".to_string(),
        run_program("not a command".to_string())
    );
}
