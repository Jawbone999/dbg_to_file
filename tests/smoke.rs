use dbg_to_file::dbg_to_file;

#[test]
fn smoke_test() {
    dbg_to_file!("test.txt");

    let x = dbg_to_file!("test.txt", "Hello!");
    dbg_to_file!("test.txt", x);

    fn foo() -> u8 {
        7
    }

    let y = dbg_to_file!("test.txt", foo());
    dbg_to_file!("test.txt", foo(), y);
}
