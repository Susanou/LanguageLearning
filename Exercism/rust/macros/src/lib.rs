#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($ ($key:expr => $val:expr), *) => {
        {
            use ::std::collections::HashMap;

            let mut hashmap = HashMap::new();
            $(
                hashmap.insert($key, $val);
            )*
            hashmap
        }
    };

    ($($key:expr => $val:expr,)+ ) => {
        $crate::hashmap!($($key => $val),+)
    }

}
