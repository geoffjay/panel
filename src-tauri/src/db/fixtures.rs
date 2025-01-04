use serde::Deserialize;
use std::fs;

use crate::db::ConnectionType;
use crate::db::models::Dashboard;

#[derive(Deserialize)]
struct DashboardFixture {
    #[serde(flatten)]
    dashboard: Dashboard,
    components: Vec<ComponentFixture>,
}

#[derive(Deserialize)]
struct ProjectFixture {
    title: String,
    subtitle: String,
    variables: Vec<VariableFixture>,
    dashboard: DashboardFixture,
    logging: LoggingFixture,
    acquisition: AcquisitionFixture,
    connection: ConnectionFixture,
}

pub fn load_fixture(conn: &ConnectionType, fixture: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(fixture)?;
    let project: ProjectFixture = serde_yaml::from_str(&contents)?;

    diesel::insert_into(projects::table)
        .values(&NewProject {
            title: project.title,
            subtitle: project.subtitle,
        })
        .execute(conn)?;

    // FIXME: variables should live on projects but they're currectly on dashboards
    for variable in project.variables {
        diesel::insert_into(variables::table)
            .values(&NewVariable {
                ref_id: variable.ref_id,
                default: variable.default,
                value: variable.value,
                dashboard_id: 1,
            })
            .execute(conn)?;
    }

    Ok(())
}
