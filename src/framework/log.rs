/// Abort unconditionally.
/// Logs a trace with println!
#[macro_export]
macro_rules! abort_msg {
    ( $msg:expr ) => {
        {
            println!("{}:{} abort(): {}", file!(), line!(), $msg);
            return Err(());
        }
    };
}

/// Abort with Err(()) if the expression evaluates to true.
/// Logs a trace with println!
#[macro_export]
macro_rules! abort_if {
    ( $e:expr ) => {
        if $e {
            println!("{}:{} abort({})", file!(), line!(), stringify!($e));
            return Err(());
        }
    };
}

/// Abort with Err(()) if the expression evaluates to true.
/// Logs a trace with println! plus additional provided context.
#[macro_export]
macro_rules! abort_if_msg {
    ( $e:expr, $msg:expr ) => {
        if $e {
            println!("{}:{} abort({}): {}", file!(), line!(), stringify!($e), $msg);
            return Err(());
        }
    };
}

/// Abort with Err($msg) if the expression evaluates to true.
/// Logs a trace with println! plus additional provided $msg context.
#[macro_export]
macro_rules! abort_if_msg_str {
    ( $e:expr, $msg:expr ) => {
        if $e {
            println!("{}:{} abort({}): {}", file!(), line!(), stringify!($e), $msg);
            return Err($msg.to_string());
        }
    };
}

/// Unwrap a Result and return Err(()).
/// Log a trace with println!.
#[macro_export]
macro_rules! unwrap_abort {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => {
                println!("{}:{} abort({})", file!(), line!(), stringify!($e));
                return Err(());
            }
        }
    };
}

/// Unwrap a Result and return Err(()).
/// Log a trace with println! with a message as context.
#[macro_export]
macro_rules! unwrap_abort_msg {
    ( $e:expr, $msg:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => {
                println!("{}:{} abort({}): {}", file!(), line!(), stringify!($e), $msg);
                return Err(());
            }
        }
    };
}

/// Unwrap a Result or return the error as a string.
#[macro_export]
macro_rules! unwrap_abort_str {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(e) => {
                let msg =
                    format!("{}:{} abort({}): {}", file!(), line!(), stringify!($e), e.to_string());
                println!("{}", msg);
                return Err(msg);
            }
        }
    };
}
