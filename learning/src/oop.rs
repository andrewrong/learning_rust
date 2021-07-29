trait format {
    fn print_it(&self);
}

struct A {
    name: String,
}

struct B {
    year: i32,
}

impl format for A {
    fn print_it(&self) {
        println!("A:{:?}", self.name);
    }
}

impl format for B {
    fn print_it(&self) {
        println!("B:{:?}", self.year);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dyn_trait() {
        //通过 dyn format 的形式来表示所有现实了 format 的类，与之前 impl format 不一样;
        let mut dyn1 :Vec<Box<dyn format>> = vec![]; 
        
        let a = A{name: String::from("hello")};
        let b = B {year: 10};

        dyn1.push(Box::new(a));
        dyn1.push(Box::new(b));

        for ele in dyn1 {
            ele.print_it();
        }
    }
}