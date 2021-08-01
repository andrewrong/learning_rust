use core::fmt;
use std::error::Error;

// error 因为孤儿规则
// impl<T> fmt::Display for Vec<T> {

// }


// 类型别名，并不是创建了新的类型，这两个可以完全无差别的使用; 主要是为了程序员对有个很长很长的类型名进行简化，但是做额外的类型检测是不行的
type Kilometer = i32;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn mmm() -> std::result::Result<String, std::io::Error> {
    Ok(String::from("hello"))
}

fn return_closure() -> Box<dyn Fn() -> i32> {
    Box::new(|| {
        5
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wrapper() {
        let a = Wrapper(vec![String::from("string"), String::from("hello"),String::from("world")]);
        println!("{}", a);
    }

    #[test]
    fn test_never_type() {
        loop {
            let a = match mmm() {
                Ok(value) => {
                    value
                },
                // continue 这边返回的就是 never type(!)
                Err(_) => continue,         
            };
        }
    }

    #[test]
    fn test_fn() {
        let list_of_numbers = vec![1, 2, 3];
        let list_of_strings: Vec<String> = list_of_numbers
            .iter()
            .map(|i| i.to_string())
        .collect();    
    }
}