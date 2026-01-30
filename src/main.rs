use test_bin_starter::*;

fn main() -> std::io::Result<()> {
    assert_eq!(hello_world(), "Hello, world!");
    Ok(())
}
