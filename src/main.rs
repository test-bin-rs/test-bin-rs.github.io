use test_bin_tokio_starter::test;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    test().await;
    Ok(())
}
