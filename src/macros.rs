macro_rules! do_http {
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr) => {
        $error_handle!($error_handle!(reqwest::get($url).await, $error).json::<$output_type>().await, $error)
    };
    ($url:ident, $output_type:ty) => {
        let res = reqwest::get($url).await.unwrap();
        res.json::<$output_type>().await.unwrap()
    };
    ($url:ident) => {
        let res = reqwest::get($url).await.unwrap();
        res.json().await.unwrap()
    };

    ($url:ident, $error_handle:ident, $error:expr) => {
        $error_handle!($error_handle!(reqwest::get($url).await, $error).json().await, $error)
    };
    ($url:ident, $error:expr) => {
        use crate::errors::ErrorHandle;
        ErrorHandle!(ErrorHandle!(reqwest::get($url).await, $error).json().await, $error)
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