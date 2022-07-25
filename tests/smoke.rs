use dbg_to_file::dbg_to_file;

#[test]
fn smoke_test() {
    dbg_to_file!("smoke.txt");

    let x = dbg_to_file!("smoke.txt", "Hello!");
    dbg_to_file!("smoke.txt", x);

    fn foo() -> u8 {
        7
    }

    let y = dbg_to_file!("smoke.txt", foo());
    dbg_to_file!("smoke.txt", foo(), y);
}

#[test]
fn path() {
    let path = String::from("path.txt");
    dbg_to_file!(&path, "Hello!");
    dbg_to_file!(path, "World!");
}
