use avro_rs::from_avro_datum;
use avro_rs::Schema;
use rocksdb::DB;
use serde_json::Value as JsonValue;
use std::error::Error;

pub fn query(db: &DB, key_str: String) -> Result<JsonValue, Box<dyn Error>> {
  let avro_schema_json = r#"
    {"type":"record","name":"topLevelRecord","fields":[{"name":"id","type":["string","null"]},{"name":"value1","type":"long"},{"name":"value2","type":"double"},{"name":"value3","type":["string","null"]},{"name":"value4","type":"int"}]}
    "#;

  let avro_schema = Schema::parse_str(avro_schema_json)?;
  // dbg!(&avro_schema);

  match db.get(&key_str) {
    Ok(Some(value)) => {
      let parsed = from_avro_datum(&avro_schema, &mut &value[..], None);
      // let parsed = reader.next().unwrap();
      use std::convert::TryFrom;
      let json: JsonValue = JsonValue::try_from(parsed.unwrap())?;
      Ok(json)
    }
    Ok(None) => {
      todo!("Create new error type")
    }
    Err(e) => Err(Box::new(e)),
  }
}
