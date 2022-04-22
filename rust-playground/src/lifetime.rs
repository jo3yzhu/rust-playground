#[cfg(test)]
mod tests {

    // there are two input reference with the same lifetime, so the output must be one of them
    fn longest_same_lifetime<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // there are to input reference, lifetime is longer of x than that of b.
    // so the output reference can be either x or y, and it shall be a valid reference when someone
    // call it to borrow one of them, so its lifetime must be short enough against dangling refernce
    fn longest_different_lifetem<'a, 'b>(x: &'a str, y: &'b str) -> &'b str
    where
        'a: 'b,
    {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    #[test]
    fn it_works() {
        println!("{}", 1);
    }
}
