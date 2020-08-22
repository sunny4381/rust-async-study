use std::future::Future;
use std::string::String;
use futures::executor::block_on;

async fn hello_world(name: &String) {
    println!("hello, {}!", name);
}

//below function signature causes lifetime errors
//fn hello_world2(name: &String) -> impl Future<Output = ()> {

// you must specify lifetime of argument and return value explicitly
fn hello_world2<'a>(name: &'a String) -> impl Future<Output = ()> + 'a {
    async move {
        println!("hello, {}!", name);
    }
}

async fn async_main() {
    let name = String::from("takahashi");

    futures::join!(hello_world(&name), hello_world2(&name));
}

fn main() {
    block_on(async_main());
}
