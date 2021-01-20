mod query;

use lambda_runtime::{error::HandlerError, lambda, Context};
use query::query;
use rocksdb::{Options, DB};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::error::Error;

#[derive(Deserialize)]
struct CustomEvent {
    key: String,
}

#[derive(Serialize)]
struct CustomOutput {
    object: Option<JsonValue>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options::default();
    let db = DB::open_for_read_only(&opts, "/mnt/db", false)?;

    lambda!(
        |e: CustomEvent, _c: Context| -> Result<CustomOutput, HandlerError> {
            Ok(CustomOutput {
                object: Some(query(&db, e.key).unwrap()),
            })
        }
    );

    Ok(())
}
