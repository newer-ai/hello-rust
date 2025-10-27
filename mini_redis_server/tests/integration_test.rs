use mini_redis_server::db::Db;
use mini_redis_server::handler::process_command;

#[tokio::test]
async fn test_end_to_end() {
    let db = Db::new();

    process_command(&db, "SET foo 42").await;

    let result = process_command(&db, "GET foo").await;

    assert_eq!(result, "42");
}
