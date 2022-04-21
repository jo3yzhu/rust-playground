/// ### different types of closure
/// inheritance hierarchy: `fn` -> `Fn` -> `FnMut` -> `FnOnce`
/// - `fn`， function pointer, no context
/// - `Fn`, trait implemented by closure with immutable borrowing of context
/// - `FnMut`, trait implemented by closure with mutable borrowing of context
/// - `FnOnce`, trait implement by closure with moving semantic of context
/// - `move` keyword can be used before parameter list if you want to force the closure to take ownership of the values it uses in its scope

struct MyStruct {
    text: &'static str,
    number: u32,
}

impl MyStruct {
    fn new(text: &'static str, number: u32) -> MyStruct {
        MyStruct {
            text,
            number,
        }
    }
    // We have to specify that 'self' is an argument.
    fn get_number(&self) -> u32 {
        self.number
    }
    // We can specify different kinds of ownership and mutability of self.
    fn inc_number(&mut self, i: u32) -> u32 {
        self.number += i;
        self.number
    }
    // There are three different types of 'self'
    fn destructor(self) {
        println!("Destructing {}", self.text);
    }
}

fn is_function_pointer<A, R>(_x: fn(A) -> R) {}

fn is_fn<A, R, F: Fn(A) -> R>(_x: &F) {}

fn is_fn_mut<A, R, F: FnMut(A) -> R>(_x: &F) {}

fn is_fn_once<A, R, F: FnOnce(A) -> R>(_x: &F) {}

#[cfg(test)]
mod tests {
    use crate::closure::{is_fn, is_function_pointer, is_fn_mut, is_fn_once, MyStruct};

    #[test]
    fn function_pointer_test() {
        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        // it just acts as a pure function without context
        let closure = |x: &MyStruct| x.get_number() + 3;

        is_function_pointer(closure);
        is_fn(&closure);
        is_fn_mut(&closure);
        is_fn_once(&closure);

        assert_eq!(closure(&obj1), 18);
        assert_eq!(closure(&obj2), 13);
    }

    #[test]
    fn trait_fn_test() {
        let obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        // immutable borrow
        let closure = |x: &MyStruct| x.get_number() + obj2.get_number();

        // with captured context is not a normal function pointer
        // is_fn(closure);
        // is_Fn(&closure);
        is_fn_mut(&closure);
        is_fn_once(&closure);

        assert_eq!(closure(&obj1), 25);
    }

    #[test]
    fn trait_fn_mut_test() {
        let obj1 = MyStruct::new("Hello", 15);
        let mut obj2 = MyStruct::new("More Text", 10);

        // mutable borrow, the closure must be annotated as mutable
        let mut closure = |x: &MyStruct| obj2.inc_number(x.get_number());

        // is_fn(closure);
        // is_Fn(&closure);
        is_fn_mut(&closure);
        is_fn_once(&closure);

        assert_eq!(closure(&obj1), 25);
    }

    #[test]
    fn trait_fn_once_test() {
        let mut obj1 = MyStruct::new("Hello", 15);
        let obj2 = MyStruct::new("More Text", 10);

        let closure = |x: &MyStruct| {
            // moving semantic happened here because equivalent MyStruct::destructor(obj) calling
            obj2.destructor();
            x.get_number()
        };

        is_fn_once(&closure);
        // is_fn(closure);
        // is_Fn(&closure);
        // is_FnMut(&closure);

        assert_eq!(closure(&mut obj1), 15);

        // closure can be only moved
        // let c2 = closure;
    }

    #[test]
    fn copy_clone_traits_test() {
        // copy trait is implemented for &str
        // rust can simply call memcpy to copy it due to it owns no external resource
        let s = "hello";
        let ss = s;

        let sp = s.as_ptr();
        let sp = format!("{:p}", sp);
        let ssp = ss.as_ptr();
        let ssp = format!("{:p}", ssp);

        assert_eq!(sp.to_string(), ssp.to_string());

        let sss = s.clone();
        let sssp = sss.as_ptr();
        let sssp = format!("{:p}", sssp);

        // &str is kind of static, behavior of clone and copy is the same
        assert_eq!(sp.to_string(), sssp.to_string());
    }
}