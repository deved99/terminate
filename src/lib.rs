#[macro_export]
macro_rules! term {
    ($s:expr, $($v:expr),*) => {
        {
            println!($s, $($v),*);
            println!("");
            panic!("term! macro")
        }
    };
    ($s:expr) => {
        term!("{}", $s)
    }
}
