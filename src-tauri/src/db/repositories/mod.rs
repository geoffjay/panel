use crate::db::ConnectionType;

pub mod dashboard;
pub mod project;
pub mod variable;

pub use project::ProjectRepository;

pub trait Repository<T, C, U> {
    fn create(connection: &mut ConnectionType, item: C) -> Result<T, diesel::result::Error>;
    fn update(
        connection: &mut ConnectionType,
        id: i32,
        item: U,
    ) -> Result<T, diesel::result::Error>;
    fn delete(connection: &mut ConnectionType, id: i32) -> Result<(), diesel::result::Error>;
}
