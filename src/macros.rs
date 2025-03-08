macro_rules! do_http {
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr) => {
        if let Ok(response) = minreq::get($url).send() {
            match response.status_code {
                200 => {
                    $error_handle!(response.json::<$output_type>(), $error)
                }
                404 => {
                    $error_handle!(response.json::<$output_type>(), $error)
                }
                _ => {
                    return Err($error(format!(
                        "Expected 200 Status, got {}",
                        response.status_code
                    )));
                }
            }
        } else {
            // TODO: Make this more descriptive
            return Err($error("HTTPS Error".to_string()));
        }
    };

    // Post Support
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr, $body:ident) => {
        if let Ok(response) = minreq::post($url)
            .with_header("content-type", "application/x-www-form-urlencoded")
            .with_body($body)
            .send()
        {
            match response.status_code {
                200 => {
                    $error_handle!(response.json::<$output_type>(), $error)
                }

                _ => {
                    return Err($error(format!(
                        "Expected 200 Status, got {}",
                        response.status_code
                    )));
                }
            }
        } else {
            // TODO: Make this more descriptive
            return Err($error("HTTPS Error".to_string()));
        }
    };

    // Post support with debugging
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr, $json_data:ident, $debug:literal) => {
        if let Ok(response) = minreq::post($url)
            .with_header("Content-Type", "application/json")
            .with_body($json_data.to_owned())
            .send()
        {
            match response.status_code {
                200 => $error_handle!(response.text().await, $error),

                _ => {
                    return Err($error(format!(
                        "Expected 200 Status, got {}",
                        response.status_code
                    )));
                }
            }
        } else {
            // TODO: Make this more descriptive
            return Err($error("HTTPS Error".to_string()));
        }
    };

    ($url:ident, $output_type:ty) => {
        minreq::get($url).send()?.json::<$output_type>()?
    };

    ($url:ident, $error_handle:ident, $error:expr) => {
        $error_handle!(
            $error_handle!(minreq::get($url).send()?, $error).json()?,
            $error
        )
    };
    ($url:ident, $error:expr) => {
        use crate::errors::ErrorHandle;
        ErrorHandle!(
            ErrorHandle!(minreq::get($url).send()?, $error).json().await,
            $error
        )
    };
    ($url:ident) => {
        minreq::get($url).send()?.as_str()?
    };
}

pub(crate) use do_http;

macro_rules! error {
    ($enum_name:ident { $($variant:ident($err:expr)),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $enum_name {
            $($variant(String),)*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $($enum_name::$variant(err) => write!(f, "{}", err),)*
                }
            }
        }

        impl std::error::Error for $enum_name {}
    };
}

pub(crate) use error;

macro_rules! optional_argument {
    () => { String::new() };

    ($key:ident, $rename:literal) => {
        match $key {
            Some(value) => format!("&{}={}", String::from($rename), value),
            None => String::new()
        }
    };

    ($first:expr $(, $rest:expr)*) => {
        match $first {
            Some(value) => format!("&{}={:?}{}", stringify!($first), value,optional_argument!($($rest),*)),
            None => format!("{}", optional_argument!($($rest),*) )
        }
    };


}

pub(crate) use optional_argument;

macro_rules! gen_args {
    // Base case: When there are no variables left to concatenate, return an empty string.
    () => { String::new() };

    // Recursive case: Concatenate the first variable with the rest of the variables recursively.
    ($first:expr $(, $rest:expr)*) => {
        format!("&{}={}{}", stringify!($first), $first, gen_args!($($rest),*))
    };
}

pub(crate) use gen_args;
