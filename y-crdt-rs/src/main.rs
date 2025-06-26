use serde_json::json;
use std::collections::HashMap;
use yrs::{Doc, Map, Transact, ReadTxn, MapRef, updates::decoder::Decode, Update, StateVector};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The initial state can be any JSON object
    let initial_json = json!({
        "@context": "https://schema.org",
        "@type": "Thing",
        "name": "Foo",
        "description": "Lorem ipsum",
        "url": "https://example.com"
    });

    // 1. Create a Y.js document from the initial state
    let doc1 = Doc::new();
    let map1 = doc1.get_or_insert_map("data");
    
    // Initialize with the initial state
    {
        let mut txn = doc1.transact_mut();
        for (key, value) in initial_json.as_object().unwrap() {
            if let serde_json::Value::String(s) = value {
                map1.insert(&mut txn, key.as_str(), s.as_str());
            }
        }
        txn.commit();
    }

    // Get initial state vector before changes
    let initial_sv = doc1.transact().state_vector();

    // 2. A user makes changes
    {
        let mut txn = doc1.transact_mut();
        map1.insert(&mut txn, "name", "Bar");
        map1.insert(&mut txn, "image", "https://example.com/cat.png");
        map1.remove(&mut txn, "url");
        txn.commit();
    }

    // 3. Get only the new changes that represent the modifications
    let changes = doc1.transact().encode_state_as_update_v1(&initial_sv);

    // 4. On another machine, which has the original document
    // Let's create a new doc to simulate this
    let remote_doc = Doc::new();
    let remote_map = remote_doc.get_or_insert_map("data");

    // Apply initial state to remote doc
    {
        let mut txn = remote_doc.transact_mut();
        for (key, value) in initial_json.as_object().unwrap() {
            if let serde_json::Value::String(s) = value {
                remote_map.insert(&mut txn, key.as_str(), s.as_str());
            }
        }
        txn.commit();
    }

    println!("Generated binary change object to represent the patch.");

    // Log both hex and human-readable representation of the changes
    let hex = hex::encode(&changes);
    println!("\nHex of change:");
    println!("{}", hex);
    
    // Get human-readable representation
    println!("\nHuman-readable change:");
    println!("Update size: {} bytes", changes.len());
    println!("Binary data: {}", hex::encode(&changes[..20.min(changes.len())]));
    
    // Try to decode the update for more details
    match Update::decode_v1(&changes) {
        Ok(decoded_update) => {
            println!("Decoded update: {:?}", decoded_update);
        }
        Err(e) => {
            println!("Could not decode update: {}", e);
        }
    }

    // 5. Apply the received changes to the remote document
    {
        let mut txn = remote_doc.transact_mut();
        let _ = txn.apply_update(Update::decode_v1(&changes)?);
        txn.commit();
    }

    // Let's see the final, merged state from the remote doc's perspective
    println!("\nFinal merged document:");
    let final_json = doc_to_json(&remote_doc, &remote_map);
    println!("{}", serde_json::to_string_pretty(&final_json)?);

    Ok(())
}

fn doc_to_json(doc: &Doc, map: &MapRef) -> serde_json::Value {
    let txn = doc.transact();
    let mut result = HashMap::new();
    
    for (key, value) in map.iter(&txn) {
        let json_value = match value {
            yrs::Out::Any(yrs::Any::String(s)) => serde_json::Value::String(s.to_string()),
            yrs::Out::Any(yrs::Any::Bool(b)) => serde_json::Value::Bool(b),
            yrs::Out::Any(yrs::Any::Number(n)) => serde_json::Value::Number(serde_json::Number::from_f64(n).unwrap()),
            yrs::Out::Any(yrs::Any::BigInt(i)) => serde_json::Value::Number(serde_json::Number::from(i)),
            yrs::Out::Any(yrs::Any::Null) => serde_json::Value::Null,
            _ => serde_json::Value::String(format!("{:?}", value)),
        };
        result.insert(key.to_string(), json_value);
    }
    
    serde_json::Value::Object(result.into_iter().collect())
}