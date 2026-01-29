pub fn test() {
    assert!(true);
}

#[cfg(test)]
#[test]
fn core_unit_test() {
    test();
}
