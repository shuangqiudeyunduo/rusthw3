use std::ops::Deref;
use std::cell::RefCell;

struct MyRc<T> {
    data: T,
    scount: RefCell<usize>,
}

impl<T> MyRc<T> {
    fn new (data:T) -> MyRc<T> {
        return MyRc{
            data: data,
            scount: RefCell::new(1),
        };
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        return &self.data;
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        *self.scount.borrow_mut() -= 1;
        println!("Dropped!");
    }
}

impl<T> MyRc<T> {
    fn strong_count(&self) -> usize {
        return *self.scount.borrow();
    }
}

impl<T> MyRc<T> 
where
    T:Clone,
{
    fn clone(&self) ->MyRc<T>{
        *self.scount.borrow_mut() += 1;
        return MyRc {
            data: self.data.clone(),
            scount: self.scount.clone(),
        };
    }
}

fn main() {
    let five = MyRc::new(5);
    let five1 = five.clone();

    println!("{}",*five1);
    println!("{}",MyRc::strong_count(&five1));
}