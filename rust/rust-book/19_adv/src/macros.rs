#[macro_export]
macro_rules! vecr {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                    temp_vec.push($x);
                    )*
            temp_vec
        }
    };
}

pub fn example() {
    let v = vecr![1, 2, 3];
    println!("v: {:?}", v);
}