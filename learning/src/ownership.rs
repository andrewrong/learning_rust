#[derive(Debug)]
struct A {
    x :i32,
    y :i32, 
}

impl A {
    fn new(x :i32, y :i32) -> Self {
        A { x: x, y: y }
    }
}

impl Drop for A {
    fn drop(&mut self) {
        println!("A is dropping");
    }
}

#[derive(Debug, Copy, Clone)]
struct B {
    x :i32,
    y :i32,
}

impl B {
    fn new(x :i32, y :i32) -> Self {
        B { x: x, y: y }
    }
}

fn first_world(a :&String) -> &str {
    &a[0..4]
}

// impl Drop for B {
//     fn drop(&mut self) {
//         println!("B is dropping");
//     }
// }

fn borrow_fn(a :&A) {
    println!("borrow_fn");
    println!("{:?}", a);
}

fn borrow_fn_2(a :&A) {
    println!("borrow_fn_2");
    println!("{:?}", a);
}


// rust 枚举的比较神奇地方，将匹配和数值类型存在一起
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum IpAddr {
    V4(IpV4),
    V6(IpV6)
}
#[derive(Debug)]
struct IpV4 (
    u8,u8,u8,u8   
);

#[derive(Debug)]
struct IpV6 (String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_type() {
        // 字符串字面值
        let st = "hello,world";
        println!("{:?}", st);
    }

    #[test]
    fn string_type() {
        let st = String::from("hello,world");
        println!("{:?}", st);
    }

    #[test]
    fn clone_type() {
        let a = A::new(1,2);
        let b = a;

        println!("{:?}", b);
        // println!("{:?}", a);

        // 继承了 copy/clone 语意之后赋值的过程走的是 copy 语意
        let c = B::new(1,2);
        let d = c;
        println!("{:?}", d);
    }

    #[test]
    fn test_drop() {
        let a = A::new(1,2);
        println!("{:?}", a);

        {
            let b = B::new(1,2);
        }
    }

    #[test]
    fn test_borrow() 
    {
        let a = A::new(1,2);
        // 不可变的引用 1
        borrow_fn(&a);
        // 不可变的引用 2
        borrow_fn_2(&a);
        println!("{:?}", a);
    }

    #[test]
    fn test_mut_borrow() 
    {
        let mut a = String::from("hello");
        let b = &mut a;
        // let c = &mut a;
        // println!("{:?}, {:?}",b, c)
    }

    #[test]
    fn test_mut_borrow_imutable() 
    {
        let mut a = String::from("hello");
        let c = &a;
        // let b = &mut a;
        // let c = &mut a;
        // println!("{:?}, {:?}",b, c)
    }

    #[test]
    fn test_slice() {
        let mut a = String::from("hello, world");
        let b = first_world(&a);
        // a.clear();
        println!("{:?}", b);
    }

    #[test]
    fn test_struct() {
        // 用一个实例初始化另外一个实例并且修改其中一部分的值
        let a = A::new(1,2);
        let b = A {
            x: 2,
            ..a
        };
        println!("{:?}",b);
    }

    #[test]
    fn test_enum() {
        let v4 = IpAddrKind::V4(String::from("127.0.0.1"));
        println!("{:?}",v4);
    }

    #[test]
    fn test_enum_1() {
        let v4 = IpAddr::V4(IpV4(127,0,0,1));
        let v6 = IpAddr::V6(IpV6(String::from(":001")));

        match v6 {
            IpAddr::V4(addr) => {
                println!("{:?}",addr);
            }
            IpAddr::V6(addr) => {
                println!("{:?}",addr);
            }
        }
    }
}