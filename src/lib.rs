use octocrab::Octocrab;
pub use octocrab::Result;
pub async fn test() -> Result<()> {
    let octocrab = Octocrab::builder().build()?;

    let id = "183abcff1346814d9957e7ff73c51e3a";
    let gist = octocrab.gists().get(id).await?;
    let file = gist.files.get("gistfile1.txt").unwrap();
    let content = file.content.clone().ok_or("");

    assert_eq!(
        format!("{}", content.unwrap()),
        "test-bin-tokio-octocrab-gist"
    );

    Ok(())
}
