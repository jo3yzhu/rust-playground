#[cfg(test)]
mod tests {
    use std::thread;
    use std::sync::{Mutex, Arc};

    #[test]
    fn spawn_move_test() {
        let v = vec![1, 2, 3, 4];
        let handle = thread::spawn(move || {
            println!("{:?}", v);
        });
        handle.join().unwrap();

        // BTW, what the differences between operator ? and unwrap()?

        // 1. operator ? can be very convenient in nested Option<T> to reduce ugly nested code.
        // when it's None, it just returns None at once
        // when it's Some<T>, it returns the wrapped value
        //
        // for example:
        // struct Person {
        //     job: Option<Job>,
        // }
        //
        // struct Job {
        //     phone_number: Option<PhoneNumber>,
        // }
        //
        // struct PhoneNumber {
        //     area_code: Option<u8>,
        //     number: u32,
        // }
        //
        // given a person, return the area code of one person's phone number if he has one, you can code as bellow:
        // return person.phone_number?.area_code;
        // instead of:
        // match person.phone_number {
        //      None => None,
        //      match person.phone_number.area_code {
        //          None => None,
        //          Some(ac) => ac
        //      }
        // }
        //
        // 2. operator ? can be also used to propagate error from callee to caller,
        //
        //  you can write as bellow when deal with a Result<T>:
        //  let mut f = File::open("hello.txt")?;
        //  let mut s = String::new();
        //  f.read_to_string(&mut s)?;
        //  Ok(s)
        //
        // as function read_to_string return empty Ok() or Err(e), operator here acts just as when dealing with Option<T>,
        // the difference is that:
        //    2.1 it interrupts the function running by "rethrowing" the error to caller
        //    2.2 it keeps the value wrapped by Ok() and give it to variable waiting for this
        // in the example above, as read_to_string returns a empty Ok(), so the wrapped value is ignored.
        //
        // 3. conversely, the unwrap() function is used when want to panic if a Result<T> proven as a Err(r)
    }

    #[test]
    fn mutex_test() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let cloned = counter.clone();
            let handle = thread::spawn(move || {
                let mut num = cloned.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}