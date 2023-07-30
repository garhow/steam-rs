macro_rules! do_http {
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr) => {
        $error_handle!($error_handle!(reqwest::get($url).await, $error).json::<$output_type>().await, $error)
    };
    ($url:ident, $output_type:ty) => {
        reqwest::get($url).await.unwrap().json::<$output_type>().await.unwrap()
    };
    // ($url:ident) => {
    //     let res = reqwest::get($url).await.unwrap();
    //     res.json().await.unwrap()
    // };

    ($url:ident, $error_handle:ident, $error:expr) => {
        $error_handle!($error_handle!(reqwest::get($url).await, $error).json().await, $error)
    };
    ($url:ident, $error:expr) => {
        use crate::errors::ErrorHandle;
        ErrorHandle!(ErrorHandle!(reqwest::get($url).await, $error).json().await, $error)
    };
    ($url:ident) => {
        reqwest::get($url).await.unwrap().text().await.unwrap()
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
    };
}

pub(crate) use error;

macro_rules! optional_argument {
    () => { String::new() };

    ($key:ident, $rename:literal) => {
        match $key {
            Some(value) => format!("&{}={}", stringify!($rename), value),
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