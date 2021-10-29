use std::error::Error;
use std::io;
use std::process;

use chrono::prelude::{DateTime, Utc};
use serde::Deserialize;

mod my_date_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(
        date: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
struct Record {
    //#[serde(rename(deserialize = "FIPS"))]
    //fips: String,
    //#[serde(rename(deserialize = "Admin2"))]
    //admin2: String,
    #[serde(rename(deserialize = "Province_State"))]
    province_state: String,
    #[serde(rename(deserialize = "Country_Region"))]
    country_region: String,
    #[serde(rename(deserialize = "Last_Update"), with = "my_date_format")]
    last_update:  DateTime<Utc>,
    //Lat: String,
    //Long_: String,
    #[serde(rename(deserialize = "Confirmed"))]
    confirmed: Option<u64>,
    #[serde(rename(deserialize = "Deaths"))]
    deaths: Option<u64>,
    #[serde(rename(deserialize = "Recovered"))]
    recovered: Option<u64>,
    #[serde(rename(deserialize = "Active"))]
    active: Option<u64>,
    #[serde(rename(deserialize = "Combined_Key"))]
    combined_key: String,
    // #[serde(rename(deserialize = "Incident_Rate"))]
    // Incident_Rate: Option<f64>,
    // #[serde(rename(deserialize = "Case_Fatality_Ratio"))]
    // Case_Fatality_Ratio: Option<f64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
