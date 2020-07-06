use reqwest::{header, Client};

pub async fn get_all_projects(token: String) -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn get_project(token: String, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(format!("https://api.todoist.com/rest/v1/projects/{}", id))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}


pub async fn new_project(token: String, json_name: String) -> Result<(), Box<dyn std::error::Error>> {
    // let uuid = ...
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", uuid.parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .post("https://api.todoist.com/rest/v1/projects")
        .headers(headers)
        .body(json_name)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn update_project(token: String, id: i32, json_data: String) -> Result<(), Box<dyn std::error::Error>> {
    // let uuid = ...
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("X-Request-Id", uuid.parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .post(format!("https://api.todoist.com/rest/v1/projects/{}", id))
        .headers(headers)
        .body(json_data)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn delete_project(token: String, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer ()", token).parse().unwrap());

    let res = Client::new()
        .delete(format!("https://api.todoist.com/rest/v1/projects/{}", id))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_collaborators(token: String, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(format!("https://api.todoist.com/rest/v1/projects/{}/collaborators", id))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_all_sections(token: String) -> Result<(), Box<dyn std::error::Error>> {
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

pub async fn get_project_sections(token: String, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(format!("https://api.todoist.com/rest/v1/sections?project_id={}", id))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn get_section(token:String, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get(format!("https://api.todoist.com/rest/v1/sections/{}", id))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub async fn new_sections(token: String, json_name_and_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .post("https://api.todoist.com/rest/v1/sections")
        .headers(headers)
        .body(json_name)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}


pub async fn update_section(token: String, id: 132, json_name:String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .post(format!("https://api.todoist.com/rest/v1/sections/{}", id))
        .headers(headers)
        .body(json_name)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

pub aync fn delete_section(token: String, id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .delete(format!("https://api.todoist.com/rest/v1/sections/{}", id))
        .headers(headers)
        .send()
        .await?
        .text()
        .await?

    Ok(res)
}

pub async fn get_all_tasks(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

    let res = Client::new()
        .get("https://api.todoist.com/rest/v1/tasks")
        .headers(headers)
        .send()
        .await?
        .text()
        .await?

    Ok(res)
}




