use backend::listener::test_listen;

#[tokio::main]
async fn main() {
    test_listen().await.unwrap();
}
