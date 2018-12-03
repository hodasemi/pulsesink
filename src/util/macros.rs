#[macro_export]
macro_rules! return_error {
    ($v:expr) => {
        if let Err(err) = $v {
            return Err(err);
        }
    };
}

#[macro_export]
macro_rules! print_error {
    ($v:expr) => {
        if let Err(err) = $v {
            println!("{}", err);
        }
    };
}

#[macro_export]
macro_rules! print_error_return {
    ($v:expr) => {
        if let Err(err) = $v {
            println!("{}", err);
            return;
        }
    };
}
