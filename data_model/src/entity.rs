use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use uuid::Uuid;
use serde_json::to_string;
use arrow::{array::{Float64Array, StringArray, LargeStringArray}, 
            record_batch::RecordBatch,
            error::{ArrowError, Result},
            datatypes::{DataType, Field, Schema}};
use std::sync::Arc;

// crate is relative directory
use crate::{location::Location, task::Task};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub position: Location,
    pub tasks: Vec<Task>,
    pub updated_at: DateTime<Utc>
}

impl Entity {
    pub fn arrow_schema() -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::LargeUtf8, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("x", DataType::Float64, false),
            Field::new("y", DataType::Float64, false),
            Field::new("tasks", DataType::Utf8, true)
        ]))
    }

    // used for snapshots, sooo bulk data (endpoint will be getSnapshot)
    pub fn to_record_batch(entities: &[Self]) -> Result<RecordBatch> {
        let ids = LargeStringArray::from_iter(entities.iter().map(|e| Some(e.id.to_string())));
        let names = StringArray::from_iter(entities.iter().map(|e| Some(e.name.as_str())));
        let xs    = Float64Array::from_iter(entities.iter().map(|e| Some(e.position.lat)));
        let ys    = Float64Array::from_iter(entities.iter().map(|e| Some(e.position.lon)));
        let tasks = StringArray::from_iter(
            entities
                .iter()
                .map(|e| Some(to_string(&e.tasks).unwrap()))
        );

        RecordBatch::try_new(
            Self::arrow_schema(),
            vec![
                Arc::new(ids),
                Arc::new(names),
                Arc::new(xs),
                Arc::new(ys),
                Arc::new(tasks),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;                 // pulls in Entity / Location from the same crate

    use chrono::Utc;
    use serde_json::{from_str, to_string};
    use uuid::Uuid;

    #[test]
    fn entity_json_roundtrip() {
        let e1 = Entity {
            id: Uuid::new_v4(),
            name: "Raptor-01".into(),
            position: Location { lat: 47.67, lon: -122.12 },
            tasks: vec![],
            updated_at: Utc::now(),
        };

        let batch = Entity::to_record_batch(&[e1.clone()]).unwrap();
        assert_eq!(batch.num_rows(), 1);

        let json = to_string(&e1).unwrap();
        let e2: Entity = from_str(&json).unwrap();
        assert_eq!(e1, e2);
    }
}
