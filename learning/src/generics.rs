use std::io;
use rand::Rng;

/*
 * T 不一定实现了比较符号操作，所以需要用 trait 去要求 T 的行为，对于这个函数 T 必须支持比较操作符;
 */ 
fn get_largest_item<T>(list :&[T]) -> T
    where T: std::cmp::PartialOrd + Copy 
 {
    let mut largest = list[0];    

    for ele in list.iter() {
        if *ele > largest { largest = *ele; }
    }
    largest
}


fn get_largest_item_ref<'a, T>(list :&'a [T]) -> &'a T
    where T: std::cmp::PartialOrd 
 {
    let mut largest = &list[0];    

    for ele in list.iter() {
        if ele > largest {
            largest = ele;
        }
    }
    largest
}

fn get_largest_str<'a>(left :&'a str, right :&'a str) -> &'a str {
    if left.len() > right.len() {
        left
    } else {
        right
    }
}

//struct generics
struct A<T> 
where T: std::fmt::Display
{
    a :T,
    b :T
}

// impl 后面的 T 很重要；这样写了之后 Rust 就知道 point 的 T 是范型不是具体数据类型，而不是通过<>去让编译器自我判断
// 我明明已经在 A 定义中对 T 实现了 display 的 trait，但是在 impl 依然还是需要指定；
impl<T> A<T> 
where T: std::fmt::Display {
    fn display(&self) {
        println!("{}:{}", self.a, self.b);
    }
}

// 特化
impl A<String> {
    fn i_am_string(&self) {
        println!("i am string");
    } 
}

// 特化 2. 只有某一些实现了确定的 trait 的类型的才会有一些函数
impl<T: std::fmt::Display + PartialOrd> A<T> {


}
// enum generics；这个真的是强大的无比的功能，+ match 就无比的厉害
enum MyResult<T, E> {
    MySome(T),
    Error(E)
}

fn get_my_result() -> MyResult<String, io::Error> {
    if rand::thread_rng().gen_range(1..100) % 2 == 0 {
        MyResult::MySome(String::from("hello"))
    } else {
        MyResult::Error(io::Error::new(io::ErrorKind::AddrInUse, "hell"))
    }
}

pub trait MyDisplay {
    fn display(&self) -> String;
}

struct Twitter {
    content :String
}

struct Article {
    author : String,
    content : String
}

//通过这种方式可以实现多态调用
fn polym(impls :impl MyDisplay) {
    println!("{}", impls.display());
}

// trait bound
fn polym2<T: MyDisplay>(impls :T) {
    println!("{}", impls.display());
}


//返回 trait 可行，但是不能返回不一样的两个实现，这个需要更加高级的语法
// fn reture_trait(switch : bool) -> impl MyDisplay {
//     if switch {
//         return Twitter{content: String::from("hello")};
//     } else {
//         Article{author:String::from("fenglin"), content: String::from("hello")}
//     }
// }

impl MyDisplay for Twitter {
    fn display(&self) -> String {
        return String::from(&self.content)
    }
}

impl MyDisplay for Article {
    fn display(&self) -> String {
        format!("author:{}, content:{}", self.author, self.content)
    }
}

pub trait Default_impl {
    fn display2(&self) -> String {
        return String::from("hello");
    }
}

// 使用默认的实现
impl Default_impl for Twitter {}

// struct 包含 ref 就必须指定生命周期，不然编译器不知道对应的 struct 的生命周期长度
#[derive(Debug)]
struct RefStruct<'a> {
    part : &'a str,
}

fn print_it(input : impl std::fmt::Debug + 'static) {
    println!("{:?}", input);
}

//相同函数名

trait duck {
    fn fly(&self);
    fn fly2();
}

trait bird {
    fn fly(&self);
    fn fly2();
}

struct AA {

}

impl duck for AA {
    fn fly(&self) {
        println!("duck");
    }

    fn fly2() {
        println!("duck2");
    }
    
}

impl bird for AA {
    fn fly(&self) {
        println!("bird");
    }

    fn fly2() {
        println!("bird2");
    }
    
}

impl AA {
    fn fly(&self)  {
        println!("self");
    }

    fn fly2() {
        println!("self2");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_struct_genercis() {
        let a = A {
            a: 1,
            b: 2
        };

        a.display();

        let b = A {
            a: String::from("name"),
            b: String::from("hello")
        };
        b.i_am_string();
    }

    #[test]
    fn test_enum() {
        for i in 0..10 {
            match get_my_result() {
                MyResult::MySome(value) => {
                    println!("{}", value)
                }
                MyResult::Error(_) => {
                    println!("{}", "none")
                }
            }
        }
    }

    #[test]
    fn test_trait() {
        let t = Twitter {
            content: String::from("twitter")
        };

        polym(t);

        let a = Article {
            author: String::from("author"), 
            content : String::from("content")
        };
        polym(a);

    }

    #[test]
    fn test_ref() {
        let a = String::from("hello,world");
        let b = a.split(",").next().expect("can't find ,");
        let c = RefStruct {part: b};

        println!("{:?}", c);
    }

    #[test]
    fn test_static() {
        let a = String::from("hello");
        print_it(a);

        let b = String::from("hello");
        // print_it(&b);
    }

    #[test]
    fn test_multi_func() {
        let a = AA{};
        a.fly();

        AA::fly(&a);

        // 这个告知我们其实方法本身没有任何的特殊性，只不过有的时候会有一些语法糖而已
        duck::fly(&a);
        bird::fly(&a);
        
        // self2
        AA::fly2();

        // bird2
        <AA as bird>::fly2();

        //duck3
        <AA as duck>::fly2();
    }
}