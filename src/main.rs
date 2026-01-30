use test_bin_tokio_octocrab_workflow::Result;
use test_bin_tokio_octocrab_workflow::test;

#[tokio::main]
async fn main() -> Result<()> {
    test().await
}
