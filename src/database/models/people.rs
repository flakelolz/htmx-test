#![allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePerson {
    pub first_name: String,
    pub last_name: String,
    #[serde(deserialize_with = "deserialize_age")]
    pub age: i32,
}

fn deserialize_age<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

impl CreatePerson {
    pub fn new(first_name: String, last_name: String, age: i32) -> Self {
        Self {
            first_name,
            last_name,
            age,
        }
    }

    pub fn group() -> Vec<Self> {
        vec![
            CreatePerson::new("Anna".to_string(), "Williams".to_string(), 41),
            CreatePerson::new("Bryan".to_string(), "Fury".to_string(), 31),
            CreatePerson::new("Claudio".to_string(), "Serafino".to_string(), 27),
            CreatePerson::new("Eddy".to_string(), "Gordo".to_string(), 29),
            CreatePerson::new("Lili".to_string(), "Rochefort".to_string(), 16),
            CreatePerson::new("Lee".to_string(), "Chaolan".to_string(), 48),
            CreatePerson::new("Leroy".to_string(), "Smith".to_string(), 50),
            CreatePerson::new("Feng".to_string(), "Wei".to_string(), 26),
            CreatePerson::new("Heichachi".to_string(), "Mishima".to_string(), 75),
            CreatePerson::new("Jin".to_string(), "Kazama".to_string(), 21),
            CreatePerson::new("Victor".to_string(), "Chevalier".to_string(), 54),
            CreatePerson::new("Wang".to_string(), "Jinrei".to_string(), 103),
        ]
    }
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}, age: {}",
            self.first_name, self.last_name, self.age
        )
    }
}
