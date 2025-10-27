use std::sync::{Arc, Mutex};

#[allow(dead_code)]
trait Animal {
    fn speak(&self);
}

#[allow(dead_code)]
struct Dog;
#[allow(dead_code)]
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

#[allow(dead_code)]
struct Node {
    value: Box<dyn Animal + Send>,
    next: Option<Arc<Mutex<Node>>>,
}

#[cfg(test)]
mod tests {
    use super::{Cat, Dog, Node};
    use std::{
        cell::RefCell,
        rc::Rc,
        sync::{Arc, Mutex, OnceLock},
        thread,
    };

    #[test]
    fn test_box() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(*y, 5);
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(1);
        let b = Rc::clone(&a);
        let c = Rc::clone(&a);

        assert_eq!(Rc::strong_count(&a), 3);
        assert_eq!(Rc::strong_count(&b), 3);
        assert_eq!(Rc::strong_count(&c), 3);
    }

    #[test]
    fn test_rc_2() {
        let a = Rc::new(5);
        let _b = Rc::clone(&a);
        let _b = Rc::clone(&a);

        fn do_something(v: Rc<i32>) {
            println!("{}", v);
        }

        let c = Rc::clone(&a);
        do_something(c); // c 转移所有权，do_something执行完成后，作用域结束

        {
            let _d = Rc::clone(&a);
            assert_eq!(Rc::strong_count(&a), 4);
        } // 子作用域结束，d释放引用，值引用数减一

        assert_eq!(Rc::strong_count(&a), 3); // 注意，这里的引用数量为 3
    }

    #[test]
    fn test_refcell_for_mut_borrow() {
        let x = RefCell::new(10);
        *x.borrow_mut() += 5;

        assert_eq!(*x.borrow(), 15);
    }

    #[test]
    fn test_rc_refcell_for_singlethread() {
        let x = 10;
        let y = Rc::new(RefCell::new(x));

        *y.borrow_mut() += 5;

        assert_eq!(*y.borrow(), 15);
    }

    #[test]
    fn test_arc_for_multithreads_read() {
        let data = Arc::new(5);

        let handles: Vec<_> = (0..3)
            .map(|_| {
                let data = Arc::clone(&data);
                thread::spawn(move || {
                    println!("{}", data);
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_mutex_for_multithreads_mut() {
        let data = Mutex::new(5);

        {
            let mut data = data.lock().unwrap();
            *data += 10;
        }

        assert_eq!(*data.lock().unwrap(), 15);
    }

    #[test]
    fn test_arc_mutex_multithreads() {
        let node_3 = Arc::new(Mutex::new(Node { value: Box::new(Dog), next: None }));
        let node_2 =
            Arc::new(Mutex::new(Node { value: Box::new(Cat), next: Some(Arc::clone(&node_3)) }));
        let node_1 =
            Arc::new(Mutex::new(Node { value: Box::new(Dog), next: Some(Arc::clone(&node_2)) }));

        let nodes = vec![Arc::clone(&node_1), Arc::clone(&node_2), Arc::clone(&node_3)];

        let handles: Vec<_> = nodes
            .into_iter()
            .map(|node| {
                thread::spawn(move || {
                    let node = node.lock().unwrap();
                    node.value.speak();
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        }
    }

    #[test]
    fn test_once_cell() {
        let config = OnceLock::new();

        config.set("hello".to_string()).unwrap();

        assert_eq!(config.get(), Some(&"hello".to_string()));
    }
}
