use octocrab::Octocrab;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let octocrab = Octocrab::builder().build()?;

    let repo = octocrab
        .repos("test-bin-rs", "test-bin-rs.github.io")
        .get()
        .await?;

    assert_eq!(format!("{}", repo.id), "1145166759");

    Ok(())
}
