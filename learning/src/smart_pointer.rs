use std::{cell::RefCell, ops::Deref};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

#[derive(Debug)]
struct MyBox<T: std::fmt::Display> (T, String);

impl<T> MyBox<T> 
where T: std::fmt::Display
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x, String::from("hello"))
    }
}

impl<T: std::fmt::Display> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop:{:?}", self.1)
    }
}

trait message {
    fn send(&self, msg: String);
}

struct MessageMock {
    msg :RefCell<Vec<String>>
}

impl MessageMock {
    fn new() -> MessageMock {
        MessageMock {
            msg: RefCell::new(vec![])
        }
    }

    fn getMsgLen(&self) -> usize {
        self.msg.borrow().len()
    }
}

impl message for MessageMock {
    fn send(&self, msg: String) {
        let mut one = self.msg.borrow_mut();
        let mut two = self.msg.borrow_mut();
        one.push(msg);
        two.push(String::from("hello"));
        
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_box() {
        let a = Box::new(1);
        println!("{}", a);
    }

    #[test]
    fn test_box_2() {
        let list = List::Cons(3, 
                                    Box::new(List::Cons(2, 
                                        Box::new(List::Cons(1, 
                                            Box::new(List::Nil)
                                        ))
                                    ))
                                );
        println!("{:?}", list);
    }

    #[test]
    fn test_mybox() {
        let a = MyBox::new(5);
        assert_eq!(a.0, 5); 

        // 底层运行为 *(a.deref())
        assert_eq!(*a, 5); 

        let mut b = Box::new(vec![1]);
        b.push(5);
    }

    #[test]
    fn test_mannual_drop() {
        let a = MyBox::new(5);
        //手动的调用 std::mem::drop 来 destructor 某个 value
        drop(a);

        println!("not over");
        //会报错，因为已经被 drop 掉 
        // println!("{:?}",a);
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(vec![1,2,3]);
        // rc shard mem, so it don't mutable
        // a.push(5);
        {
            let b = Rc::clone(&a);
            println!("{}", Rc::strong_count(&b));
        }
        println!("{}", Rc::strong_count(&a));
    }

    #[test]
    fn test_msg() {
        let m = MessageMock::new();
        m.send(String::from("1"));
    }
}