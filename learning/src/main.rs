mod ownership;
mod option;
mod compound;
mod error;
mod generics;
mod function;
mod iter;
mod smart_pointer;
mod thread;
mod oop;
mod unsafe_;
mod advance;
mod micro;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;
    #[test]
    fn it_works() {
        let ch  = b'A'; 
        println!("u8 single byte {:?}", ch);
    }

    #[test]
    fn char_type() {
        let cat = '😻';
        let cat_u8 = cat as u8;        
        println!("u8:{:?}", cat_u8);
        println!("char sizeof : {:?}", size_of::<char>());
    }

    #[test]
    fn tuple_type() {
        let t1 = (1,2,3);
        let t2 = (1, "string", 0.6);
        let t3 = (1, vec!{1,23,4}, "string");

        let (x, y, z) = t3;

        println!("y:{:?}", y);
        //解构的过程中，t3 中的第二个元素就将 owner 转到给了 y，所以不能直接访问;
        // println!("t3'2:{:?}", t3.1);
    }

    #[test]
    fn array_type() {
        let a1 = [1,3,4];
        let a2: [&str; 4] = ["string", "string", "1", "2"];
        println!("a2:{:?}", a2);
        println!("a2.2:{:?}", a2[2]);
    }
}
