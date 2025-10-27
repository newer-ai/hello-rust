//! 内存数据库模块
//!
//! 封装一个基于 `RwLock<HashMap>` 的简单键值数据库。
//! 支持异步 get / set 操作。
//!
//! 特点：
//! - 多任务共享（通过 `Arc` 实现）
//! - 并发安全（通过 `RwLock` 实现）
//! - 异步友好

use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

/// 异步可共享的数据库类型
#[derive(Clone, Default)]
pub struct Db {
    /// 内部存储结构： RwLock 确保并发安全
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl Db {
    /// 创建一个新的空数据库
    pub fn new() -> Self {
        Self::default()
    }

    /// 异步读取键的值
    pub async fn get(&self, key: &str) -> Option<String> {
        let guard = self.inner.read().await;
        guard.get(key).cloned()
    }

    /// 异步写入键的值
    pub async fn set(&self, key: String, value: String) {
        let mut guard = self.inner.write().await;

        guard.insert(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_missing_key() {
        let db = Db::new();
        assert_eq!(db.get("nope").await, None);
    }

    #[tokio::test]
    async fn test_db_get_set() {
        let db = Db::new();

        db.set("foo".into(), "bar".into()).await;
        assert_eq!(db.get("foo").await, Some("bar".into()));
    }
}
