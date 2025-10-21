#[cfg(test)]
mod tests {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    struct TestDrop {
        dropped: Arc<AtomicBool>,
    }
    impl Drop for TestDrop {
        fn drop(&mut self) {
            self.dropped.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_move_ownership() {
        let s1 = String::from("hello"); // 在堆上分配内存
        let s2 = s1; // 所有权移动（move）

        // println!("{}", s1); // ❌ 编译错误：s1 已经失效
        println!("{}", s2); // ✅
    }

    #[test]
    fn test_borrowing() {
        let s1 = String::from("hello");
        let s2 = &s1;

        println!("{}", s1);
        println!("{}", s2);

        assert_eq!(s1, String::from("hello"));
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{}", s1);
        println!("{}", s2);

        assert_eq!(s1, String::from("hello"));
        assert_eq!(s2, String::from("hello"));
    }

    #[test]
    fn test_borrowing_multiple_immutable() {
        let s1 = String::from("hi");
        let s2 = &s1;
        let s3 = &s1;

        println!("{s2}, {s3}");
        assert_eq!(s2, "hi");
        assert_eq!(s3, "hi");
    }

    #[test]
    fn test_borrowing_mut_with_scope() {
        let mut s1 = String::from("hi");

        // 将可变借用放到代码块中，形成一个子作用域，避免冲突
        // 代码块结束后再创建不可变借用
        {
            let s2 = &mut s1;
            s2.push_str("!");
            println!("{s2}");
        } // 🔸 代码块结束了，s2的作用域在这里结束，可变借用释放。

        let s3 = &s1;
        println!("{s3}");
        assert_eq!(s3, "hi!"); // ✅ 此时可安全使用不可变借用
    }

    #[test]
    fn test_borrowing_mut_then_immutable() {
        let mut s1 = String::from("hi");

        // 先可变借用
        let s2 = &mut s1;
        s2.push_str("!");
        println!("{s2}"); // s2 最后一次出现，可变借用到此为止

        // 然后再不可变借用，这是合法的
        let s3 = &s1;
        println!("{s3}");
        assert_eq!(s3, "hi!");
    }

    #[test]
    fn test_stack_copy() {
        let x = 10;
        let y = x; // 复制一份值

        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }

    #[test]
    fn test_drop() {
        let dropped = Arc::new(AtomicBool::new(false));
        {
            let _x = TestDrop {
                dropped: dropped.clone(),
            };
            assert_eq!(dropped.load(Ordering::SeqCst), false);
        }

        assert_eq!(
            dropped.load(Ordering::SeqCst),
            true,
            "drop() should be called"
        );
    }

    #[test]
    fn test_vec_move() {
        let v1 = vec![1, 2, 3];
        let v2 = v1;

        // println!("{:?}", v1); // ❌ 编译错误：v1 已经失效
        println!("{:?}", v2);
        assert_eq!(v2, vec![1, 2, 3]);
    }

    #[test]
    fn test_shadowing_and_drop() {
        let dropped_1 = Arc::new(AtomicBool::new(false));
        let dropped_2 = Arc::new(AtomicBool::new(false));
        let _x = TestDrop {
            dropped: dropped_1.clone(),
        };
        let _x = TestDrop {
            dropped: dropped_2.clone(),
        };

        assert_eq!(
            dropped_1.load(Ordering::SeqCst),
            false,
            "drop() should be called"
        );
        assert_eq!(
            _x.dropped.load(Ordering::SeqCst),
            false,
            "drop() should be called"
        );
    }

    #[test]
    fn test_shadowing_and_drop2() {
        struct Tracer(&'static str);

        impl Drop for Tracer {
            fn drop(&mut self) {
                println!("Dropping {}", self.0);
            }
        }

        let _t = Tracer("first");
        let _t = Tracer("second");
        println!("done");
    }
}
