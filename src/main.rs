use structopt::StructOpt;

mod config;
mod ui;

#[derive(StructOpt)]
struct Cli {
    /// Remove existing API key
    #[structopt(short, long)]
    remove: bool,
}

// I think I should reduce the number of strings in these structs
struct Project {
    count: i32,
    id: String,
    name: String,
    shared: String,
}

struct Section {
    id: String,
    project_id: String,
    order: String,
    real_order: String,
    name: String,
}

struct Task {
    id: String,
    section_id: String,
    order: String,
    real_order: String,
    content: String,
    priority: String,
    created: String,
    due: String,
}

fn main() {
    if Cli::from_args().remove == true {
        config::remove_key();
    }
    let key: String = config::config_setup().unwrap();

    ui::project_overview();
}
