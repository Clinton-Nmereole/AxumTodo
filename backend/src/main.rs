use std::net::SocketAddr;
use axum::{Router, routing::{get, post}, extract::{State, Path}, Json, Form, response::Redirect};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/", get(list))
        .route("/create", post(create))
        .route("/delete/:id", get(delete))
        .route("/update", post(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let address  = SocketAddr::from(([0,0,0,0], 443));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    description: String,
    done: bool,
}

async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    //List all todos
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    //Create Todo
    sqlx::query!("INSERT INTO todos (title, description, done) VALUES (?,?,?)", todo.title, todo.description, todo.done)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173/"))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<Redirect> {
    //Create Todo
    sqlx::query!("DELETE FROM todos where id = ?", id)
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173/"))
}


async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<Redirect> {
    //Create Todo
    sqlx::query!("UPDATE todos SET title = ?, description = ?, done = ? WHERE id = ?", todo.title, todo.description, todo.done, todo.id )
        .execute(&pool)
        .await?;
    Ok(Redirect::to("http://localhost:5173/"))
}

