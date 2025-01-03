#[allow(unused_imports)]
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use std::sync::{Arc, Mutex};

use crate::MIGRATIONS;

pub struct TestContext {
    pub connection: Arc<Mutex<SqliteConnection>>,
}

pub fn setup_test_environment() -> TestContext {
    let database_url = ":memory:".to_string();
    let mut connection = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection.run_pending_migrations(MIGRATIONS).unwrap();

    TestContext {
        connection: Arc::new(Mutex::new(connection)),
    }
}
