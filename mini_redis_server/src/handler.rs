//! 命令处理模块
//!
//! 负责执行具体命令逻辑：
//! 1. 解析输入字符串为 Command；
//! 2. 调用数据库操作；
//! 3. 返回结果字符串。
//!
//! 模块设计目标：
//! - 与 I/O 解耦（纯逻辑层）
//! - 可独立单元测试

use crate::{command::Command, db::Db};

/// 处理一条命令行字符串，返回执行结果。
///
/// # 参数
/// * `db` - 共享数据库引用
/// * `input` - 客户端输入命令行字符串
///
/// # 返回
/// * 返回 Redis 风格的字符串响应：例如 `"OK"` 或 `"ERR ..."`
pub async fn process_command(db: &Db, input: &str) -> String {
    let command: Command = Command::parse(input);

    match command {
        Command::Get(key) => match db.get(&key).await {
            Some(value) => value,
            None => "(nil)".into(),
        },
        Command::Set(key, value) => {
            db.set(key, value).await;
            "OK".into()
        }
        Command::Unknown => "ERR unknown command".into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::Db, handler::process_command};

    #[tokio::test]
    async fn test_get_missing_key() {
        let db = Db::new();

        let expected = "(nil)";

        let actual = process_command(&db, "get foo").await;

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn test_get_set() {
        let db = Db::new();

        assert_eq!(process_command(&db, "set foo bar").await, "OK");

        assert_eq!(process_command(&db, "get foo").await, "bar");
    }

    #[tokio::test]
    async fn test_unknown() {
        let db = Db::new();

        assert_eq!(process_command(&db, "???").await, "ERR unknown command");
    }
}
