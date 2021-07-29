fn print123(v :Vec<i32>) {
    println!("{:?}", v);
}
struct cache<T> 
where T: Fn(u32) -> u32
{
    calc: T,
    value: Option<u32>
} 

impl<T> cache<T> 
    where T: Fn(u32) -> u32 {
    fn new(calc :T) -> Self {
        cache { 
            calc: calc,
            value: None
        }
    }

    fn value(&mut self, arg :u32) -> u32 {
        match self.value { 
            Some(v) => v,
            None => {
                let v =  (self.calc)(arg);
                self.value = Some(v);
                return v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] 
    fn test_closure() {
        let te = |x| x;
        te(String::from("5"));
        // lambda 参数只会被推导过一次，不能类似与泛型一样可以对每一种类型都生成一个函数
        // te(5);
    }

    #[test]
    fn test_closure_2() {
        let mut v = vec![1];
        let mut c = || {
            if v.len() > 0 {
                println!("in:{:?}", v);
            }
            print123(v);
        };
        c();
        // println!("out:{:?}",v);
    }

}