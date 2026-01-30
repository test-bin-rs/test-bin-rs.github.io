use test_bin_tokio_starter::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    assert_eq!(hello_world().await, "Hello, world!");
    Ok(())
}
