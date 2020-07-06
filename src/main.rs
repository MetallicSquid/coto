use tokio::runtime::Runtime;
use reqwest::header;

fn main() {
    Runtime::new().expect("Failed").block_on(coto::get_function());
}


