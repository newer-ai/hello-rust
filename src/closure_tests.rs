#[cfg(test)]
mod tests {
    /// 闭包是匿名函数，可以捕获外部环境的变量
    ///
    /// 第一种情况：按值拷贝的捕获
    ///
    /// 对应 `Fn` trait
    #[test]
    fn test_closure_capture_environment() {
        let x = 10;
        let add_x = |y| y + x;

        assert_eq!(add_x(20), 30);
        assert_eq!(add_x(10), 20);
        assert_eq!(x, 10);
    }

    /// 第二种情况：不可变借用的捕获
    ///
    /// 对应 `Fn` trait
    #[test]
    fn test_closure_capture_by_reference() {
        let prefix = "你好，".to_string();
        let hello = |name: &str| format!("{}{}", &prefix, name);

        assert_eq!(hello("张三"), "你好，张三");
        assert_eq!(&prefix, "你好，");
    }

    /// 第三种情况：借用引用捕获
    ///
    /// 对应 `FnMut` trait
    #[test]
    fn test_closure_capture_by_mut_reference() {
        let mut count = 0;
        let mut inc = || {
            count += 1;
            count
        };

        assert_eq!(inc(), 1);
        assert_eq!(inc(), 2);
        assert_eq!(&count, &2);
    }

    /// 第四种情况：按值移动捕获
    ///
    /// 对应 `FnOnce` trait
    ///
    /// 特征：
    ///
    /// - 按值移动（move）
    /// - 只能调用一次（调用即move）
    /// - 移动所有权闭包
    /// - 要使用 `move` 关键字
    #[test]
    fn test_closure_capture_by_move() {
        let s = String::from("hello");
        let consume = move || s;

        let result = consume();
        assert_eq!(result, "hello");

        // ❌ 不能再使用 s 了（已 move）
        // println!("{}", s);
        // ❌ 也不能再次调用 consume（已 move）
        // let result = consume();
    }

    /// 函数也是闭包
    ///
    /// 普通函数不捕获任何环境，所以可以被当作 `Fn` 、`FnMut`或`FnOnce` 使用
    #[test]
    fn test_function_is_closure() {
        fn call_with_3<F>(f: F) -> i32
        where
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        let x = 10;
        let add_x = |y| y + x;
        assert_eq!(call_with_3(add_x), 13);
    }
}
