use tokio::runtime::Runtime;
use console::{Term, Style};
use dialoguer::Input;
use serde_json::{json, Result, Value};
use std::{thread, time};

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
    name: String,
}
struct Task {
    id: String,
    section_id: String,
    order: String,
    content: String,
    priority: String,
    created: String,
    due: String,
}

// ##### Handle token input #####
fn main() {
    let token_input = Input::<String>::new()
        .with_prompt("API token")
        .interact()
        .expect("Could not collect token.");

    project_overview(token_input);
}

// ##### Display projects #####
fn project_overview(token_input: String) {
    let project_call = Runtime::new().expect("Could not query projects.")
        .block_on(coto::get_all_projects(&token_input))
        .unwrap();

    // Projects
    let project_count = project_call.matches("}").count();
    let json_projects: Value = serde_json::from_str(&project_call).unwrap();

    let mut project_vec: Vec<Project> = Vec::new();
    for p in 0..project_count {
        project_vec.push(Project {
            count: p as i32,
            id: json_projects[p]["id"].to_string(),
            name: json_projects[p]["name"].to_string(),
            shared: json_projects[p]["shared"].to_string(),
        });
    }

    // Display the overview
    let cyan = Style::new().cyan();
    let term = Term::stdout();
    term.clear_screen().expect("Could not clear screen.");
    println!("\nPress `Ctrl + C` to end session.");
    println!("\n\nYour projects:\n");
    for project in project_vec.iter() {
        println!("\t{}.\tName: {}", &project.count, cyan.apply_to(&project.name));
        println!("\t\tId: {}", cyan.apply_to(&project.id));
        println!("\t\tShared? {}\n", cyan.apply_to(&project.shared));
    }

    // Display the controls
    println!("Controls:\n\t`{}#` -> Display project contents (e.g. P3).\t`{}` -> Create new project.", cyan.apply_to(&"P"), cyan.apply_to(&"C"));
    println!("\t`{}#` -> Delete project.\t\t\t\t`{}#` -> Update project.\n", cyan.apply_to(&"D"), cyan.apply_to(&"U"));

    // Wait for the user to make an action
    let action: String = Input::new()
        .with_prompt("Action")
        .interact()
        .unwrap();

    // Process the action
    if action.contains("P") && action.len() == 2 {
        let project_num = &action[1..].parse::<i32>().unwrap();
        for project in project_vec.iter() {
            if &project.count == project_num {
                sect_and_task_overview(&project.name, &token_input, &project.id);
            }
        }
        project_overview(token_input.to_string())

    } else if action.contains("C") && action.len() == 1 {
        let new_name: String = Input::new()
            .with_prompt("\nNew project name")
            .interact()
            .unwrap();
        let json_name = json!({"name": &new_name}).to_string();
        Runtime::new().expect("Could not create new project.")
            .block_on(coto::new_project(&token_input, json_name))
            .unwrap();
        project_overview(token_input);

    } else if action.contains("D") && action.len() == 2 {
        let verification: String = Input::new()
            .with_prompt("\nAre you certain? y/n")
            .interact()
            .unwrap();
        let project_num = &action[1..].parse::<i32>().unwrap();
        for project in project_vec.iter() {
            if &project.count == project_num {
                match &verification.to_lowercase()[..] {
                    "y" => {
                    Runtime::new().expect("Could not delete project.")
                        .block_on(coto::delete_project(&token_input, &project.id))
                        .unwrap();
                    project_overview(token_input.to_string()); },
                    "n" => { project_overview(token_input.to_string()); },
                    _ => { println!("That isn't one of the projects");
                    project_overview(token_input.to_string()); },
                }
            }
        }

    } else if action.contains("U") && action.len() == 2 {
        let project_num = &action[1..].parse::<i32>().unwrap();
        for project in project_vec.iter() {
            if &project.count == project_num {
                let new_name: String = Input::new()
                    .with_prompt("New project name")
                    .interact()
                    .unwrap();
                let json_name = json!({"name": &new_name});
                Runtime::new().expect("Could not update project.")
                    .block_on(coto::update_project(&token_input, &project.id, json_name.to_string()))
                    .unwrap();
                project_overview(token_input.to_string());
            }
        }

    } else {
        println!("That was not one of the options.");

    }

}

// ##### Display Sections and Tasks #####
fn sect_and_task_overview(project_name: &String, token: &String, id: &String) {
    // Sections
    let section_call = Runtime::new().expect("Could not query sections.")
        .block_on(coto::get_project_sections(token, id))
        .unwrap();
    let section_count = section_call.matches("}").count();
    let json_sections: Value = serde_json::from_str(&section_call).unwrap();

    let mut section_vec: Vec<Section> = Vec::new();
    for s in 0..section_count {
        section_vec.push(Section {
            id: json_sections[s]["id"].to_string(),
            project_id: json_sections[s]["project_id"].to_string(),
            order: json_sections[s]["order"].to_string(),
            name: json_sections[s]["name"].to_string(),
        });
    }

    // Tasks
    let task_call = Runtime::new().expect("Could not query tasks.")
        .block_on(coto::get_all_tasks(token))
        .unwrap();
    let task_count = task_call.matches("}").count();
    let json_tasks: Value = serde_json::from_str(&task_call).unwrap();

    let mut task_vec: Vec<Task> = Vec::new();
    let mut task_counter: i32 = 1;
    for t in 0..task_count {
        task_vec.push(Task {
            id: json_tasks[t]["id"].to_string(),
            section_id: json_tasks[t]["section_id"].to_string(),
            order: task_counter.to_string(),
            content: json_tasks[t]["content"].to_string(),
            priority: json_tasks[t]["priority"].to_string(),
            created: json_tasks[t]["created"].to_string(),
            due: json_tasks[t]["due"]["date"].to_string(),
        });
        task_counter += 1;
    }

    // Display the overview
    let cyan = Style::new().cyan();
    let term = Term::stdout();
    term.clear_screen();
    println!("\nPress `Ctrl + C` to end session.");
    println!("\n\nProject - {}:", cyan.apply_to(&project_name));
    for section in section_vec.iter() {
        println!("\n\tSection {} - {}:", cyan.apply_to(&section.order), cyan.apply_to(&section.name));
        for task in task_vec.iter() {
            if task.section_id == section.id {
                println!("\n\t\tTask {} - Name: {}.", cyan.apply_to(&task.order), cyan.apply_to(&task.content));
                println!("\t\t\t Due date: {}.", cyan.apply_to(&task.due));
                println!("\t\t\t Priority: {}.", cyan.apply_to(&task.priority));
                println!("\t\t\t Created: {}.", cyan.apply_to(&task.created));
            }
        }
    }
    for task in task_vec.iter() {
        if task.section_id == "0".to_string() {
                println!("\n\tTask {} - Name: {}.", cyan.apply_to(&task.order), cyan.apply_to(&task.content));
                println!("\t\t Due date: {}.", cyan.apply_to(&task.due));
                println!("\t\t Priority: {}.", cyan.apply_to(&task.priority));
                println!("\t\t Created: {}.", cyan.apply_to(&task.created));
        }

    }

    // Display the controls
    println!("Controls:\n\t`{}#` -> Complete task (e.g. T2).\t`{}#` -> Create new task.", cyan.apply_to(&"T"), cyan.apply_to(&"C"));
    println!("\t`{}#` -> Delete task.\t\t\t`{}#` -> Update task.", cyan.apply_to(&"D"), cyan.apply_to(&"U"));
    println!("\t`{}` -> Go back to projects.\n", cyan.apply_to(&"B"));

    // Wait for the user to make an action
    let action: String = Input::new()
        .with_prompt("Action")
        .interact()
        .unwrap();

    if action.contains("T") && action.len() == 2 {
        let task_num = &action[1..].parse::<i32>().unwrap();
        for task in task_vec.iter() {
            if task_num.to_string() == task.order {
                Runtime::new().expect("Could not delete task.")
                    .block_on(coto::close_task(token, &task.id))
                    .unwrap();
            }
        }
        sect_and_task_overview(project_name, token, id);

    } else if action.contains("C") && action.len() <= 4 {
        let section_str = &action[1..];
        let mut section_num: i64 = 0;
        let content: String = Input::new()
            .with_prompt("\nTask content")
            .interact()
            .unwrap();
        let priority: String = Input::new()
            .with_prompt("Priority (lowest = 1, highest = 4)")
            .interact()
            .unwrap();
        let due: String = Input::new()
            .allow_empty(true)
            .with_prompt("Due date (YYYY-MM-DD)")
            .interact()
            .unwrap();

        if section_str.is_empty() == false {
            section_num = section_str.parse::<i64>().unwrap();
            for section in section_vec.iter() {
                if section_num == section.order.parse::<i64>().unwrap() {
                    let json_data = json!({"content": content,
                        "project_id": id.parse::<i64>().unwrap(),
                        "section_id": section.id.parse::<i64>().unwrap(),
                        "order": 5,
                        "priority": priority.parse::<i32>().unwrap(),
                        "due_date": due});
                    Runtime::new().expect("Could not create new task.")
                        .block_on(coto::new_task(token, json_data.to_string()))
                        .unwrap();
                }
            }
        } else {
            let json_data = json!({"content": content,
                "project_id": id.parse::<i64>().unwrap(),
                "section_id": 0,
                "order": 5,
                "priority": priority.parse::<i32>().unwrap(),
                "due_date": due});
            Runtime::new().expect("Could not create new task.")
                .block_on(coto::new_task(token, json_data.to_string()))
                .unwrap();
        }
        sect_and_task_overview(project_name, token, id);

    } else if action.contains("D") && action.len() == 2 {
        let verification: String = Input::new()
            .with_prompt("\nAre you certain? y/n")
            .interact()
            .unwrap();

        let task_num = &action[1..].parse::<i32>().unwrap();
        for task in task_vec.iter() {
            if task_num.to_string() == task.order {
                match &verification.to_lowercase()[..] {
                    "y" => {
                        Runtime::new().expect("Could not delete task.")
                            .block_on(coto::delete_task(token, &task.id))
                            .unwrap();
                    },
                    "n" => { sect_and_task_overview(project_name, token, id); },
                    _ => { println!("That was not an option.") },
                }
            }
        }
        sect_and_task_overview(project_name, token, id);

    } else if action.contains("U") && action.len() == 2 {
        let task_num = &action[1..].parse::<i32>().unwrap();
        for task in task_vec.iter() {
            if task_num.to_string() == task.order {
                let content: String = Input::new()
                    .with_prompt("\nTask content")
                    .with_initial_text(&task.content.replace("\"", ""))
                    .interact()
                    .unwrap();
                let priority: String = Input::new()
                    .with_prompt("Priority (lowest = 1, highest = 4)")
                    .with_initial_text(&task.priority.replace("\"", ""))
                    .interact()
                    .unwrap();
                let mut due: String = Input::new()
                    .allow_empty(true)
                    .with_prompt("Due date (YYYY-MM-DD)")
                    .with_initial_text(&task.due.replace("\"", ""))
                    .interact()
                    .unwrap();
                if &due[..] == "null" {
                    due = String::new();
                }
                let json_data = json!({"content": content,
                    "priority": priority.parse::<i32>().unwrap(),
                    "due_date": due});
                let res = Runtime::new().expect("Could not create new task.")
                    .block_on(coto::update_task(token, &task.id, json_data.to_string()))
                    .unwrap();
            }
        }
        sect_and_task_overview(project_name, token, id);
    } else if action.contains("B") && action.len() == 1 {
        project_overview(token.to_string());
    } else {
        println!("That was not one of the options.");
    }

}

