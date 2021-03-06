use serde_json::{json, Value};
use tokio::runtime::Runtime;
use structopt::StructOpt;
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::views::{Dialog, EditView, LinearLayout, SelectView,
    TextView, OnEventView, TextArea, TextContent};
use cursive::traits::*;

mod config;
mod query;

#[derive(StructOpt)]
struct Cli {
    /// Remove existing API key
    #[structopt(short, long)]
    remove: bool,
}

#[derive(Debug)]
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

fn main() {
    // Config setup
    if Cli::from_args().remove == true {
        config::remove_key();
    }
    config::config_setup();

    let mut ui = cursive::default();
    project_overview(&mut ui);
}

fn project_overview(ui: &mut Cursive) {
    let project_vec = gather_projects().unwrap();

    // Display the overview
    ui.pop_layer();
    ui.load_toml(include_str!("../styles/original.toml")).unwrap();

    let mut select = SelectView::new();
    for project in project_vec.iter() {
        select.add_item(project.name.to_string(), project.id.to_string());
    }
    select.set_on_submit(task_overview);

    // Display controls
    let controls = "[C]reate project\n[D]elete project\n[U]pdate project\n[S]ettings\n[Q]uit";

    // Control callbacks
    let callbacks = OnEventView::new(select)
        // Create Project
        .on_event('c', |ui| {
            fn ok(ui: &mut Cursive, name: &str) {
                let conf_call: config::Config = confy::load("coto").expect("Could not load config");
                let key_call: String = conf_call.todoist_key;
                let json_name = json!({"name": name}).to_string();
                Runtime::new().expect("Could not create new project")
                    .block_on(query::new_project(&key_call, json_name))
                    .unwrap();
                project_overview(ui);
            }

            ui.pop_layer();
            ui.add_layer(Dialog::new()
                .content(EditView::new().on_submit(ok))
                .title("New project name"));
        })
        // Delete Project
        .on_event('d', |ui| {
            fn ok(ui: &mut Cursive, project_id: &str) {
                let conf_call: config::Config = confy::load("coto").expect("Could not load config");
                let key_call: String = conf_call.todoist_key;

                Runtime::new().expect("Could not delete project")
                    .block_on(query::delete_project(&key_call, &project_id.to_string()))
                    .unwrap();
                project_overview(ui);
            }

            let mut sub_select = SelectView::new();
            let projects = gather_projects().unwrap();
            for project in projects.iter() {
                sub_select.add_item(project.name.to_string(), project.id.to_string());
            }
            sub_select.set_on_submit(ok);
            let delete_callback = OnEventView::new(sub_select)
                .on_event('b', project_overview)
                .on_event('q', |ui| ui.quit());

            ui.pop_layer();
            ui.add_layer(Dialog::around(LinearLayout::vertical()
                    .child(TextView::new("Be careful, once it's gone - it's gone.\nYou can go [B]ack or [Q]uit if you want. "))
                    .child(delete_callback))
                .title("Project to delete"));

        })
        // Update Project
        .on_event('u', |ui| {
            fn ok(ui: &mut Cursive, project_id: &str) {
                let conf_call: config::Config = confy::load("coto").expect("Could not load config");
                let key_call: String = conf_call.todoist_key;

                let mut new_name: String = String::new();
                // Just need to find a way to chain text entries, then the whole thing will come together

                let json_name = json!({"name": &new_name}).to_string();
                Runtime::new().expect("Could not update project")
                    .block_on(query::update_project(&key_call, &project_id.to_string(), json_name))
                    .unwrap();
                project_overview(ui);
            }

            let mut sub_select = SelectView::new();
            let projects = gather_projects().unwrap();
            for project in projects.iter() {
                sub_select.add_item(project.name.to_string(), project.id.to_string());
            }
            sub_select.set_on_submit(ok);

            let update_callback = OnEventView::new(sub_select)
                .on_event('b', project_overview)
                .on_event('q', |ui| ui.quit());

            ui.pop_layer();
            ui.add_layer(Dialog::around(LinearLayout::vertical()
                    .child(TextView::new("You can go [B]ack or [Q]uit if you want."))
                    .child(update_callback))
                .title("Project to update"));


        })
        .on_event('s', |ui| {})
        .on_event('q', |ui| ui.quit());

    ui.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(callbacks)
            .child(TextView::new(controls)))
        .title("Projects")
        .h_align(HAlign::Center));

    ui.run();
}

fn task_overview(ui: &mut Cursive, id: &str) {
    let section_vec = gather_sections(id).unwrap();
    let task_vec = gather_tasks().unwrap();

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

    let callbacks = OnEventView::new(column)
        .on_event('t', |ui| {})
        .on_event('c', |ui| {})
        .on_event('d', |ui| {})
        .on_event('u', |ui| {})
        .on_event('b', project_overview)
        .on_event('s', |ui| {});

    ui.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(callbacks)
            .child(TextView::new(controls)))
        .title("Tasks")
        .h_align(HAlign::Center));
}

fn gather_projects() -> Result<Vec<Project>, Box<dyn std::error::Error>> {
    let conf: config::Config = confy::load("coto").expect("Could not load config");
    let key: String = conf.todoist_key;

    let project_call = Runtime::new().expect("Could not query projects")
        .block_on(query::get_all_projects(&key))
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

    println!("{:?}", project_vec);
    Ok(project_vec)
}

fn gather_sections(id: &str) -> Result<Vec<Section>, Box<dyn std::error::Error>> {
    let conf: config::Config = confy::load("coto").expect("Could not load config");
    let key: String = conf.todoist_key;

    let section_call = Runtime::new().expect("Could not query sections")
        .block_on(query::get_project_sections(&key, &id.to_string()))
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

    Ok(section_vec)
}

fn gather_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let conf: config::Config = confy::load("coto").expect("Could not load config");
    let key: String = conf.todoist_key;

    let task_call = Runtime::new().expect("Could not query tasks")
        .block_on(query::get_all_tasks(&key))
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

    Ok(task_vec)
}
