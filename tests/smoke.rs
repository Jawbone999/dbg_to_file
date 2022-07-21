use dbg_to_file::dbg_to_file;

#[test]
fn smoke_test() {
    let x = dbg_to_file!("Hello!");
    dbg_to_file!(x);

    fn foo() -> u8 {
        7
    }

    let y = dbg_to_file!(foo());
    dbg_to_file!(foo(), y);
}
