use std::future::Future;
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn hello_world2() -> impl Future<Output = ()> {
    async {
        println!("hello, world2!");
    }
}

async fn async_main() {
    futures::join!(hello_world(), hello_world2());
}

fn main() {
    block_on(async_main());
}
