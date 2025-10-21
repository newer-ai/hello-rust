#[allow(dead_code)]
struct Holder<'a, T>
where
    T: std::fmt::Debug,
{
    value: &'a T,
}
#[allow(dead_code)]
impl<'a, T> Holder<'a, T>
where
    T: std::fmt::Debug,
{
    fn show(&self) {
        println!("{:?}", self.value);
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    #[test]
    fn test_generic_fn() {
        fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) {
            (b, a)
        }

        let a = 10.;
        let b = 20;
        let (a, b) = swap(a, b);
        assert_eq!(a, 20);
        assert_eq!(b, 10.);
    }

    #[test]
    fn test_generic_struct() {
        struct Point<T> {
            x: T,
            y: T,
        }

        let p1 = Point { x: 10, y: 10 };
        let p2 = Point { x: 1.5, y: 2.5 };

        assert_eq!(p1.x, 10);
        assert_eq!(p1.y, 10);
        assert_eq!(p2.x, 1.5);
        assert_eq!(p2.y, 2.5);
    }

    #[test]
    fn test_generic_struct_impl() {
        struct Container<T> {
            value: T,
        }
        impl<T: Copy> Container<T> {
            fn take_value(&self) -> T {
                self.value
            }
        }

        impl<T> Container<T> {
            fn get_value(&self) -> &T {
                &self.value
            }
        }

        let value = "Alice".to_string();
        let container = Container { value };
        let value = container.get_value();
        assert_eq!(value, "Alice");

        let value = 10;
        let container = Container { value };
        let value = container.take_value();
        println!("{}", value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_generic_enum() {
        enum ResultValue<T, E> {
            Ok(T),
            Err(E),
        }
        fn print_result<T, E>(result: &ResultValue<T, E>)
        where
            T: Display,
            E: Display,
        {
            match result {
                ResultValue::Ok(value) => println!("结果值：{value}"),
                ResultValue::Err(error) => println!("错误：{error}"),
            }
        }

        let v: ResultValue<String, String> = ResultValue::Ok("123".to_string());
        print_result(&v);

        let v: ResultValue<String, String> = ResultValue::Err("数据溢出".to_string());
        print_result(&v);
    }
}
