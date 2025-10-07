use std::{f64::consts::PI, fmt::Display};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Circle {
    radius: f64,
}

#[allow(dead_code)]
impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn grow(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct Config {
    port: u16,
    debug: bool,
}

#[allow(dead_code)]
struct Pair<T> {
    first: T,
    second: T,
}

#[allow(dead_code)]
struct RefPoint<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}
#[allow(dead_code)]
impl<'a> RefPoint<'a> {
    fn new(x: &'a mut i32, y: &'a mut i32) -> Self {
        RefPoint { x, y }
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
        *self.x += dx;
        *self.y += dy;
    }
}

#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    Quit,                       // 不带数据
    Move { x: i32, y: i32 },    // 带字段
    Write(String),              // 带一个数据
    ChangeColor(i32, i32, i32), // 带多个值
}

#[allow(dead_code)]
impl Message {
    fn process(&self) -> String {
        match self {
            Message::Quit => "退出游戏".to_string(),
            Message::Move { x, y } => format!("移动到坐标 ({x}, {y})"),
            Message::Write(text) => format!("写入文本：{text}"),
            Message::ChangeColor(r, g, b) => format!("颜色：({r}, {g}, {b})"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
impl Display for IpAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            v4 @ IpAddr::V4(127, 0, 0, 1) => write!(f, "本地环回地址：{:?}", v4),
            IpAddr::V4(a, b, c, d) => write!(f, "IPv4: {a}.{b}.{c}.{d}"),
            IpAddr::V6(addr) => write!(f, "IPv6: {addr}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::{Circle, Config, Direction, IpAddr, Message, Pair, Person, Point, RefPoint};

    #[test]
    fn test_struct_copy_trait() {
        let p1 = Point { x: 1, y: 2 };
        let mut p2 = p1;

        p2.x = 10;
        p2.y = 20;

        assert_eq!(p1.x, 1);
        assert_eq!(p1.y, 2);
        assert_eq!(p2.x, 10);
        assert_eq!(p2.y, 20);
    }

    #[test]
    fn test_struct_clone_trait() {
        let p1 = Person {
            name: "Alice".to_string(),
            age: 21,
        };
        let p2 = p1;

        // println!("{:?}", p1); // ❌ 编译错误：p1 已经失效。value borrowed here after move
        println!("{:?}", p2);
        assert_eq!(p2.name, "Alice".to_string());
        assert_eq!(p2.age, 21);
    }

    #[test]
    fn test_struct_move_field_value() {
        let p = Person {
            name: "Alice".to_string(),
            age: 18,
        };

        assert_eq!(p.name, "Alice".to_string()); // 临时变量，未发生所有权转移
        let name = p.name; // 发生了 move 操作，所有权已经转移
        assert_eq!(name, "Alice".to_string());

        // let name = p.name; // ❌ 编译错误：use of moved value: `p.name`
    }

    #[test]
    fn test_struct_new() {
        let c = Circle::new(1.);

        assert_eq!(c.area(), PI);
    }

    #[test]
    fn test_struct_mut_fn() {
        let mut c = Circle::new(1.);

        c.grow(2.);

        assert_eq!(c.radius, 2.);
    }

    #[test]
    fn test_struct_update_syntax() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 5, ..p1 };

        assert_eq!(p2.x, 5);
        assert_eq!(p2.y, 2);
    }

    #[test]
    fn test_struct_destructuring() {
        let p = Point { x: 3, y: 7 };
        let Point { x, y } = p;
        assert_eq!(x, 3);
        assert_eq!(y, 7);

        let Point { x, .. } = p; // 只取部分字段
        assert_eq!(x, 3);
    }

    #[test]
    fn test_struct_new_2() {
        let p = Point::new(1, 2);

        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_struct_default() {
        let c = Config {
            debug: true,
            ..Default::default()
        };

        assert_eq!(c.debug, true);
        assert_eq!(c.port, 0);
    }

    #[test]
    fn test_struct_generic() {
        let p = Pair {
            first: 1,
            second: 2,
        };

        assert_eq!(p.first, 1);
        assert_eq!(p.second, 2);
    }

    #[test]
    fn test_struct_lifetime() {
        let mut x = 0;
        let mut y = 1;
        let mut p = RefPoint::new(&mut x, &mut y);
        p.move_by(10, 20);

        assert_eq!(*p.x, 10);
        assert_eq!(*p.y, 21);

        assert_eq!(x, 10);
        assert_eq!(y, 21);
    }

    fn move_player(direction: Direction) -> String {
        match direction {
            Direction::Up => "向上移动".to_string(),
            Direction::Down => "向下移动".to_string(),
            Direction::Left => "向左移动".to_string(),
            Direction::Right => "向右移动".to_string(),
        }
    }

    #[test]
    fn test_enum_match() {
        assert_eq!(move_player(Direction::Up), "向上移动");
        assert_eq!(move_player(Direction::Down), "向下移动");
        assert_eq!(move_player(Direction::Left), "向左移动");
        assert_eq!(move_player(Direction::Right), "向右移动");
    }

    #[test]
    fn test_enum_with_data() {
        assert_eq!(Message::Quit.process(), "退出游戏");
        assert_eq!(Message::Move { x: 1, y: 3 }.process(), "移动到坐标 (1, 3)");
        assert_eq!(
            Message::Write("Hello".to_string()).process(),
            "写入文本：Hello"
        );
        assert_eq!(
            Message::ChangeColor(255, 100, 0).process(),
            "颜色：(255, 100, 0)"
        );
    }

    #[test]
    fn test_enum_bind_variable() {
        let localhost = IpAddr::V4(127, 0, 0, 1);
        assert_eq!(format!("{localhost}"), "本地环回地址：V4(127, 0, 0, 1)");

        let ipv4 = IpAddr::V4(192, 168, 90, 100);
        assert_eq!(format!("{ipv4}"), "IPv4: 192.168.90.100");

        let ipv6 = IpAddr::V6(String::from("2001:db8::8a2e:370:7334"));
        assert_eq!(format!("{ipv6}"), "IPv6: 2001:db8::8a2e:370:7334");
    }

    #[test]
    fn test_enum_if_let() {
        let msg = Message::Write("Hello".into());

        let text = if let Message::Write(text) = msg {
            Some(format!("内容：{text}"))
        } else {
            None
        };

        assert_eq!(text, Some("内容：Hello".to_string()));
    }

    #[test]
    fn test_enum_matches() {
        let msg = Message::Quit;

        assert_eq!(matches!(msg, Message::Quit), true);
    }

    #[test]
    fn test_enum_match_with_condition() {
        let msg = Message::Move { x: 5, y: 10 };
        let result = match msg {
            Message::Move { x, y } if x > 0 && y > 0 => "ok",
            _ => "other",
        };

        assert_eq!(result, "ok");
    }

    #[test]
    fn test_enum_matches_with_condition() {
        let msg = Message::Move { x: 5, y: 10 };

        assert!(matches!(msg, Message::Move { x, y } if x > 0 && y > 0));
    }

    #[test]
    fn test_enum_if_let_match_guard() {
        let msg = Message::Move { x: 0, y: 10 };

        let result = if let Message::Move { x, .. } = msg
            && x == 0
        {
            "垂直移动"
        } else if matches!(msg, Message::Move { .. }) {
            "其他移动"
        } else {
            "其他动作"
        };

        assert_eq!(result, "垂直移动");
    }

    #[test]
    fn test_enum_match_match_guard() {
        let msg = Message::Move { x: 0, y: 10 };
        let result = match msg {
            Message::Move { x, .. } if x == 0 => "垂直移动",
            Message::Move { .. } => "其他移动",
            _ => "其他动作",
        };

        assert_eq!(result, "垂直移动");
    }

    #[test]
    fn test_enum_move() {
        let msg = Message::Write("Hello".into());

        match msg {
            // 简单使用，不会发生所有权move
            Message::Quit => println!("退出游戏"),
            _ => {}
        };

        match msg {
            // `text` 取得了 String 的所有权，所以 msg 也就发生了所有权 move
            Message::Write(text) => println!("内容: {text}"),
            _ => {}
        };

        // println!("msg 不可以使用: {:?}", msg);     // 编译错误：borrow of partially moved value: `msg`
    }

    #[test]
    fn test_enum_borrow() {
        let msg = Message::Write("Hello".into());

        match &msg {
            Message::Write(text) => println!("内容：{}", text),
            _ => {}
        };

        println!("msg 依然可以使用: {msg:?}");
    }
}
