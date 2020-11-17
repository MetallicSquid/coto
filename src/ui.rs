// ##### Cursive UI #####

// This is the command
// line interface for the
// application.

use tokio::runtime::Runtime;
use serde_json::{json, Value};
use cursive::views::{Dialog, TextView};

pub fn project_overview() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("../styles/original.toml")).unwrap();

    siv.add_layer(Dialog::text("This is a survey!\nPress <Next> when you're ready.")
        .title("Important survey")
        .button("Next", show_next));

    siv.run();
}

fn show_next(s: &mut cursive::Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("A pertinent question")
        .title("C'est un question important")
        .button("Ouais", |s| show_answer(s, "bon"))
        .button("Non", |s| show_answer(s, "d'acc"))
        .button("Quoi", |s| s.add_layer(Dialog::info("bof"))));
}

fn show_answer(s: &mut cursive::Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(msg)
        .title("Wow")
        .button("Finish", |s| s.quit()));
}
