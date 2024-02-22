use anyhow::Result;
use sqlx::SqlitePool;

use super::models::people::{CreatePerson, Person};

pub async fn get_all(pool: &SqlitePool) -> Result<Vec<Person>> {
    let response = sqlx::query_as::<_, Person>("SELECT * FROM person")
        .fetch_all(pool)
        .await?;
    Ok(response)
}

pub async fn create_person(pool: &SqlitePool, person: CreatePerson) -> Result<Person> {
    let response = sqlx::query_as::<_, Person>(
        "INSERT INTO person (first_name, last_name, age) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(person.first_name)
    .bind(person.last_name)
    .bind(person.age)
    .fetch_one(pool)
    .await?;
    Ok(response)
}
