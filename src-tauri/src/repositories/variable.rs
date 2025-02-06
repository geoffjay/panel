// use diesel::prelude::*;

// use crate::db::models::{NewVariable, Variable};

// pub fn find_variable(
//     connection: &mut SqliteConnection,
//     id: i32,
// ) -> Result<Variable, diesel::result::Error> {
//     use crate::schema::variables::dsl;

//     dsl::variables.find(id).first(connection)
// }

// pub fn find_variables(
//     connection: &mut SqliteConnection,
// ) -> Result<Vec<Variable>, diesel::result::Error> {
//     use crate::schema::variables::dsl;

//     dsl::variables.load::<Variable>(connection)
// }

// pub fn create_variable(
//     connection: &mut SqliteConnection,
//     variable: NewVariable,
// ) -> Result<Variable, diesel::result::Error> {
//     use crate::schema::variables::dsl;

//     diesel::insert_into(dsl::variables)
//         .values(&variable)
//         .returning(Variable::as_returning())
//         .get_result(connection)
// }

// pub fn update_variable(
//     connection: &mut SqliteConnection,
//     variable: Variable,
// ) -> Result<Variable, diesel::result::Error> {
//     use crate::schema::variables::dsl;

//     diesel::update(dsl::variables.find(variable.id))
//         .set(&variable)
//         .returning(Variable::as_returning())
//         .get_result(connection)
// }

// pub fn delete_variable(
//     connection: &mut SqliteConnection,
//     id: i32,
// ) -> Result<(), diesel::result::Error> {
//     use crate::schema::variables::dsl;

//     diesel::delete(dsl::variables.find(id)).execute(connection)?;
//     Ok(())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use diesel_migrations::MigrationHarness;

//     use crate::MIGRATIONS;
//     use crate::db::models::NewVariableValue;

//     struct TestContext {
//         connection: SqliteConnection,
//     }

//     fn setup_test_environment() -> TestContext {
//         let database_url = ":memory:".to_string();
//         let mut connection = SqliteConnection::establish(&database_url)
//             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

//         connection.run_pending_migrations(MIGRATIONS).unwrap();

//         TestContext { connection }
//     }

//     #[test]
//     fn test_find_variable() {
//         let mut context = setup_test_environment();
//         let variable = create_variable(
//             &mut context.connection,
//             NewVariable {
//                 ref_id: Some("test".to_string()),
//                 default: None,
//                 value: NewVariableValue {
//                     var_type: "string".to_string(),
//                     inner_value: "test".to_string(),
//                 },
//                 dashboard_id: 1,
//             },
//         )
//         .unwrap();

//         let found_variable = find_variable(&mut context.connection, variable.id).unwrap();

//         assert_eq!(variable, found_variable);
//     }
// }
