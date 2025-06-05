#[cfg(test)]
mod tests {
    use chrono::{Utc};
    use uuid::Uuid;
    use serde_json::{to_string, from_str};

    use data_model::entity::Entity;
    use data_model::location::Location;

    #[test]
    fn entity_json_roundtrip() {
        let e1 = Entity {
            id: Uuid::new_v4(),
            name: "Raptor-01".into(),
            position: Location { lat: 47.67, lon: -122.12 },
            tasks: vec![],
            updated_at: Utc::now(),
        };

        let json = to_string(&e1).unwrap();
        let e2: Entity = from_str(&json).unwrap();
        assert_eq!(e1, e2);
    }
}
