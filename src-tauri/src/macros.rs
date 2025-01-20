
#[macro_use]
macro_rules! unwrap_or_err {
    ( $e:expr, $err:tt ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return Err($err),
        }
    };
}

#[macro_use]
macro_rules! unwrap_or_log_and_panic {
    ($e:expr) =>{
        match $e{
            Ok(x) => x,
            Err(err) => {
                log::error!("{}",err);
                panic!();
            }
        }
    }
}