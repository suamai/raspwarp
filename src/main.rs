use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /
    let hello_world = warp::fs::dir("/home/sua/git/suamai/raspwarp/public");

    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    let times =
        warp::path!(u16 / "times" / u16).map(|a, b| format!("{} times {} = {}", a, b, a * b));

    let help = warp::path::end()
        .map(|| "This is the Math API. Try calling /math/sum/:u32/:u32 or /math/:u16/times/:u16");

    let math = warp::path("math").and(help.or(sum).or(times));

    let routes = warp::get().and(hello_world.or(math));

    warp::serve(routes).run(([127, 0, 0, 1], 80)).await;
}
