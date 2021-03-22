use async_trait::async_trait;

async fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[async_trait]
trait AsyncTrait {
    async fn f() {
        println!("Could compile")
    }
}

#[async_std::main]
async fn main() {
    let ans = add(2, 3).await;
    println!("{}", ans);
}
