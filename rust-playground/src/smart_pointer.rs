#[cfg(test)]
mod tests {
    use crate::smart_pointer::tests::List::{UniqueNil, UniqueCons};

    enum List {
        UniqueCons(i32, Box<List>),
        UniqueNil,
    }

    #[test]
    fn box_borrow_test() {
        let _ptr = Box::new(233);

        /*
        {
            let imu_ref1 = &ptr;
            println!("{}", imu_ref1);

            let mu_ref1 = &mut ptr;
            **mu_ref1 += 1;
            let mu_ref2 = &mut ptr;
            **mu_ref2 += 2;

            // imu_ref1: what the fuck?
            println!("{}", imu_ref1);
        }
        */

        /*
        {
            let mu_ref1 = &mut ptr;
            let imu_ref1 = &ptr;
            **mu_ref1 += 1;
            // imu_ref1: what the fuck?
            println!("{}", imu_ref1);
        }
        */
    }

    #[test]
    fn unique_list_test() {
        let mut list = UniqueCons(1, Box::new(UniqueCons(2, Box::new(UniqueCons(3, Box::new(UniqueNil))))));

        loop {
            match list {
                UniqueCons(i, next) => {
                    println!("{}", i);
                    list = *next; // deref trait is implemented by Box
                }
                UniqueNil => {
                    break;
                }
            }
        }
    }

    use std::rc::Rc;
    use crate::smart_pointer::tests::SharedList::{SharedNil, SharedCons};
    use std::cell::RefCell;
    use crate::smart_pointer::tests::CellList::{CellCons, Nil};

    enum SharedList {
        SharedCons(i32, Rc<SharedList>),
        SharedNil,
    }

    #[test]
    fn shared_list_test() {
        let tail = Rc::new(SharedCons(3, Rc::new(SharedCons(4, Rc::new(SharedCons(5, Rc::new(SharedNil)))))));
        let head1 = SharedCons(1, Rc::clone(&tail));
        let head2 = SharedCons(2, Rc::clone(&tail));

        let mut head = &head1;
        loop {
            match head {
                SharedCons(i, next) => {
                    println!("{}", i);
                    head = next;
                }
                SharedNil => {
                    break;
                }
            }
        }

        let mut head = &head2;
        loop {
            match head {
                SharedCons(i, next) => {
                    println!("{}", i);
                    head = next;
                }
                SharedNil => {
                    break;
                }
            }
        }
    }

    #[derive(Debug)]
    enum CellList {
        CellCons(Rc<RefCell<i32>>, Rc<CellList>),
        Nil
    }

    #[test]
    fn cell_list_test() {
        let common_value = Rc::new(RefCell::new(5));
        let tail = Rc::new(CellCons(Rc::clone(&common_value), Rc::new(Nil)));
        let head1 = Rc::new(CellCons(Rc::new(RefCell::new(4)), Rc::clone(&tail)));
        let head2 = Rc::new(CellCons(Rc::new(RefCell::new(3)), Rc::clone(&tail)));
        *common_value.borrow_mut() += 10;
        println!("tail: {:?}", tail);
        println!("head1: {:?}", head1);
        println!("head2: {:?}", head2);

    }
}