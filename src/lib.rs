use tokio::process::Command;

async fn hello_world() -> String {
    let output = Command::new("echo").arg("hello").arg("world").output();
    let output = output.await.expect("No such file or directory");
    String::from_utf8(output.stdout).expect("Format error")
}

pub async fn test() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(hello_world().await, "hello world\n");
    Ok(())
}
