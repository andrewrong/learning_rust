
#[cfg(test)]

mod tests {
    use std::fs::File;

    #[test]
    fn test_panic_error() {
        // let f = File::open("/tmp/hello").unwrap();
        let d = File::open("/tmp/hello").expect("open hello is error");
        println!("{:?}",d);
    }

    #[test]
    fn test_recover_error() {
        let f = File::open("/tmp/hello");

        match f {
            Ok(_) => {
                println!("success");
            }
            Err(err) => {
                println!("err:{:?}", err);
            }
        }
    }
}