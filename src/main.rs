use std::{env, path};
use warp::Filter;

#[tokio::main]
async fn main() {
    let home = env::var("HOME").expect("You must be on a unix like system");
    let index_path: path::PathBuf = [home.as_str(), "www", "static", "index.html"]
        .iter()
        .collect();

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(index_path));

    warp::serve(index).run(([127, 0, 0, 1], 3030)).await;
}
