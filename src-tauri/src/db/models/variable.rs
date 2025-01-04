use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    prelude::*,
    serialize::{self, Output, ToSql},
    sqlite::Sqlite,
};
use serde::{Deserialize, Serialize};

use crate::db::models::Dashboard;
use crate::db::ConnectionType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, FromSqlRow, AsExpression)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct JsonValue {
    #[serde(rename = "type")]
    pub var_type: String,
    pub value: JsonValueType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonValueType {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl FromSql<diesel::sql_types::Text, Sqlite> for JsonValue {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let str_value = <String as FromSql<diesel::sql_types::Text, Sqlite>>::from_sql(bytes)?;
        serde_json::from_str(&str_value).map_err(|e| e.into())
    }
}

impl ToSql<diesel::sql_types::Text, Sqlite> for JsonValue {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        let json_string = serde_json::to_string(self)?;
        out.set_value(json_string);
        Ok(serialize::IsNull::No)
    }
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    QueryableByName,
    Associations,
    Identifiable,
)]
#[diesel(table_name=crate::schema::variables, primary_key(id), belongs_to(Dashboard, foreign_key=dashboard_id))]
pub struct Variable {
    pub id: i32,
    pub ref_id: Option<String>,
    pub default: Option<JsonValue>,
    pub value: JsonValue,
    pub dashboard_id: i32,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.ref_id == other.ref_id
            && self.default == other.default
            && self.value == other.value
            && self.dashboard_id == other.dashboard_id
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name=crate::schema::variables)]
pub struct CreateVariable {
    pub ref_id: Option<String>,
    pub default: Option<JsonValue>,
    pub value: JsonValue,
    pub dashboard_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq, Default)]
#[diesel(table_name=crate::schema::variables)]
pub struct UpdateVariable {
    pub ref_id: Option<Option<String>>,
    pub default: Option<Option<JsonValue>>,
    pub value: Option<JsonValue>,
    pub dashboard_id: Option<i32>,
}

impl Variable {
    pub fn new(
        dashboard_id: i32,
        ref_id: Option<String>,
        default: Option<JsonValue>,
        value: JsonValue,
    ) -> Self {
        Self {
            id: 0,
            ref_id,
            default,
            value,
            dashboard_id,
        }
    }

    pub fn find(db: &mut ConnectionType, id: i32) -> QueryResult<Self> {
        use crate::schema::variables::dsl;

        dsl::variables.find(id).first(db)
    }

    pub fn find_all(db: &mut ConnectionType) -> QueryResult<Vec<Self>> {
        use crate::schema::variables::dsl;

        dsl::variables.load::<Self>(db)
    }

    pub fn create(db: &mut ConnectionType, item: &CreateVariable) -> QueryResult<Self> {
        use crate::schema::variables::dsl;

        diesel::insert_into(dsl::variables)
            .values(item)
            .get_result::<Self>(db)
    }

    pub fn update(db: &mut ConnectionType, id: i32, item: &UpdateVariable) -> QueryResult<Self> {
        use crate::schema::variables::dsl;

        diesel::update(dsl::variables.find(id))
            .set(item)
            .get_result(db)
    }

    pub fn delete(db: &mut ConnectionType, id: i32) -> QueryResult<usize> {
        use crate::schema::variables::dsl;

        diesel::delete(dsl::variables.find(id)).execute(db)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::db::models::dashboard::{CreateDashboard, Dashboard};
    use crate::utils::test_helpers::setup_test_environment;

    #[test]
    fn test_variable_value_serialization() {
        let value = serde_json::json!({
            "type": "string",
            "value": "test",
        });

        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();

        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_variable_equality() {
        let var1 = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: JsonValue {
                var_type: "string".to_string(),
                value: JsonValueType::String("test".to_string()),
            },
            dashboard_id: 0,
        };

        let var2 = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: JsonValue {
                var_type: "string".to_string(),
                value: JsonValueType::String("test".to_string()),
            },
            dashboard_id: 0,
        };

        let var3 = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: JsonValue {
                var_type: "string".to_string(),
                value: JsonValueType::String("different".to_string()),
            },
            dashboard_id: 0,
        };

        assert_eq!(var1, var2);
        assert_ne!(var1, var3);
    }

    #[test]
    fn test_variable_serde() {
        let var = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: JsonValue {
                var_type: "string".to_string(),
                value: JsonValueType::String("test".to_string()),
            },
            dashboard_id: 0,
        };

        let serialized = serde_json::to_string(&var).unwrap();
        let deserialized: Variable = serde_json::from_str(&serialized).unwrap();

        assert_eq!(var, deserialized);

        let json_value: serde_json::Value = serde_json::from_str(&serialized).unwrap();
        assert_eq!(json_value["id"], 1);
        assert_eq!(json_value["ref_id"], "test");
        assert_eq!(json_value["value"]["type"], "string");
        assert_eq!(json_value["value"]["value"], "test");
    }

    #[test]
    fn test_json_value_serialization() {
        let value = JsonValue {
            var_type: "string".to_string(),
            value: JsonValueType::String("test".to_string()),
        };

        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: JsonValue = serde_json::from_str(&serialized).unwrap();

        assert_eq!(value, deserialized);

        // Test other types
        let int_value = JsonValue {
            var_type: "int".to_string(),
            value: JsonValueType::Integer(42),
        };
        let float_value = JsonValue {
            var_type: "float".to_string(),
            value: JsonValueType::Float(std::f64::consts::PI),
        };
        let bool_value = JsonValue {
            var_type: "bool".to_string(),
            value: JsonValueType::Boolean(true),
        };

        assert_eq!(
            serde_json::from_str::<JsonValue>(&serde_json::to_string(&int_value).unwrap()).unwrap(),
            int_value
        );
        assert_eq!(
            serde_json::from_str::<JsonValue>(&serde_json::to_string(&float_value).unwrap())
                .unwrap(),
            float_value
        );
        assert_eq!(
            serde_json::from_str::<JsonValue>(&serde_json::to_string(&bool_value).unwrap())
                .unwrap(),
            bool_value
        );
    }

    #[test]
    fn test_variable_find() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let variable = Variable::create(
            &mut context.connection.lock().unwrap(),
            &CreateVariable {
                ref_id: Some("test".to_string()),
                default: None,
                value: JsonValue {
                    var_type: "string".to_string(),
                    value: JsonValueType::String("test".to_string()),
                },
                dashboard_id: dashboard.id,
            },
        )
        .unwrap();

        let found_variable =
            Variable::find(&mut context.connection.lock().unwrap(), variable.id).unwrap();
        assert_eq!(found_variable, variable);

        let found_all_variables =
            Variable::find_all(&mut context.connection.lock().unwrap()).unwrap();
        assert_eq!(found_all_variables.len(), 1);
        assert_eq!(found_all_variables[0], variable);
    }

    #[test]
    fn test_variable_create() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let variable = Variable::create(
            &mut context.connection.lock().unwrap(),
            &CreateVariable {
                ref_id: Some("test".to_string()),
                default: None,
                value: JsonValue {
                    var_type: "string".to_string(),
                    value: JsonValueType::String("test".to_string()),
                },
                dashboard_id: dashboard.id,
            },
        )
        .unwrap();

        assert_eq!(variable.dashboard_id, dashboard.id);
        assert_eq!(variable.ref_id, Some("test".to_string()));
        assert_eq!(variable.default, None);
        assert_eq!(
            variable.value,
            JsonValue {
                var_type: "string".to_string(),
                value: JsonValueType::String("test".to_string()),
            }
        );
    }

    #[test]
    fn test_variable_update() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let variable = Variable::create(
            &mut context.connection.lock().unwrap(),
            &CreateVariable {
                ref_id: Some("test".to_string()),
                default: None,
                value: JsonValue {
                    var_type: "string".to_string(),
                    value: JsonValueType::String("test".to_string()),
                },
                dashboard_id: dashboard.id,
            },
        )
        .unwrap();

        let updated_variable = Variable::update(
            &mut context.connection.lock().unwrap(),
            variable.id,
            &UpdateVariable {
                ref_id: Some(Some("test".to_string())),
                default: Some(None),
                value: Some(JsonValue {
                    var_type: "string".to_string(),
                    value: JsonValueType::String("test".to_string()),
                }),
                dashboard_id: Some(dashboard.id),
            },
        )
        .unwrap();

        assert_eq!(updated_variable.id, variable.id);
        assert_eq!(updated_variable.ref_id, Some("test".to_string()));
        assert_eq!(updated_variable.default, None);
        assert_eq!(
            updated_variable.value,
            JsonValue {
                var_type: "string".to_string(),
                value: JsonValueType::String("test".to_string()),
            }
        );
        assert_eq!(updated_variable.dashboard_id, dashboard.id);
    }

    #[test]
    fn test_variable_delete() {
        let context = setup_test_environment();
        let dashboard = Dashboard::create(
            &mut context.connection.lock().unwrap(),
            &CreateDashboard {
                title: "Test Dashboard".to_string(),
                subtitle: "This is a test dashboard".to_string(),
                description: "This is a test dashboard".to_string(),
            },
        )
        .unwrap();

        let variable = Variable::create(
            &mut context.connection.lock().unwrap(),
            &CreateVariable {
                ref_id: Some("test".to_string()),
                default: None,
                value: JsonValue {
                    var_type: "string".to_string(),
                    value: JsonValueType::String("test".to_string()),
                },
                dashboard_id: dashboard.id,
            },
        )
        .unwrap();

        let deleted =
            Variable::delete(&mut context.connection.lock().unwrap(), variable.id).unwrap();
        assert_eq!(deleted, 1);

        let found_variable =
            Variable::find(&mut context.connection.lock().unwrap(), variable.id).unwrap_err();
        assert!(matches!(found_variable, diesel::result::Error::NotFound));
    }
}
