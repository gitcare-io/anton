use std::time::{Duration, SystemTime, UNIX_EPOCH};

// DMR - Daily Merge Rate

#[derive(Serialize, Deserialize, Debug)]
pub struct DMRProjection {
    id: String,
    aggregate_id: i64,
    aggregate_type: String,
    from: SystemTime,
    to: SystemTime,
    r#type: Option<String>,
    data: Option<DMRProjectionData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DMRProjectionData {
    target: f32,
    value: f32,
    index: f32,
}

pub struct DMRProjectionIdentity {
    aggregate_id: i64,
    aggregate_type: String,
    from: SystemTime,
    to: SystemTime,
}

impl DMRProjection {
    pub fn new(identity: DMRProjectionIdentity) -> Self {
        Self {
            id: format!(
                "{}_{}_{}_{}",
                identity.aggregate_id,
                identity.from.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                identity.to.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                identity.aggregate_type,
            ),
            aggregate_id: identity.aggregate_id,
            aggregate_type: identity.aggregate_type,
            from: identity.from,
            to: identity.to,
            r#type: None,
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_dmr_projection() {
        let dmr_projection = DMRProjection::new(DMRProjectionIdentity {
            aggregate_id: 10,
            aggregate_type: String::from("user"),
            from: UNIX_EPOCH + Duration::from_secs(1554076800),
            to: UNIX_EPOCH + Duration::from_secs(1556668800),
        });
        assert_eq!(
            String::from("10_1554076800_1556668800_user"),
            dmr_projection.id
        );
    }
}
