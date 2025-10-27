use std::f64::consts::PI;

#[allow(dead_code)]
fn format_score(score: u8) -> &'static str {
    match score {
        90..=100 => "优秀",
        70..90 => "良好",
        60..70 => "及格",
        _ => "不及格",
    }
}

#[allow(dead_code)]
fn describe_number(x: i32) -> &'static str {
    match x {
        n if n % 2 == 1 && n < 10 => "奇数小于10",
        n if n % 2 == 0 && n < 10 => "偶数小于10",
        _ => "其他数字",
    }
}

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    #[allow(dead_code)]
    fn describe(&self) -> String {
        match self {
            Point { x: 0, y: 0 } => format!("在原点上"),
            Point { x: 0, y } => format!("在 y 轴上， y = {y}"),
            Point { x, y: 0 } => format!("在 x 轴上， x = {x}"),
            Point { x, y } => format!("在其他位置上， x = {x}, y = {y}"),
        }
    }

    #[allow(dead_code)]
    fn describe2(&self) -> String {
        match (self.x, self.y) {
            (0, 0) => format!("在原点上"),
            (0, y) => format!("在 y 轴上， y = {y}"),
            (x, 0) => format!("在 x 轴上， x = {x}"),
            (x, y) => format!("在其他位置上， x = {x}, y = {y}"),
        }
    }
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl From<Message> for String {
    fn from(value: Message) -> Self {
        match value {
            Message::Quit => "退出".to_string(),
            Message::Move { x, y } => format!("移动到 {x}, {y}"),
            Message::Write(content) => format!("写入： {content}"),
        }
    }
}

#[allow(dead_code)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    #[allow(dead_code)]
    fn area(&self) -> f64 {
        match self {
            Self::Circle(radius) => PI * radius.powi(2),
            Self::Rectangle(width, height) => width * height,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum Event {
    KeyPress(char),
    MouseClick { x: i32, y: i32 },
    Resize { width: i32, height: i32 },
}

#[allow(dead_code)]
fn handle_event(event: Event) -> String {
    match event {
        Event::KeyPress(key) if ('0'..='9').contains(&key) => format!("数字键 {key} 被按下"),
        Event::KeyPress(key) => format!("字符键 {key} 被按下"),
        Event::MouseClick { x, y } if (0..100).contains(&x) && (0..100).contains(&y) => {
            format!("点击在左上角， x={x}, y={y}")
        }
        Event::MouseClick { x, y } => format!("点击在其他位置， x={x}, y={y}"),
        e @ Event::Resize { width, height } => {
            format!("窗口大小调整为 ({width}, {height}) -> 事件对象：{:?}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 模式支持范围匹配
    #[test]
    fn test_match_range() {
        assert_eq!(format_score(90), "优秀");
        assert_eq!(format_score(89), "良好");
        assert_eq!(format_score(69), "及格");
        assert_eq!(format_score(50), "不及格");
    }

    /// 模式匹配：变量绑定
    #[test]
    fn test_match_binding() {
        let x = 42;

        let result = match x {
            n @ 1..=100 => format!("在 1~100 之间，值是 {n}"),
            _ => "其他值".to_string(),
        };

        assert_eq!(result, "在 1~100 之间，值是 42");
    }

    /// 多模式匹配（or）
    #[test]
    fn test_match_or() {
        let x = 2;

        let result = match x {
            1 | 2 | 3 => "1、2 或 3",
            4 => "4",
            _ => "其他",
        };

        assert_eq!(result, "1、2 或 3");
    }

    /// 匹配守卫
    #[test]
    fn test_match_guard() {
        assert_eq!(describe_number(1), "奇数小于10");
        assert_eq!(describe_number(2), "偶数小于10");
        assert_eq!(describe_number(100), "其他数字");
    }

    /// 结构体匹配
    #[test]
    fn test_match_struct() {
        assert_eq!(Point::new(0, 0).describe(), "在原点上");
        assert_eq!(Point::new(2, 0).describe(), "在 x 轴上， x = 2");
        assert_eq!(Point::new(0, 3).describe(), "在 y 轴上， y = 3");
        assert_eq!(Point::new(2, 3).describe(), "在其他位置上， x = 2, y = 3");
    }

    /// 元组模式匹配
    #[test]
    fn test_match_tuple() {
        assert_eq!(Point::new(0, 0).describe2(), "在原点上");
        assert_eq!(Point::new(2, 0).describe2(), "在 x 轴上， x = 2");
        assert_eq!(Point::new(0, 3).describe2(), "在 y 轴上， y = 3");
        assert_eq!(Point::new(2, 3).describe2(), "在其他位置上， x = 2, y = 3");
    }

    /// 枚举模式匹配
    #[test]
    fn test_match_enum() {
        let message: String = Message::Quit.into();
        assert_eq!(message, "退出");

        let messge: String = Message::Move { x: 2, y: 3 }.into();
        assert_eq!(messge, "移动到 2, 3".to_string());

        let message: String = Message::Write("你好！".to_string()).into();
        assert_eq!(message, "写入： 你好！")
    }

    #[test]
    fn test_match_enum_2() {
        assert_eq!(Shape::Circle(2.).area(), PI * 4.);
        assert_eq!(Shape::Rectangle(1.0, 2.0).area(), 2.0);
    }

    #[test]
    fn test_while_let() {
        let mut nums = vec![1, 2, 3];
        let mut size = 0;

        while let Some(_) = nums.pop() {
            size += 1;
        }

        println!("共弹出了 {size} 个数字");

        assert_eq!(size, 3);
    }

    #[test]
    fn test_for_in_move() {
        let nums = vec![1, 2, 3];
        let mut size = 0;

        for n in nums {
            println!("数字：{n}");
            size += 1;
        }

        println!("共输出了 {size} 个数字");

        assert_eq!(size, 3);
    }

    #[test]
    fn test_for_in_borrow() {
        let nums = vec![1, 2, 3];
        let mut size = 0;

        for n in nums.iter() {
            println!("数字：{n}");
            size += 1;
        }

        println!("共输出了 {size} 个数字");

        assert_eq!(size, 3);
        assert_eq!(nums.len(), 3);
    }

    #[test]
    fn test_match_guard_with_pattern_and_binding() {
        assert_eq!(handle_event(Event::KeyPress('0')), "数字键 0 被按下");
        assert_eq!(handle_event(Event::KeyPress('b')), "字符键 b 被按下");
        assert_eq!(handle_event(Event::MouseClick { x: 10, y: 10 }), "点击在左上角， x=10, y=10");
        assert_eq!(
            handle_event(Event::MouseClick { x: 210, y: 10 }),
            "点击在其他位置， x=210, y=10"
        );
        assert_eq!(
            handle_event(Event::Resize { width: 10, height: 20 }),
            "窗口大小调整为 (10, 20) -> 事件对象：Resize { width: 10, height: 20 }"
        );
    }

    #[test]
    fn test_matches_macro() {
        let event = Event::MouseClick { x: 50, y: 60 };

        assert!(matches!(event, Event::MouseClick { x: 0..100, y: 0..100 }));
        assert!(
            matches!(event, Event::MouseClick { x, y } if (0..100).contains(&x) && (0..100).contains(&y))
        );
    }

    #[test]
    fn test_matches_macro_with_guard() {
        let events = vec![Event::KeyPress('1'), Event::KeyPress('A'), Event::KeyPress('5')];

        let count = events.iter().filter(|e| matches!(e, Event::KeyPress('0'..'9'))).count();

        println!("数字键数量：{}", count);
        assert_eq!(count, 2);
    }

    /// 数组模式匹配
    #[test]
    fn test_match_array() {
        fn f(arr: &[i32]) -> String {
            match arr {
                [1, 2, 3] => "完全匹配 [1, 2, 3]".to_string(),
                [1, x, 3] => format!("中间元素是 {x}"),
                [first, middle @ .., last] => {
                    format!("first={first}, middle={middle:?}, last={last}")
                }
                _ => "其他情况".to_string(),
            }
        }

        assert_eq!(f(&[1, 2, 3]), "完全匹配 [1, 2, 3]");
        assert_eq!(f(&[1, 4, 3]), "中间元素是 4");
        assert_eq!(f(&[1, 2]), "first=1, middle=[], last=2");
        assert_eq!(f(&[1, 2, 3, 4]), "first=1, middle=[2, 3], last=4");
        assert_eq!(f(&[1]), "其他情况");
    }

    /// vec模式匹配
    #[test]
    fn test_match_vec() {
        fn f(v: &[i32]) -> String {
            match v {
                [a, b, c] => format!("三个元素： {a}, {b}, {c}"),
                [first, rest @ ..] => format!("first={first}, rest={rest:?}"),
                [] => "空 Vec".to_string(),
            }
        }

        assert_eq!(f(&vec![1, 2, 3]), "三个元素： 1, 2, 3");
        assert_eq!(f(&vec![1, 2, 3, 4]), "first=1, rest=[2, 3, 4]");
        assert_eq!(f(&vec![]), "空 Vec");
    }

    #[test]
    fn test_match_ref() {
        let arr = [String::from("a"), String::from("b")];

        let result = match arr {
            [ref first, ref second] => {
                format!("first = {first}, second = {second}")
            }
        };

        assert_eq!(result, "first = a, second = b");
        assert_eq!(arr.len(), 2);
    }
}
