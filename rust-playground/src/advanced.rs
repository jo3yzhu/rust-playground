#[cfg(test)]
mod test {
    use std::fmt::Formatter;

    /*
        there's a constraint when you want to implement a trait:
            1. you can implement trait for type only when more than one of them are local in current crate
            2. I think this rule prevent user to override behavior or to add function for other's library arbitrarily
        so struct PrintableVec here is a simple wrapper aiming to import Vec<T> in standard library to my crate
     */
    struct PrintableVec {
        v: Vec<String>,
    }

    /*
        implement Deref and DerefMut traits for PrintableVec<T>, which is derived from Vec<T>.
        it simulate inherit mechanism in rust by make PrintableVec<T> a reference agency both mutable and immutable
    */

    impl std::ops::Deref for PrintableVec {
        type Target = Vec<String>;

        fn deref(&self) -> &Self::Target {
            &self.v
        }
    }

    impl std::ops::DerefMut for PrintableVec {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.v
        }
    }

    impl std::fmt::Display for PrintableVec {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[{}]", self.v.join(", "))
        }
    }

    #[test]
    fn printable_vector_test() {
        let mut pv = PrintableVec {
            v: Vec::new(),
        };
        pv.push(String::from("1"));
        pv.push(String::from("2"));
        pv.push(String::from("3"));
        pv.push(String::from("4"));
        assert_eq!(pv.len(), 4);
        println!("{}", pv);
    }

    fn just_do_it_once(f: fn(i32) -> i32, i: i32) -> i32 {
        f(i)
    }

    fn just_do_it_twice(f: fn(i32) -> i32, i: i32) -> i32 {
        let j = f(i);
        f(j)
    }

    fn multiple_2x(i: i32) -> i32 {
        2 * i
    }

    #[test]
    fn function_pointer_test() {
        let i = 233;
        let x2 = just_do_it_once(multiple_2x, i);
        let x4 = just_do_it_twice(multiple_2x, i);
        assert_eq!(x2, 233 * 2);
        assert_eq!(x4, 233 * 4);
    }

    #[macro_export]
    macro_rules! my_vec {
        ($($x:expr), *) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    #[test]
    fn my_macro_test() {
        let mine = my_vec![1,2,3,4,5];
        let theirs = vec![1, 2, 3, 4, 5];
        assert_eq!(mine, theirs);
    }
}