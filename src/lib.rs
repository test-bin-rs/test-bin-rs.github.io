fn hello_world() -> String {
    "Hello, world!".to_string()
}

pub fn test() {
    assert_eq!(hello_world(), "Hello, world!");
}
