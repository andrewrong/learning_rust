use rand::Rng;

fn get_one(value :i32) -> Option<i32> {
    return Some(value)
}

#[derive(Debug)]
struct M1Msg(String); 

#[derive(Debug)]
struct M2Msg(String); 

#[derive(Debug)]
struct M3Msg(String); 
enum msg {
   M1(M1Msg),
   M2(M2Msg),
   M3(M3Msg), 
}

fn get_random_msg() -> msg {
    let idx = rand::thread_rng().gen_range(1..3);
    if idx == 1 {
        msg::M1(M1Msg(String::from("m1")))
    } else if idx == 2 {
        msg::M2(M2Msg(String::from("m2")))
    } else {
        msg::M3(M3Msg(String::from("m3")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option() {
        let a = get_one(5);
        if let Some(value) = a {
            println!("{:?}", value);
        }
    }

    #[test]
    fn test_match_1() {
        // 通过 match 不但可以匹配对应的模式，还能提取对应的 value
        let a = get_one(5);
        match a {
            Some(value) => {
                println!("{:?}", value);
            }
            None => {
                println!("None");
            }
        }
    }

    #[test]
    fn test_match_2() {
        let m1 = get_random_msg();
        match m1 {
            msg::M1(m) => {
                println!("{:?}", m);
            }
            _ => {
                println!("default");
            }
        }
    }

}