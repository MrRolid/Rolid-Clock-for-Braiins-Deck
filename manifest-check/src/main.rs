use bmc_widget_manifest::Manifest;
fn main() {
 let path=std::env::args().nth(1).expect("Usage: rolid-manifest-check manifest.json");
 match Manifest::from_reader(std::fs::File::open(&path).expect("read manifest")) {
  Ok(_)=>println!("Deck manifest parser: OK ({path})"),
  Err(e)=>{eprintln!("Deck manifest parser: {e}");std::process::exit(1);}
 }
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn release_manifest_passes_deck_parser() {
  Manifest::from_reader(include_bytes!("../../manifest.json").as_slice()).unwrap();
 }
 #[test]
 fn deleted_id_reference_is_rejected() {
  let mut m:serde_json::Value=serde_json::from_slice(include_bytes!("../../manifest.json")).unwrap();
  m["params"]["alarms"]["unique_items"]=serde_json::json!(["id"]);
  let bytes=serde_json::to_vec(&m).unwrap();
  let err=Manifest::from_reader(bytes.as_slice()).unwrap_err().to_string();
  assert!(err.contains("unique_items names"),"{err}");
 }
}
