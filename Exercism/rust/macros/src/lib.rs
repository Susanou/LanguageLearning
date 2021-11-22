

#[macro_export]
macro_rules! hashmap {
    ( $($key:expr => $value:expr),* ) => {

        use ::std::collections::HashMap;

        let mut _temp_map = HashMap::new();
        $(
            let _ = _temp_map.insert($key, $value);
        )*

        _temp_map
    };

    ( $($key:expr => $value:expr,)+ ) => {
        $crate::hashmap!($($key => $value),+)
    };

}
