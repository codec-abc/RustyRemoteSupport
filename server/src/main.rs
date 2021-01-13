use warp::path;
use warp::Filter;

#[tokio::main]
async fn main() {


    println!("Answer to everything is {}", shared::ANSWER_TO_EVERYTHING);

    let index = 
        warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("./www/static/index.html"));

    let static_files = 
        warp::get().and(warp::path("static")).and(warp::fs::dir("./www/static"));

    let routes = index.or(static_files);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await;
}