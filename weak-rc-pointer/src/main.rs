use std::rc::Rc;

fn main() {
    let r1 = Rc::new(0);

    let r4 = {
        let r2 = Rc::clone(&r1);

        Rc::downgrade(&r2)
    };

    let _r5 = Rc::clone(&r1);

    let _r6 = r4.upgrade();

    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
    // we have a valuge because the original value 0 is still refernced by others
    println!("{:?}", _r6);
}

