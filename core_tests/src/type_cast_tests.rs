#[cfg(test)]
mod tests {
    use std::slice;

    #[test]
    fn test_print_number_bitary() {
        let x = 100_000;
        let s = format!("{:b}", x);
        assert_eq!(s, "11000011010100000");

        let x = 1024.123f32;
        let bits = x.to_bits();
        let s = format!("{bits:032b}");
        assert_eq!(s, "01000100100000000000001111110000");
    }

    #[test]
    fn test_parse_number_from_binary() {
        let s = "1101";
        let n = u8::from_str_radix(s, 2);

        assert_eq!(n, Ok(13u8));
    }

    /// as 类型转换：整数 -> 整数
    ///
    /// 如果能表示则保留，否则溢出截断（按二进制位重新解释）
    #[test]
    fn test_as_int_to_int() {
        let i: i8 = 1;
        let i = i as u8;
        assert_eq!(i, 1);

        let i: i8 = -1; // 采用补码表示二进制：1111_1111
        let i = i as u8; // 对于 u8 来说，1111_1111 的解释是 2^8 - 1
        assert_eq!(i, 255);

        let i = 100_000; // 二进制：11000011010100000
        let i = i as u8; // 保留末尾的八位二进制：10100000
        assert_eq!(i, 160);
    }

    #[test]
    fn test_as_int_to_float() {
        let i = 102103;
        let i = i as f32;

        assert_eq!(i, 102103f32);
    }

    #[test]
    fn test_as_float_to_int() {
        let i: f32 = 1024.2123;
        let i = i as u8;

        assert_eq!(i, 255);

        let i: f32 = -5.0;
        let i = i as u8;
        assert_eq!(i, 0);
    }

    #[test]
    fn test_as_reference() {
        let x: i32 = 10;
        let r: &i32 = &x;
        let p: *const i32 = r as *const i32; // 转换为裸指针

        unsafe {
            let y = *p;
            assert_eq!(y, 10);
        }
    }

    #[test]
    fn test_as_mut_reference() {
        let mut x: i32 = 10;
        let r: &mut i32 = &mut x;
        let p: *mut i32 = r as *mut i32;

        unsafe {
            *p = *p + 1;
        }

        assert_eq!(x, 11);
    }

    #[test]
    fn test_safe_mut_reference() {
        let mut i: i32 = 10;
        let r: &mut i32 = &mut i;

        *r += 1;

        assert_eq!(i, 11);
    }

    fn sum_array(arr: *const i32, len: usize) -> i32 {
        let mut sum = 0;
        unsafe {
            for i in 0..len {
                sum += *arr.add(i);
            }
        }
        sum
    }

    #[test]
    fn test_unsafe() {
        let v = vec![1, 2, 3, 4, 5];
        let p = v.as_ptr();
        let actual = sum_array(p, v.len());

        assert_eq!(actual, 15);
    }

    #[test]
    fn test_from_into() {
        let o: i16 = -1;
        let i: i32 = o.into();
        assert_eq!(i, -1);

        let i = i32::from(o);
        assert_eq!(i, -1);
    }

    #[test]
    fn test_try_from_into() {
        let a: i32 = 300;
        let b = u8::try_from(a);

        assert!(b.is_err());
    }

    #[test]
    fn test_custom_from() {
        #[derive(PartialEq, Debug)]
        struct MyInt(i32);

        impl From<i32> for MyInt {
            fn from(value: i32) -> Self {
                MyInt(value)
            }
        }

        let a = MyInt::from(10);
        let b: MyInt = 10.into();

        assert_eq!(a, MyInt(10));
        assert_eq!(a, b);
    }

    #[test]
    fn test_custom_try_from() {
        #[derive(PartialEq, Debug)]
        struct Age(u8);

        impl TryFrom<i32> for Age {
            type Error = &'static str;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value >= 0 && value <= 120 {
                    Ok(Age(value as u8))
                } else {
                    Err("年龄不在合法范围")
                }
            }
        }

        assert_eq!(Age::try_from(12), Ok(Age(12)));
        assert!(Age::try_from(121).is_err());

        // 自动实现 try_into 方法
        let age = 12;
        let age: Result<Age, &'static str> = age.try_into();
        assert_eq!(age, Ok(Age(12)));
    }

    #[test]
    fn test_array_to_slice() {
        let arr = [1, 2, 3];
        let slice = &arr;

        assert_eq!(slice.len(), 3);

        let slice = &arr[0..2];
        assert_eq!(slice.len(), 2);
        assert_eq!(slice, [1, 2]);
    }

    #[test]
    fn test_array_to_mut_slice() {
        let mut arr = [1, 2, 3];
        let slice = &mut arr;

        slice[2] = 4;

        assert_eq!(arr, [1, 2, 4]);
    }

    #[test]
    fn test_slice_to_array() {
        let slice: &[i32] = &[1, 2, 3];

        let array: &[i32; 3] = slice.try_into().unwrap();
        let array = *array;

        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn test_vec_to_slice() {
        let v = vec![1, 2, 3, 4, 5];
        let slice: &[i32] = &v;

        assert_eq!(slice.len(), 5);

        let slice: &[i32] = &v[0..2];
        assert_eq!(slice, [1, 2]);
    }

    #[test]
    fn test_vec_to_mut_slice() {
        let mut v = vec![1, 2, 3];
        let slice: &mut [i32] = &mut v;

        slice[0] = 5;

        assert_eq!(v, [5, 2, 3]);
    }

    #[test]
    fn test_slice_to_vec() {
        let slice = &[1, 2, 3];
        let v = slice.to_vec();

        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_array_to_vec() {
        let arr = [1, 2, 3];
        let v = arr.to_vec();

        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_vec_to_array() {
        let vec = vec![1, 2, 3];
        let arr: [i32; 3] = vec.try_into().unwrap();

        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn test_slice_to_raw_pointer() {
        let slice = &[1, 2, 3];
        let ptr = slice.as_ptr();

        unsafe {
            assert_eq!(*ptr, 1);
            assert_eq!(*ptr.add(1), 2);
            assert_eq!(*ptr.add(2), 3);
        }
    }

    #[test]
    fn test_slice_to_mut_raw_pointer() {
        let slice: &mut [i32] = &mut [1, 2, 3];
        let ptr = slice.as_mut_ptr();

        unsafe {
            *ptr.add(1) += 1;
        }

        assert_eq!(slice, [1, 3, 3]);
    }

    #[test]
    fn test_raw_pointer_to_slice() {
        let slice = &[1, 2, 3];
        let ptr = slice.as_ptr();

        unsafe {
            let slice = slice::from_raw_parts(ptr, 3);
            assert_eq!(slice, [1, 2, 3]);
        }
    }

    #[test]
    fn test_raw_pointer_to_vec() {
        let mut v = vec![1, 2, 3, 4];

        let ptr = v.as_mut_ptr(); // 获取裸指针
        let len = v.len();
        let cap = v.capacity();

        std::mem::forget(v); // 不要 drop v，避免裸指针指向的数据被释放掉了

        unsafe {
            let new_v = Vec::from_raw_parts(ptr, len, cap);
            assert_eq!(new_v, [1, 2, 3, 4]);
        }
    }

    #[test]
    fn test_str_slice_to_string() {
        let s = "Hello, World!";

        let s = s.to_string();
        assert_eq!(s, "Hello, World!");

        let s: String = String::from(s);
        assert_eq!(s, "Hello, World!");
    }

    #[test]
    fn test_string_to_str_slice() {
        let string = String::from("Hello, World!");

        let s = string.as_str();
        assert_eq!(s, "Hello, World!");

        let s: &str = &string;
        assert_eq!(s, "Hello, World!");
    }

    fn string_len(str: &str) -> usize {
        str.len()
    }

    #[test]
    fn test_str_slice() {
        let string = String::from("Hello, World!");

        let s = &string;
        assert_eq!(string_len(s), 13);

        let s: &str = &string;
        assert_eq!(string_len(s), 13);
    }

    fn string_len2(str: &String) -> usize {
        str.len()
    }

    #[test]
    fn test_str_reference() {
        let string = String::from("Hello, World!");

        let s: &str = &string;
        assert_eq!(string_len2(&(s.to_string())), 13);
    }

    #[test]
    fn test_string_to_bytes() {
        let str = "Hello, World!".to_string();
        let bytes: Vec<u8> = str.into_bytes();

        assert_eq!(bytes, [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33],);
    }

    #[test]
    fn test_str_slice_to_bytes() {
        let str = "Hello, World!";
        let bytes: &[u8] = str.as_bytes();

        assert_eq!(bytes, [72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33],);
    }

    #[test]
    fn test_bytes_to_string() {
        let bytes = vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33];
        let string = String::from_utf8(bytes).unwrap();

        assert_eq!(string, "Hello, World!");
    }

    /// 联合体是一种特殊的数据结构：所有字段共享同一块内存，只能安全地读写当前活跃的那个字段。
    #[test]
    fn test_union() {
        union MyUnion {
            i: i32,
            f: f32,
        }

        let u = MyUnion { i: 89 };
        unsafe {
            println!("as i32: {}", u.i);
            println!("as f32: {}", u.f); // 不安全的例子
        }
    }
}
