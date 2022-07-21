#[macro_export]
macro_rules! dbg_to_file {
    // Handle the base case of no arguments.
    () => {
        use std::io::Write;

        let mut file = get_file("text.txt");
        writeln!(file, "[{}:{}]", file!(), line!());
    };

    // Handle a single argument.
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                match $crate::__private_helpers::get_file("text.txt") {
                    Ok(mut file) => {
                        use std::io::Write;
                        match writeln!(file, "[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp) {
                            Ok(()) => tmp,
                            Err(e) => {
                                dbg!("dbg_to_file failed to write to file: {:?}", e);
                                dbg!(tmp)
                            }
                        }
                    }
                    Err(e) => {
                        dbg!("dbg_to_file failed to create/open file: {:?}", e);
                        dbg!(tmp)
                    }
                }
            }
        }
    };

    // Call dbg_to_file! for each argument.
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg_to_file!($val)),+,)
    };
}

#[doc(hidden)]
pub mod __private_helpers {
    use std::fs::File;

    pub fn get_file(path: &str) -> Result<File, std::io::Error> {
        File::options().create(true).append(true).open(path)
    }
}
