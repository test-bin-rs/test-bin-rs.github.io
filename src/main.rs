use test_bin_tokio_echo::test;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let _ = test().await;
Ok(())
}
