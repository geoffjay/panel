use diesel::prelude::*;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::db::models::Dashboard;

#[derive(Deserialize, Serialize, PartialEq, FromSqlRow, AsExpression, Debug)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct VariableValue {
    #[serde(rename = "type")]
    pub var_type: String,
    #[serde(rename = "value")]
    pub inner_value: String,
}

#[derive(Queryable, Selectable, AsChangeset, Identifiable, Associations, Debug)]
#[diesel(table_name = crate::schema::variables)]
#[diesel(belongs_to(Dashboard))]
// #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Variable {
    pub id: i32,
    pub ref_id: Option<String>,
    pub default: Option<VariableValue>,
    pub value: VariableValue,
    pub dashboard_id: i32,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.ref_id == other.ref_id
            && self.value == other.value
            && self.dashboard_id == other.dashboard_id
    }
}

impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Variable", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("ref_id", &self.ref_id)?;
        state.serialize_field("value", &self.value)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct VariableHelper {
            id: i32,
            ref_id: Option<String>,
            value: VariableValue,
        }

        let helper = VariableHelper::deserialize(deserializer)?;

        Ok(Variable {
            id: helper.id,
            ref_id: helper.ref_id,
            default: None,
            value: helper.value,
            dashboard_id: 0,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub struct NewVariableValue {
    pub var_type: String,
    pub inner_value: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[diesel(table_name = crate::schema::variables)]
pub struct NewVariable {
    pub ref_id: Option<String>,
    pub default: Option<NewVariableValue>,
    pub value: NewVariableValue,
    pub dashboard_id: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_value_serialization() {
        let value = VariableValue {
            var_type: "string".to_string(),
            inner_value: "test".to_string(),
        };

        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: VariableValue = serde_json::from_str(&serialized).unwrap();

        assert_eq!(value, deserialized);
    }

    #[test]
    fn test_variable_equality() {
        let var1 = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: VariableValue {
                var_type: "string".to_string(),
                inner_value: "test".to_string(),
            },
            dashboard_id: 0,
        };

        let var2 = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: VariableValue {
                var_type: "string".to_string(),
                inner_value: "test".to_string(),
            },
            dashboard_id: 0,
        };

        let var3 = Variable {
            id: 1,
            ref_id: Some("test".to_string()),
            default: None,
            value: VariableValue {
                var_type: "string".to_string(),
                inner_value: "different".to_string(),
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
            value: VariableValue {
                var_type: "string".to_string(),
                inner_value: "test".to_string(),
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
}
