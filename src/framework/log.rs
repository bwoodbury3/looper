/// Abort unconditionally.
/// Logs a trace with println!
#[macro_export]
macro_rules! abort_msg {
    ( $msg:expr ) => {{
        println!("{}:{} abort(): {}", file!(), line!(), $msg);
        return Err(());
    }};
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

/// Unwrap a Result or return Err(()).
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

/// Unwrap a Result or return Err(()).
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

/// Unwrap an Option or return Err(()).
#[macro_export]
macro_rules! opt_abort {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => {
                println!("{}:{} abort({})", file!(), line!(), stringify!($e));
                return Err(());
            }
        }
    };
}

/// Unwrap an Option or return Err(()).
/// Log a trace with println! with a message as context.
#[macro_export]
macro_rules! opt_abort_msg {
    ( $e:expr, $msg:expr ) => {
        match $e {
            Some(x) => x,
            None => {
                println!("{}:{} abort({}): {}", file!(), line!(), stringify!($e), $msg);
                return Err(());
            }
        }
    };
}

/// Unwrap an Option and return a message as a string.
/// Log a trace with println! with a message as context.
#[macro_export]
macro_rules! opt_abort_str {
    ( $e:expr, $msg:expr ) => {
        match $e {
            Some(x) => x,
            None => {
                let msg = format!("{}:{} abort({}): {}", file!(), line!(), stringify!($e), $msg);
                println!("{}", msg);
                return Err(msg);
            }
        }
    };
}

// Unit test macros

// Super cheap approximation for floating point equality.
#[macro_export]
macro_rules! assert_approx_eq {
    ( $x:expr, $y:expr, $z:expr ) => {
        if ($x - $y).abs() > $z {
            panic!("Numbers were not equal: {} != {}", $x, $y);
        }
    };
}
