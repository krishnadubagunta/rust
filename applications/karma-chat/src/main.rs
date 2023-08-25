use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::path!("sum" / u32 / u32)
        .map(|a,b| {
            format!("{} + {} = {}", a,b,a+b)
        });

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}