use database::connect::Database;

mod database;

#[tokio::main]
async fn main() {
    let db = Database::connect_database().await;
    println!("{:#?}", db.unwrap().connection.await);
}
