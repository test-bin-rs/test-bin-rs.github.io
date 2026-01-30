use test_bin_tokio_starter::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    test().await;
    Ok(())
}
