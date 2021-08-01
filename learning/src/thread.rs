use std::sync::{mpsc, Mutex, MutexGuard};

struct Producer {
    sender: mpsc::Sender<i32>
}

struct Consumer {
    receiver: mpsc::Receiver<i32>
}

impl Producer {
    fn new(sender: mpsc::Sender<i32>) -> Producer { 
        Producer{ 
            sender: sender,
        }
    }

    fn produce(&mut self, value :i32) -> Result<(), mpsc::SendError<i32>>{
        self.sender.send(value)
    }
}

impl Consumer {
    fn new(receiver: mpsc::Receiver<i32>) -> Consumer {
        Consumer { 
            receiver: receiver
        }
    }

    fn consume(&mut self) -> i32 {
        match self.receiver.recv() {
            Ok(value) => {
                value
            }
            _ => {
                println!("receive error");
                -1
            }
        }
    }
} 

fn print_guard(i :&mut i32) {
    println!("{}", i);
}

#[cfg(test)]
mod tests {
    use std::ops::DerefMut;
    use std::sync::Arc;
    use std::thread;
    use dashmap::DashMap;
    use std::time::Duration;
    use super::*;

    #[test]
    fn test_create_thread() {
        let t = std::thread::spawn(move || {
            for i in 0..10 {
                println!("{} running", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        match t.join() {
            Ok(()) => {
                println!("over")
            }
            Err(_) => {
                println!("err")
            }
        }
    }

    #[test]
    fn test_pm() {
        let (tx, rx) = mpsc::channel();       
        let mut producer = Producer::new(tx);
        let mut consumer = Consumer::new(rx);

        let t1 = thread::spawn(move || {
            for i in 0..100 {
                producer.produce(i);
                thread::sleep(Duration::from_millis(100));
            }
        });

        let c1 = thread::spawn(move || {
            loop {
                let v = consumer.consume();    
                if v == -1{
                    break;
                }
                println!("receive value:{}", v);
            }
        });

        t1.join().unwrap();
        c1.join().unwrap();
    }

    #[test]
    fn test_mutex() {
        let m = Mutex::new(vec![]);

        {
            let mut num = m.lock().unwrap();
            num.push(1);
        }

        println!("m = {:?}", m);

        let n = Mutex::new(1);
        let mut n1 = n.lock().unwrap();
        print_guard(&mut n1);
    }

    #[test]
    fn test_atomic() {
        let counter = Arc::new(Mutex::new(0));
        let mut group = vec![];

        for i in 0.. 10 {
            let tmp = counter.clone();
            let handler = thread::spawn(move || {
                let mut num = tmp.lock().unwrap();
                *num += 1;
            });
            group.push(handler);
        }

        for i in group {
            i.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    #[test]
    fn test_dashmap() {
        let map = DashMap::new();
        map.insert("hello", "world");

        for ele in map.iter() {
            println!("k:{},v:{}", ele.key(), ele.value()); 
        }
    }
}