#[macro_export]
macro_rules! check_result {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(_) => {}
            Err(e) => log::error!("{}: {}", $msg, e),
        }
    };
}
