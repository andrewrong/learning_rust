/**
* 5 种必须在 unsafe 种使用的语法
* 1. 解引用裸指针 (*pointer)；理论上 rust 本身安全的操作是没有指针的，只有借用，并且可以解引用
* 2. 调用本身不安全的函数
* 3. 访问或者修改可变静态变量
* 4. 实现不安全的 trait
* 5. 访问 union 的字段
*/

struct A {

}

impl Drop for A {
    fn drop(&mut self) {
        println!("drop a");
    }
}

unsafe fn dangerous() {

}

fn split_at_mut_self(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    //变成指针
    let ptr = slice.as_mut_ptr();

    assert!(len > mid);
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid), std::slice::from_raw_parts_mut(ptr.add(mid), len - mid))
    }
}

//定义 c 的接口，可以调用
extern "C" {
    fn abs(input :i32) -> i32;
}

// 输出给别的语言使用
pub extern "C" fn call_from_c() {

}

static mut COUNTER :u32 = 0;

//不安全的 trait
unsafe trait ttt {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    /**
     * 1. 定义 raw pointer 是完全没问题的;即使 raw pointer 指向的无效的内存地址
     * 2. 但是你绝对不能解指针，也就是 (*pointer)，这个必须放到 unsafe 中
     */
    fn test_raw_pointer() {
        let a = 5;
        
        // rust raw pointer, 
        let r1 = &a as *const i32;
        println!("{:?}", r1);
        unsafe {
            println!("{:?}", *r1);
        }

        // raw 可以忽略借用规则，可以同时持有可变和不可变得 raw 指针
        let mut c = 5;
        let r1 = &c as *const i32;
        let r2 = &mut c as *mut i32;

        // 指向无效内存
        let address = 0x012345usize;
        let r1 = address as *const i32;    

        // 指向 nul
        let r2 :*const i32; 

        // 不会实现任何自动清理功能
        let a = A{}; 

        {
            //除了 scope 不会自动清理，谁家指针还能自动清理 heap
            let r = &a as *const A; 
        }
        println!("pointer after");
        
    }

    #[test]
    fn test_dangerous()  {
        // 调用不安全的
        unsafe {
            dangerous();
        }
    }

    #[test]
    fn test_split() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        // 这个安全原因是因不能对 slice 进行 push 的操作，只能在原先地址上进行修改
        let (s1, s2) = split_at_mut_self(&mut v, 3);
        println!("{:?}", s1);
        println!("{:?}", s2);
    }

    #[test]
    fn test_abs() {
        unsafe {
            println!("{}", abs(-3));
        }
    }

    #[test]
    fn test_static() {
        unsafe {
            COUNTER += 2
        }
        // let v :Vec<>
    }
}