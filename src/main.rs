use test_bin_tokio_octocrab_gist::Result;
use test_bin_tokio_octocrab_gist::test;

#[tokio::main]
async fn main() -> Result<()> {
    test().await
}
