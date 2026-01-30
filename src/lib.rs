async fn hello_world() -> String {
    "Hello, world!".to_string()
}

pub async fn test() {
    assert_eq!(hello_world().await, "Hello, world!");
}
