use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{
    de::{Deserializer, Visitor},
    Deserialize, Serialize,
};

/// The snowflake struct.
#[derive(Clone, Debug)]
pub struct Snowflake {
    inner: u64,
}

impl From<u64> for Snowflake {
    fn from(id: u64) -> Self {
        Snowflake { inner: id }
    }
}

impl From<&str> for Snowflake {
    fn from(id: &str) -> Self {
        Snowflake {
            inner: id.parse::<u64>().unwrap(),
        }
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Snowflake, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.inner)
    }
}

struct SnowflakeVisitor;

impl<'de> Visitor<'de> for SnowflakeVisitor {
    type Value = Snowflake;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a snowflake")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Snowflake { inner: v })
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Snowflake {
            inner: v.parse::<u64>().unwrap(),
        })
    }
}

pub const DISCORD_EPOCH: u64 = 1420070400000;

impl Snowflake {
    /// Create a new snowflake for the given `DateTime` instance.
    pub fn new(dt: DateTime<Utc>) -> Snowflake {
        let delta = (dt.timestamp_millis() as u64) - DISCORD_EPOCH;
        Snowflake {
            inner: (delta << 22),
        }
    }

    /// Create a new snowflake representing the current time instant.
    pub fn now() -> Snowflake {
        Snowflake::new(Utc::now())
    }

    /// Returns the timestamp of the snowflake.
    pub fn timestamp(&self) -> DateTime<Utc> {
        let millis = (self.inner >> 22) + DISCORD_EPOCH;
        let secs = (millis / 1000) as i64;
        let nanos = ((millis % 1000) * 1_000_000) as u32;
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, nanos), Utc)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serde::Deserialize;

    use super::Snowflake;

    #[test]
    fn test_snowflake_decode() {
        let snowflake: Snowflake = "940565712559304734".into();
        let timestamp = Utc.ymd(2022, 02, 8).and_hms_milli(11, 12, 20, 740);
        assert_eq!(snowflake.timestamp(), timestamp);
    }

    #[test]
    fn test_snowflake_deserialize() {
        let json = r#"{"timestamp":"940573225321132082"}"#;
        #[derive(Deserialize)]
        struct MyData {
            timestamp: Snowflake,
        }
        let data: MyData = serde_json::from_str(json).unwrap();
        assert_eq!(
            data.timestamp.timestamp(),
            Utc.ymd(2022, 02, 8).and_hms_milli(11, 42, 11, 922)
        );
    }

    #[test]
    fn test_snowflake_new() {
        let timestamp = Utc.ymd(2022, 02, 8).and_hms_milli(11, 42, 11, 922);
        let snowflake = Snowflake::new(timestamp);
        assert_eq!(snowflake.timestamp(), timestamp);
    }
}
