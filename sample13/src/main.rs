use futures::future::Future;
use futures::executor;

fn something_great_async_function() -> impl Future<Output = i32> {
    async {
        let ans = async_add(2, 3).await;
        println!("{}", ans);
        ans
    }
}

//use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

fn move_to_async_block() -> impl Future<Output = ()> {
    let outside_variable = "this is outside".to_string();
    async move {
        println!("{}", outside_variable);
    }
}

// async fn something_great_async_function() -> i32 {
//     let ans = async_add(2, 3).await;
//     println!("{}", ans);
//     ans
// }

fn main() {
    //executor::block_on(something_great_async_function());
    move_to_async_block();
}
