#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_vector() {
        let mut vi32 :Vec<i32> = Vec::new();
        //insert
        vi32.push(1);
        vi32.push(2);
        vi32.push(3);

        println!("{:?}", vi32);

        //pop
        let pop_value = vi32.pop();
        if let Some(value) = pop_value {
            println!("{:?}", value);
        }

        //get / []
        let index1 = &vi32[0];
        println!("{:?}", index1);

        let index2 = vi32.get(0);
        if let  Some(value) = index2 {
            println!("{:?}", value);
        }

        let idx3 = vi32.get(4);
        if let  Some(value) = idx3 {
            println!("{:?}", value);
        } else {
            println!("none");
        }
    }

    #[test]
    fn test_vector_borrow() {
        let mut v1 = vec![1,2,3];
        //  a1 会占用不可变的借用，v1.push 就不能用，因为会修改到 a1 的借用
        // let a1 = &v1[0];
        v1.push(5);

        // println!("{:?}", a1);
    }

    #[test]
    fn test_vector_for() {
        let mut v1 = vec![1,2,3];
        for ele in &v1 {
            println!("{:?}", ele);
        }
        
        // 迭代器; 会不会影响 ownership
        for ele in &mut v1 {
            *ele += 5;
        }
        println!("{:?}", v1);
    }

    #[test]
    fn test_vector_ownership() {
        let mut v1 = vec![String::from("hello"), String::from("world")];

        /**
         * 迭代器有 3 种，三种分别
         * 1. 返回 immutable reference
         * 2. 返回 mutable reference
         * 3. 返回 对应的 ownership，已经是 move 语意了
         */
        for ele in &v1 {
            println!("{:?}", ele);
        }

        for ele in & mut v1 {
            println!("{:?}", ele);
        }

        for ele in v1.into_iter() {
            println!("{:?}", ele);
        }

        // println!("{:?}", v1);
    }

    #[test]
    fn test_iter() {
        //iter 其实本质是一个状态机器，它并没有所有的计算结果保存起来，只有你在调用 next 的时候才驱动它
        let mut iter = (1..10).filter(|x| x % 2 == 0);
        while let Some(x) = iter.next() { 
            println!("x:{:?}", x);
        }
    }

    #[test]
    fn test_string() {
        let a = "hello";
        let strA = a.to_string();

        println!("{:?}", strA);
    }

    #[test]
    fn test_add_string() {
        let a = "hello".to_string();
        let b = "world".to_string();

        let c = a + &b;
        // println!("{:?}", a); // error, a have moved
        println!("{:?}", c);

        println!("{:?}", format!("{}-{}", c, b));
    }

    #[test]
    fn test_hashmap() {
        let mut h = HashMap::new();
        h.insert(String::from("hello"), 1);
        h.insert(String::from("world"), 1);

        h.entry(String::from("wz")).or_insert(1);

        println!("{:?}", h);

        let teams = vec![String::from("hello"), String::from("world")];
        let score = vec![10, 20];
        
        // iter 获得是 reference，所以 hashMap 中存储的是 reference
        let scores :HashMap<_,_> = teams.iter().zip(score.iter()).collect();
        println!("{:?}",scores);

        let t = scores.get(&String::from("hello"));
        if let Some(value) = t {
            println!("{:?}", value);
        }

        for (k,v) in &scores {
            println!("{:?}:{:?}",k, v);
        }

        println!("{:?}", scores);
    }
}