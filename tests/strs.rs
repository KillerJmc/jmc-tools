use jmc_tools::Strs;

#[test]
fn index_of_test() {
    // 查找成功
    let s = "aaaaHello there!";
    let res = Strs::index_of(s, 4, "there").unwrap();
    assert_eq!(res, 10);

    // 查找不到
    match Strs::index_of(s, 0, "slf") {
        None => println!("查不到！"),
        Some(_) => println!()
    }
}
