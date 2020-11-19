use serde_json::{json, Value};
use tokio::runtime::Runtime;
use structopt::StructOpt;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::traits::*;

mod config;
mod ui;

#[derive(StructOpt)]
struct Cli {
    /// Remove existing API key
    #[structopt(short, long)]
    remove: bool,
}

struct Project {
    count: i32,
    id: String,
    name: String,
    shared: String,
}

struct Section {
    id: String,
    project_id: String,
    order: i32,
    real_order: String,
    name: String,
}

struct Task {
    id: String,
    section_id: String,
    order: i32,
    real_order: String,
    content: String,
    priority: String,
    created: String,
    due: String,
}

fn project_overview() {
    let conf: config::Config = confy::load("coto").expect("Could not load config");
    let key: String = conf.todoist_key;

    // Gather 'Projects'
    let project_call = Runtime::new().expect("Could not query projects")
        .block_on(coto::get_all_projects(&key))
        .unwrap();

    let project_count = project_call.matches("}").count();
    let project_json: Value = serde_json::from_str(&project_call).unwrap();
    let mut project_vec: Vec<Project> = Vec::new();
    for p in 0..project_count {
        project_vec.push(Project {
            count: p as i32,
            id: project_json[p]["id"].to_string(),
            name: project_json[p]["name"].to_string(),
            shared: project_json[p]["shared"].to_string(),
        });
    }

    // Display the overview
    let mut ui = cursive::default();
    ui.load_toml(include_str!("../styles/original.toml")).unwrap();

    let mut select = SelectView::new();
    for project in project_vec.iter() {
        select.add_item(project.name.to_string(), project.id.to_string())
    }
    select.set_on_submit(task_overview);

    // Display controls
    let controls = "[C]reate project\n[D]elete project\n[U]pdate project\n[S]ettings\n[Q]uit";

    ui.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(TextView::new(controls)))
        .title("Projects")
        .h_align(HAlign::Center));

    // Control callbacks

    // ** Create **
    // ** Delete **
    // ** Update **
    // ** Settings **
    // ** Quit **

    ui.run();
}

fn task_overview(ui: &mut Cursive, id: &String) {
    let conf: config::Config = confy::load("coto").expect("Could not load config");
    let key: String = conf.todoist_key;

    // Gather 'Sections' and 'Tasks'
    let section_call = Runtime::new().expect("Could not query sections")
        .block_on(coto::get_project_sections(&key, id))
        .unwrap();
    let section_count = section_call.matches("}").count();
    let section_json: Value = serde_json::from_str(&section_call).unwrap();

    let mut section_vec: Vec<Section> = Vec::new();
    let mut section_counter: i32 = 1;
    for s in 0..section_count {
        section_vec.push(Section {
            id: section_json[s]["id"].to_string(),
            project_id: section_json[s]["project_id"].to_string(),
            order: section_counter,
            real_order: section_json[s]["order"].to_string(),
            name: section_json[s]["name"].to_string(),
        });
        section_counter += 1;
    }

    let task_call = Runtime::new().expect("Could not query tasks")
        .block_on(coto::get_all_tasks(&key))
        .unwrap();
    let task_count = task_call.matches("}").count();
    let task_json: Value = serde_json::from_str(&task_call).unwrap();

    let mut task_vec: Vec<Task> = Vec::new();
    let mut task_counter: i32 = 1;
    for t in 0..task_count {
        task_vec.push(Task {
            id: task_json[t]["id"].to_string(),
            section_id: task_json[t]["section_id"].to_string(),
            order: task_counter,
            content: task_json[t]["content"].to_string(),
            priority: task_json[t]["priority"].to_string(),
            created: task_json[t]["created"].to_string(),
            due: task_json[t]["due"]["date"].to_string(),
            real_order: task_json[t]["order"].to_string(),
        });
        task_counter += 1;
    }

    // Display the overview
    ui.pop_layer();

    let mut column = LinearLayout::vertical();

    for section in section_vec.iter() {
        let mut select = SelectView::new();
        for task in task_vec.iter() {
            if task.section_id == section.id {
                select.add_item(task.content.to_string(), task.id.to_string())
            }
        }
        column.add_child(TextView::new(section.name.to_string()));
        column.add_child(select);
    }

    // Display controls
    let controls = "Complete [T]ask\n[C]reate Task/Section\n[D]elete Task/Section\n[U]pdate Task/Section\nGo [B]ack\n[S]ettings";

    ui.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(column)
            .child(TextView::new(controls)))
        .title("Tasks")
        .h_align(HAlign::Center));

    // Control callbacks

    // ** Complete **
    // ** Create **
    // ** Delete **
    // ** Update **
    // ** Back **
    // ** Settings **
}

fn main() {
    // Config setup
    if Cli::from_args().remove == true {
        config::remove_key();
    }

    project_overview();
}
