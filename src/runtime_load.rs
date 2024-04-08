
#[macro_export]
macro_rules! library_load {
    (
        $library_variable_name:ident,
        $(#[link(name = $_:expr)])?
        extern "C" {
            $(pub fn $name:ident($($param:ident: $ptype:ty$(,)?)*)$( -> $rtype:ty)?;)*
        }
    ) => {
        $(
            pub static $name: Lazy<unsafe fn($($param: $ptype,)*)$( -> $rtype)*> =
                Lazy::new(|| unsafe {
                    *$library_variable_name
                        .as_ref()
                        .expect("failed to load library")
                        .get(stringify!($name).as_bytes())
                        .expect("failed to load function")
                });
        )*
    };
    (
        $(#[link(name = $_:expr)])?
        extern "C" $block:tt
    ) => {
        library_load! {
            LIBRARY,
            extern "C" $block
        }
    };
    (
        $feature:expr,
        $(#[link(name = $name:expr)])?
        extern "C" $block:tt
    ) => {
        #[cfg(feature = $feature)]
        library_load! {
            LIBRARY,
            extern "C" $block
        }
        
        #[cfg(not(feature = $feature))]
        $(#[link(name = $name)])?
        extern "C" $block
    };
    (
        $library_variable_name:ident,
        $feature:expr,
        $(#[link(name = $name:expr)])?
        extern "C" $block:tt
    ) => {
        #[cfg(feature = $feature)]
        library_load! {
            $library_variable_name,
            extern "C" $block
        }
        
        #[cfg(not(feature = $feature))]
        $(#[link(name = $name)])?
        extern "C" $block
    };
}

#[macro_export]
macro_rules! library_static {
    ($variable_name:ident, $library_name:expr) => {
        pub static $variable_name: Lazy<Result<Library, libloading::Error>> =
            Lazy::new(|| Library::new($library_name));        
    };
    ($variable_name:ident, $library_name:expr, |$error:pat| $on_error:expr) => {
        pub static $variable_name: Lazy<Result<Library, libloading::Error>> =
            Lazy::new(|| Library::new($library_name).or_else(|$error| $on_error));        
    };
}