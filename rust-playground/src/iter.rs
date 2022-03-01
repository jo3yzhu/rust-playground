#[derive(PartialEq, Debug)]
struct MyClass {
    i: u32,
    s: String,
}

impl Drop for MyClass {
    fn drop(&mut self) {
        println!("drop {:?}", self);
        assert_eq!(self.i, 0);
        assert_eq!(self.s, String::from("0"));
    }
}

#[cfg(test)]
mod tests {
    use super::MyClass;

    #[test]
    fn iteration_test() {
        let mut v: Vec<MyClass> = vec![
            MyClass { i: 0, s: String::from("0") },
            MyClass { i: 1, s: String::from("1") },
            MyClass { i: 2, s: String::from("2") },
        ];

        // lazy, nothing happened
        let mut _iter = v.iter();

        // mutable borrowing
        for val in v.iter_mut() {
            val.i += 1;
        }

        // mutable borrowing
        v.iter_mut().for_each(|val| (*val).i += 1);

        // iteration using for loop with immutable borrowing
        for val in v.iter() {
            println!("{:?}", val);
            // impossible
            // val.i += 1;
        }

        // iteration using for_each() and closure with immutable borrowing
        v.iter().for_each(|val| println!("{:?}", val));

        // moving semantic
        for mut val in v.into_iter() {
            val.i = 0;
            val.s = String::from("0");
        }

        // v is already moved here
    }

    #[test]
    fn consuming_adaptors_test() {
        // a consuming adaptor is a function built over iteration by calling next()
        let v = vec![1, 2, 3, 4];
        let iter = v.iter();
        let total: u32 = iter.sum();
        // cannot compile because sum() calling has token ownership of iter
        // let x: u32 = iter.sum()
        assert_eq!(total, 10);

        // another one, which multiplies all the elements
        assert_eq!(v.iter().product::<u32>(), 24);
    }

    #[test]
    fn iterator_adaptors_test() {
        // a iterator adaptor is a function convert an iterator into another for different purposes
        let v = vec![1, 2, 3, 4];
        // for each iteration, return x + 1 without change original x
        let converted = v.iter().map(|x| x + 1);
        let vv:Vec<_> = converted.collect();
        assert_eq!(vv, vec![2, 3, 4, 5]);
    }
}