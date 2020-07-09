use reqwest::{header, Client, Url};
use uuid::Uuid;

// ########## Projects ##########

pub async fn get_all_projects(token: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get("https://api.todoist.com/rest/v1/projects")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_project(token: &String, id: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(Url::parse(&format!("https://api.todoist.com/rest/v1/projects/{}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}


pub async fn new_project(token: &String, json_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", uuid.to_string().parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .post("https://api.todoist.com/rest/v1/projects")
        .headers(headers)
        .body(json_name)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn update_project(token: &String, id: &String, json_data: String) -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", uuid.to_string().parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .post(Url::parse(&format!("https://api.todoist.com/rest/v1/projects/{}", id)).unwrap())
        .headers(headers)
        .body(json_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn delete_project(token: &String, id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .delete(Url::parse(&format!("https://api.todoist.com/rest/v1/projects/{}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

// ########## Collaborators ##########

pub async fn get_collaborators(token: &String, id: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(Url::parse(&format!("https://api.todoist.com/rest/v1/projects/{}/collaborators", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

// ########## Sections ##########

pub async fn get_all_sections(token: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get("https://api.todoist.com/rest/v1/sections")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_project_sections(token: &String, id: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(Url::parse(&format!("https://api.todoist.com/rest/v1/sections?project_id={}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_section(token: &String, id: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(Url::parse(&format!("https://api.todoist.com/rest/v1/sections/{}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn new_section(token: &String, json_name_and_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .post("https://api.todoist.com/rest/v1/sections")
        .headers(headers)
        .body(json_name_and_id)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}


pub async fn update_section(token: &String, id: &String, json_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .post(Url::parse(&format!("https://api.todoist.com/rest/v1/sections/{}", id)).unwrap())
        .headers(headers)
        .body(json_name)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn delete_section(token: &String, id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .delete(Url::parse(&format!("https://api.todoist.com/rest/v1/sections/{}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

// ########## Tasks ##########

pub async fn get_all_tasks(token: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get("https://api.todoist.com/rest/v1/tasks")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_task(token: &String, id: &String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(Url::parse(&format!("https://api.todoist.com/rest/v1/tasks/{}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn new_task(token: &String, json_data: String) -> Result<(), Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", uuid.to_string().parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .post("https://api.todoist.com/rest/v1/tasks")
        .headers(headers)
        .body(json_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn update_task(token: &String, id: &String, json_data: String) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let uuid = Uuid::new_v4();
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", uuid.to_string().parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .post(Url::parse(&format!("https://api.todoist.com/rest/v1/tasks/{}", id)).unwrap())
        .headers(headers)
        .body(json_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn close_task(token: &String, id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .post(Url::parse(&format!("https://api.todoist.com/rest/v1/tasks/{}/close", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn reopen_task(token: &String, id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .post(Url::parse(&format!("https://api.todoist.com/rest/v1/tasks/{}/reopen", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

pub async fn delete_task(token: &String, id: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let _res = Client::new()
        .delete(Url::parse(&format!("https://api.todoist.com/rest/v1/tasks/{}", id)).unwrap())
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(())
}

// ########## Comments ##########

// ToDo

// ########## Labels ##########

// ToDo

