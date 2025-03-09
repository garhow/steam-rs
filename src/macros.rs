macro_rules! do_http {
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr) => {
        if let Ok(response) = reqwest::get($url).await {
            match response.status() {
                reqwest::StatusCode::OK => {$error_handle!(response.json::<$output_type>().await, $error)}
                reqwest::StatusCode::NOT_FOUND => {$error_handle!(response.json::<$output_type>().await, $error)}
                _ => {
                    return Err($error(format!(
                        "Expected 200 Status, got {}",
                        response.status()
                    )));
                }
            }
        } else {
            // TODO: Make this more descriptive
            return Err($error("HTTPS Error".to_string()));
        }
    };

    // Post Support
    ($url:ident, $output_type:ty, $error_handle:ident, $error:expr, $json_data:ident) => {
        if let Ok(response) = reqwest::Client::new()
            .post($url)
            .header("Content-Type", "application/json")
            .body($json_data.to_owned())
            .send()
            .await
        {
            match response.status() {
                reqwest::StatusCode::OK => {
                    $error_handle!(response.json::<$output_type>().await, $error)
                }

                _ => {
                    return Err($error(format!(
                        "Expected 200 Status, got {}",
                        response.status()
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
        if let Ok(response) = reqwest::post($url)
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
        reqwest::get($url).await?.json::<$output_type>()?
    };

    ($url:ident, $error_handle:ident, $error:expr) => {
        $error_handle!(
            $error_handle!(reqwest::get($url).await?, $error).json()?,
            $error
        )
    };
    ($url:ident, $error:expr) => {
        use crate::errors::ErrorHandle;
        ErrorHandle!(
            ErrorHandle!(reqwest::get($url).await?, $error).json().await,
            $error
        )
    };
    ($url:ident) => {
        reqwest::get($url).await?.as_str()?
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

#[macro_export]
macro_rules! gen_args {
    // Base case: When there are no variables left to concatenate, return an empty string.
    () => { String::new() };

    // Recursive case: Concatenate the first variable with the rest of the variables recursively.
    ($first:expr $(, $rest:expr)*) => {
        format!("&{}={}{}", stringify!($first), $first, gen_args!($($rest),*))
    };
}

pub(crate) use gen_args;

// 2025 Marco

// TODO: I am yet to decided whether this is needed
// pub trait SteamAPI {
//     fn endpoint_name() -> &'static str;
// }

macro_rules! EndPoint {
    (
        $fn_name:ident, // Name of the function to generate
        $struct_name:ident, // Name of the Builder struct (i.e. RequestNameReq or RequestNameBuilder)
        $url:expr, // The endpoint URL, without any ?
        $output_type:ty, // The Finial output type
        ( $( $arg_name1:ident : $arg_ty1:ty ),* ),  // Required arguments
        [ $( $arg_name2:ident : Option<$arg_ty2:ty> ),* ],  // Optional arguments
        $body:item // the HTTP handler
    ) => {
        use serde::ser::{Serializer, SerializeStruct};
        use crate::gen_args;

        #[derive(Debug)]
        pub struct $struct_name {
            // #[serde(skip_serializing)]
            _steam: Steam,
            $(
                $arg_name1: $arg_ty1,
            )*
            $(
                $arg_name2: Option<$arg_ty2>,
            )*
        }

        impl $struct_name {
            $(
                pub fn $arg_name2(mut self, value: $arg_ty2) -> Self {
                    self.$arg_name2 = Some(value);
                    self
                }
            )*

            fn to_args(&self) -> String {
                let mut req = String::new();
                let key = self._steam.api_key.clone();
                req.push_str(&gen_args!(key));
                $(
                    let $arg_name1 = self.$arg_name1;
                    req.push_str(&gen_args!($arg_name1));
                )*
                $(
                    let $arg_name2 = &self.$arg_name2;
                    req.push_str(&optional_argument!(&$arg_name2));
                )*
                req
            }

            fn to_url(&self) -> String {
                format!("{}{}", $url, self.to_args())
            }

            $body

            pub async fn send(&self) -> Result<$output_type, Box<dyn std::error::Error>> {
                let url = self.to_url();
                Ok(Self::internal(url).await?)
            }
        }

        impl Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut state = serializer.serialize_struct(stringify!($struct_name), 3)?;
                $(
                    state.serialize_field(stringify!($arg_name1), &self.$arg_name1)?;
                )*
                $(
                    state.serialize_field(stringify!($arg_name2), &self.$arg_name2.as_ref().unwrap())?;
                )*
                state.end()
            }
        }

        // impl SteamAPI for $struct_name {
        //     fn endpoint_name() -> &'static str {
        //         stringify!($fn_name)
        //     }
        // }

        impl Steam {
            pub fn $fn_name(&self, $( $arg_name1 : $arg_ty1 ),*) -> $struct_name {
                let request = $struct_name {
                    _steam: self.clone(),
                    $(
                        $arg_name1: $arg_name1,
                    )*
                    $(
                        $arg_name2: None,
                    )*
                };
                request
            }
        }
    };

    // DO NOT USE THIS PATTERN
    (
        $fn_name:ident,
        $struct_name:ident,
        $output_type:ty,
        $( $arg_name:ident : Option<$arg_ty:ty> ),*
    ) => {
        pub struct $struct_name {
            $(
                $arg_name: Option<$arg_ty>,
            )*
        }

        impl SteamAPI for $struct_name {
            fn endpoint_name() -> &'static str {
                stringify!($fn_name)
            }
        }

        impl $struct_name {
            $(
                pub fn $arg_name2(mut self, value: $arg_ty2) -> Self {
                    self.$arg_name2 = Some(value);
                    self
                }
            )*
        }

        impl Steam {
            complie_error!("Not implemented");
            pub fn $fn_name(&self) -> $struct_name {
                let request = $struct_name {
                    $(
                        $arg_name2: None,
                    )*
                };
                request
            }
        }
        

    };

    // DO NOT USE THIS PATTERN
    (
        $fn_name:ident,
        $struct_name:ident,
        $output_type:ty,
        $( $arg_name:ident : $arg_ty:ty ),*
    ) => {
        #[derive(Debug)]
        pub struct $struct_name {
            _steam: Steam,
            $(
                $arg_name: $arg_ty,
            )*
        }

        impl SteamAPI for $struct_name {
            fn endpoint_name() -> &'static str {
                stringify!($fn_name)
            }
        }

        impl Steam {
            pub fn $fn_name(&self,
                $( $arg_name : $arg_ty ),*
            ) -> $struct_name {
                complie_error!("Not implemented");
                let request = $struct_name {
                    _steam: self.clone(),
                    $(
                        $arg_name: $arg_name,
                    )*
                };
                request
            }
        }
    };
}

pub(crate) use EndPoint;

// 2025 Marco