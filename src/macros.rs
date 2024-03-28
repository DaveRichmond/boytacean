#[cfg(feature = "debug")]
#[macro_export]
macro_rules! debugln {
    ($($rest:tt)*) => {
        {
            std::print!("[DEBUG] ");
            std::println!($($rest)*);
        }
    }
}

#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! debugln {
    ($($rest:tt)*) => {
        ()
    };
}

#[cfg(not(feature = "embedded"))]
#[macro_export]
macro_rules! infoln {
    ($($rest:tt)*) => {
        {
            #[cfg(not(feature = "embedded"))]
            std::print!("[INFO] ");
            #[cfg(not(feature = "embedded"))]
            std::println!($($rest)*);
        }
    }
}

#[cfg(feature = "pedantic")]
#[macro_export]
macro_rules! warnln {
    ($($rest:tt)*) => {
        {
            panic!($($rest)*);
        }
    }
}

#[cfg(not(feature = "pedantic"))]
#[macro_export]
macro_rules! warnln {
    ($($rest:tt)*) => {
        {
            #[cfg(not(feature = "embedded"))]
            std::print!("[WARNING] ");
            #[cfg(not(feature = "embedded"))]
            std::println!($($rest)*);
        }
    }
}
