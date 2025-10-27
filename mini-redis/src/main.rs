use mini_redis_server::{db::Db, handler::process_command};

#[tokio::main]
async fn main() {
    let db = Db::new();
    println!("mini-redis (testing mode)");
    println!("{}", process_command(&db, "SET foo bar").await);
    println!("{}", process_command(&db, "GET foo").await);
}
