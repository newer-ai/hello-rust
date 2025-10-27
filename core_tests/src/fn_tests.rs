#[cfg(test)]
mod tests {

    #[test]
    fn test_fn_copy() {
        fn compare<T>(a: T, b: T) -> T
        where
            T: Copy + PartialOrd,
        {
            if a > b { a } else { b }
        }

        assert_eq!(compare(1, 2), 2);
        assert_eq!(compare(2.0, 1.2), 2.0);
    }

    #[test]
    fn test_fn_borrow() {
        fn compare<'a, T>(a: &'a T, b: &'a T) -> &'a T
        // 'a 称之为生命周期标注
        where
            T: PartialOrd,
        {
            if a > b { a } else { b }
        }

        let a = "abc".to_string();
        let b = "ABC".to_string();
        assert_eq!(compare(&a, &b), "abc");
    }

    #[test]
    fn test_fn_lifetime() {
        fn first_word<'a>(s: &'a str) -> &'a str {
            s.split_whitespace().next().unwrap_or("")
        }

        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_fn_default_lifetime() {
        fn find_largest<T>(list: &[T]) -> &T
        // 编译器会自动
        where
            T: PartialOrd,
        {
            let mut largest = &list[0];
            for item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        let items =
            vec!["abc".to_string(), "acd".to_string(), "bcd".to_string(), "dba".to_string()];

        assert_eq!(find_largest(&items), "dba");
    }

    /// 函数是一等公民：函数本身是值（函数指针）。
    #[test]
    fn test_fn_as_value() {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        let f = add;

        assert_eq!(f(2, 3), 5);
    }

    #[test]
    fn test_fn_as_argument() {
        fn apply(f: fn(i32) -> i32, v: i32) -> i32 {
            f(v)
        }

        fn double(x: i32) -> i32 {
            x * 2
        }

        assert_eq!(apply(double, 4), 8);
    }

    /// 函数指针与闭包之间的兼容性：
    ///
    /// 前提：闭包不捕获外部变量。
    #[test]
    fn test_fn_closure_compatibility() {
        fn call_fn(f: fn(i32) -> i32, val: i32) -> i32 {
            f(val)
        }

        let f = |x| x + 1;
        assert_eq!(call_fn(f, 4), 5);
    }

    /// 如果闭包捕获了外部变量，则不兼容函数指针类型。
    #[test]
    fn test_fn_closure_incompatibility() {
        #[allow(dead_code)]
        fn call_fn(f: fn(i32) -> i32, val: i32) -> i32 {
            f(val)
        }

        fn call_fn_2<F>(f: F, val: i32) -> i32
        where
            F: Fn(i32) -> i32,
        {
            f(val)
        }

        let offset = 2;
        let f = |x: i32| x + offset;

        // call_fn(f, 10); // ❌ 编译器错误：mismatched types
        assert_eq!(call_fn_2(f, 8), 10); // ✅
    }

    /// 函数返回值是函数/闭包
    #[test]
    fn test_fn_returns_closure() {
        fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
            move |y| x + y // move 让闭包捕获 `x` 的所有权
        }

        let add_5 = make_adder(5);
        assert_eq!(add_5(10), 15);
        assert_eq!(add_5(15), 20);
    }

    #[test]
    fn test_fn_returns_mut_closure() {
        fn make_counter() -> impl FnMut() -> i32 {
            let mut count = 0;
            move || {
                count += 1;
                count
            }
        }

        let mut counter = make_counter();

        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
    }

    #[test]
    fn test_fn_argument_match() {
        struct Point {
            x: i32,
            y: i32,
        }
        fn move_by(Point { x, y }: Point, dx: i32, dy: i32) -> Point {
            Point { x: x + dx, y: y + dy }
        }

        let p = Point { x: 3, y: 7 };
        let p = move_by(p, 7, 3);
        assert_eq!(p.x, 10);
        assert_eq!(p.y, 10);
    }
}
