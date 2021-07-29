struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let v = vec![1,2,3];
        for ele in &v {
            println!("{}", ele);
        }

        //在 for 循环中的迭代过程不需要 mut 的原因在于，迭代器的 ownership
        let i = v.iter();
        for ele in i {
            println!("{}", ele);
        }


        let mut a = v.iter();
        if let Some(value) = a.next() {
            println!("{}", value);
        }
    }

    #[test]
    fn test_ownership() {
        let v = vec![1,2,3];
        
        // ownership 被转移之后，获得所有权的变量可能做任何其他的事情，比如变成 mut 都是可以的
        let mut v2 = v;
        v2.push(4);
        println!("{:?}", v2);
    }

    #[test]
    fn test_iter_2() {
        let v = vec![1,2,3,4];
        
        // sum is consuming adaptor
        let v3 = v.iter();
        let su :i32 = v3.sum();
        println!("{}",su);

        // map is iterator adaptor
        // collect is consuming adaptor
        let v4 = v.iter();
        let new_v :Vec<_> = v4.map(|x|{
            x + 1
        }).collect();
        println!("{:?}", new_v);
    }

    #[test]
    fn test_counter() {
        let c = Counter::new();
        for i in c {
            println!("{:?}", i);
        }
    }
}