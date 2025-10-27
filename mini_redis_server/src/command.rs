//! 命令解析模块
//!
//! 负责从字符串解析出 Redis 命令的抽象结构。
//! 当前仅支持 GET / SET / Unknown 三类命令。
//!
//! 在未来可扩展为 RESP 协议解析层。

/// 代表 mini-redis 支持的命令
#[derive(PartialEq, Debug)]
pub enum Command {
    /// GET <key>: 获取键的值
    Get(String),
    /// SET <key> <value>: 设置键的值
    Set(String, String),
    /// 未知命令
    Unknown,
}

impl Command {
    /// 从用户输入（如 `SET foo bar`）解析出命令结构
    pub fn parse(input: &str) -> Self {
        let parts: Vec<_> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            [name, key] if name.eq_ignore_ascii_case("get") => Command::Get(key.to_string()),
            [name, key, value] if name.eq_ignore_ascii_case("set") => {
                Command::Set(key.to_string(), value.to_string())
            }
            _ => Command::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn test_parse_get_command() {
        let expected = Command::Get("foo".to_string());

        let actual = Command::parse("get foo");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_set_command() {
        let expected = Command::Set("foo".to_string(), "bar".to_string());

        let actual = Command::parse("set foo bar");

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_unknown_command() {
        let expected = Command::Unknown;

        let actual = Command::parse("abc abc abc");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_ignore_multiwhitespaces() {
        let expected = Command::parse("get foo");

        let actual = Command::parse("get    foo ");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_ignore_case() {
        let expected = Command::parse("get foo");
        let actual = Command::parse("Get foo");
        assert_eq!(expected, actual);

        let expected = Command::parse("set foo bar");
        let actual = Command::parse("SET foo bar");
        assert_eq!(expected, actual);
    }
}
