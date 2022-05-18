fn some_function<T, U>(t: &T, u: &U) -> u32
where
    T: Display + Clone,
    U: Debug + Clone,
{
    1
}
