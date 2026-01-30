use octocrab::Octocrab;
pub use octocrab::Result;
pub async fn test() -> Result<()> {
    let octocrab = Octocrab::builder().build()?;
    let runs = octocrab
        .workflows("test-bin-rs", "test-bin-rs.github.io")
        .list_all_runs()
        .branch("test-bin-tokio-octocrab-workflow")
        .per_page(1)
        .send()
        .await?;

    assert_eq!(
        format!("{}", runs.items.first().unwrap().head_branch),
        "test-bin-tokio-octocrab-workflow"
    );

    Ok(())
}
