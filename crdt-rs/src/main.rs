use automerge::{transaction::Transactable, Automerge, Change, ReadDoc, ScalarValue, Value};
use automerge::iter::MapRangeItem;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The initial state can be any JSON object
    let initial_json = json!({
        "@context": "https://schema.org",
        "@type": "Thing",
        "name": "Foo",
        "description": "Lorem ipsum",
        "url": "https://example.com"
    });

    // 1. Create an Automerge document from the initial state.
    let mut doc = Automerge::new();
    let mut tx = doc.transaction();
    for (key, value) in initial_json.as_object().unwrap() {
        if let serde_json::Value::String(s) = value {
            tx.put(automerge::ROOT, key, s.as_str())?;
        }
    }
    tx.commit();

    // 4. On another machine, which has the original document.
    // Let's clone doc to simulate this, before we make changes.
    let mut remote_doc = doc.clone();

    // 2. A user makes changes in a transaction.
    let old_heads = doc.get_heads(); // Get heads before the change
    let mut tx = doc.transaction();
    tx.put(automerge::ROOT, "name", "Bar")?;
    tx.put(
        automerge::ROOT,
        "image",
        "https://example.com/cat.png",
    )?;
    tx.delete(automerge::ROOT, "url")?;
    tx.commit();

    // 3. Get only the new binary changes that represent the modifications.
    // We must clone the changes because apply_changes takes owned `Change` objects.
    let changes: Vec<Change> = doc.get_changes(&old_heads).into_iter().cloned().collect();

    println!(
        "Generated {} binary change object(s) to represent the patch.",
        changes.len()
    );

    // Log both hex and human-readable representation of the changes
    for (index, change) in changes.iter().enumerate() {
        let hex = hex::encode(change.raw_bytes());
        println!("\nHex of change #{}:", index + 1);
        println!("{}", hex);
        
        // Get human-readable representation
        println!("\nHuman-readable change #{}:", index + 1);
        println!("Actor: {}", change.actor_id());
        println!("Sequence: {}", change.start_op());
        println!("Time: {}", change.timestamp());
        println!("Message: {:?}", change.message());
        println!("Hash: {}", hex::encode(change.hash()));
        println!("Operations: {} ops", change.len());
        
        // Decode the change to get more details
        let decoded = change.decode();
        println!("Decoded operations: {:?}", decoded.operations);
    }

    // 5. Apply the received changes to the remote document.
    remote_doc.apply_changes(changes)?;

    // Let's see the final, merged state from the remote doc's perspective
    println!("\nFinal merged document:");
    let final_json = serde_json::to_string_pretty(&doc_to_json(&remote_doc))?;
    println!("{}", final_json);

    Ok(())
}

fn doc_to_json(doc: &Automerge) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    // Destructure the MapRangeItem struct correctly
    for MapRangeItem { key, value, .. } in doc.map_range(automerge::ROOT, ..) {
        let val = match value {
            Value::Scalar(s) => match s.as_ref() {
                ScalarValue::Str(s) => serde_json::Value::String(s.to_string()),
                ScalarValue::Int(i) => serde_json::Value::Number((*i).into()),
                ScalarValue::Uint(u) => serde_json::Value::Number((*u).into()),
                ScalarValue::F64(f) => {
                    serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap())
                }
                ScalarValue::Boolean(b) => serde_json::Value::Bool(*b),
                ScalarValue::Null => serde_json::Value::Null,
                _ => serde_json::Value::Null,
            },
            _ => serde_json::Value::Null,
        };
        map.insert(key.to_string(), val);
    }
    serde_json::Value::Object(map)
}
