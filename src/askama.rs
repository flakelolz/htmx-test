use askama::Template;

use crate::database::models::people::Person;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "components/people_table.html")]
pub struct PeopleTable {
    pub people: Vec<Person>,
}

#[derive(Template)]
#[template(path = "components/people_form.html")]
pub struct PeopleForm;

#[derive(Template)]
#[template(path = "partials/people_tr.html")]
pub struct PeopleTableRow {
    pub person: Person,
}
