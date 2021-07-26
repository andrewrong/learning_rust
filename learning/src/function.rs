
#[cfg(test)]
mod tests {

    #[test] 
    fn test_closure() {
        let te = |x| x;
        te(String::from("5"));
        // lambda 参数只会被推导过一次，不能类似与泛型一样可以对每一种类型都生成一个函数
        te(5);
    }

}