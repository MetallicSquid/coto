use tokio::runtime::Runtime;
use console::{Term, Style};
use dialoguer::Input;

//Runtime::new().expect("Failed").block_on(coto::get_function());

fn main() {
    loop {
        let user_input = Input::<String>::new().with_prompt(" >>>   ").interact()?;
        println!("{}", user_input);
    }
}


