use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

async fn async_main() {
    hello_world().await;
}

fn main() {
    block_on(async_main());
}
