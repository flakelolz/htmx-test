use askama_axum::IntoResponse as AskamaIntoResponse;
use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use sqlx::SqlitePool;

use crate::{
    askama::{Index, PeopleTable, PeopleTableRow},
    database::{models::people::CreatePerson, people},
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/people", get(get_all_people).post(create_person))
}

async fn index() -> impl AskamaIntoResponse {
    Index {}
}

async fn create_person(
    Extension(pool): Extension<SqlitePool>,
    Json(person): Json<CreatePerson>,
) -> Result<impl AskamaIntoResponse, StatusCode> {
    match people::create_person(&pool, person).await {
        Ok(person) => Ok(PeopleTableRow { person }),
        Err(e) => {
            println!("create_person error: {e}");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

async fn get_all_people(
    Extension(pool): Extension<SqlitePool>,
) -> Result<impl AskamaIntoResponse, StatusCode> {
    match people::get_all(&pool).await {
        Ok(people) => Ok(PeopleTable { people }),
        Err(e) => {
            println!("get_all_people error: {e}");
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
