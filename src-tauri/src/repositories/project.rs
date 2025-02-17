use diesel::prelude::*;
// use panel_macro::Readable;

use crate::db::models::{CreateProject, Project, UpdateProject};
use crate::db::ConnectionType;
use crate::repositories::Repository;

// #[derive(Readable)]
// #[entity = "Project"]
// #[table = "projects"]
pub struct ProjectRepository;

impl Repository<Project, CreateProject, UpdateProject> for ProjectRepository {
    fn all(connection: &mut ConnectionType) -> Result<Vec<Project>, diesel::result::Error> {
        use crate::schema::projects::dsl;

        dsl::projects.load(connection)
    }

    fn find(connection: &mut ConnectionType, id: i32) -> Result<Project, diesel::result::Error> {
        use crate::schema::projects::dsl;

        dsl::projects.find(id).first(connection)
    }

    fn create(
        connection: &mut ConnectionType,
        project: CreateProject,
    ) -> Result<Project, diesel::result::Error> {
        use crate::schema::projects::dsl;

        diesel::insert_into(dsl::projects)
            .values(&project)
            .returning(Project::as_returning())
            .get_result(connection)
    }

    fn update(
        connection: &mut ConnectionType,
        id: i32,
        item: UpdateProject,
    ) -> Result<Project, diesel::result::Error> {
        use crate::schema::projects::dsl;

        diesel::update(dsl::projects.find(id))
            .set(&item)
            .returning(Project::as_returning())
            .get_result(connection)
    }

    fn delete(connection: &mut ConnectionType, id: i32) -> Result<(), diesel::result::Error> {
        use crate::schema::projects::dsl;

        diesel::delete(dsl::projects.find(id)).execute(connection)?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utils::test_helpers::setup_test_environment;

//     #[test]
//     fn test_find() {
//         let context = setup_test_environment();
//         let project = ProjectRepository::find(&mut context.connection.lock().unwrap(), 1).expect("find failed");

//         assert_eq!(project.id, 1);
//         assert_eq!(project.title, "Test");
//         assert_eq!(project.subtitle, "Test");
//         assert_eq!(project.description, "Test");
//     }
// }
