use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::path!("sum" / u32 / u32)
        .map(|a,b| {
            format!("{} + {} = {}", a,b,a+b)
        });

    let routes_with_logs = routes.with(warp::log("ws server"));

    warp::serve(routes_with_logs)
        .run(([0, 0, 0, 0], 3030))
        .await;
}