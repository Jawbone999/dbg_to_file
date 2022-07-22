#[macro_export]
macro_rules! dbg_to_file {
    // Handle the base case of no arguments to print.
    ($path:expr) => {
        match $crate::__private_helpers::get_file($path) {
            Ok(mut file) => {
                use std::io::Write;
                match writeln!(file, "[{}:{}]", file!(), line!()) {
                    Ok(()) => {},
                    Err(e) => {
                        eprintln!("dbg_to_file failed to write to file: {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("dbg_to_file failed to create/open file: {:?}", e);
            }
        }

    };

    // Handle a single argument to print.
    ($path:expr, $val:expr $(,)?) => {
        match $val {
            tmp => {
                match $crate::__private_helpers::get_file($path) {
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
    ($path:expr, $($val:expr),+ $(,)?) => {
        ($($crate::dbg_to_file!($path, $val)),+,)
    };
}

#[doc(hidden)]
pub mod __private_helpers {
    use std::fs::File;

    pub fn get_file(path: &str) -> Result<File, std::io::Error> {
        File::options().create(true).append(true).open(path)
    }
}
