import * as Y from "yjs";

// The initial state can be any JSON object
const initialState = {
  "@context": "https://schema.org",
  "@type": "Thing",
  name: "Foo",
  description: "Lorem ipsum",
  url: "https://example.com",
};

// 1. Create a Y.js document from the initial state
let doc1 = new Y.Doc();
const ymap1 = doc1.getMap("data");

// Initialize with the initial state
for (const [key, value] of Object.entries(initialState)) {
  ymap1.set(key, value);
}

// Get initial state to track changes
const initialState1 = Y.encodeStateAsUpdate(doc1);

// 2. A user makes changes
// Track changes before making modifications
const changes: Uint8Array[] = [];
doc1.on("update", (update: Uint8Array) => {
  changes.push(update);
});

// Make changes on the same document
ymap1.set("name", "Bar");
ymap1.set("image", "https://example.com/cat.png");
ymap1.delete("url");

console.log(`Generated ${changes.length} binary change object(s) to represent the patch.`);

// Log both hex and human-readable representation of the changes
changes.forEach((change, index) => {
  const hex = Buffer.from(change).toString('hex');
  console.log(`\nHex of change #${index + 1}:`);
  console.log(hex);
  
  // Get human-readable representation
  console.log(`\nHuman-readable change #${index + 1}:`);
  console.log('Update size:', change.length, 'bytes');
  console.log('Binary data:', Array.from(change.slice(0, 20)).map(b => b.toString(16).padStart(2, '0')).join(' '), '...');
  
  // Try to decode the update for more details
  try {
    const decoded = Y.decodeUpdate(change);
    console.log('Decoded update:', decoded);
  } catch (e) {
    console.log('Could not decode update:', e);
  }
});

// 4. On another machine, which has the original document
// Create a new document to simulate this
let remoteDoc = new Y.Doc();
const remoteMap = remoteDoc.getMap("data");

// Initialize remote doc with initial state
Y.applyUpdate(remoteDoc, initialState1);

// 5. Apply the received changes
changes.forEach(change => {
  Y.applyUpdate(remoteDoc, change);
});

// Let's see the final, merged state
console.log("\nFinal merged document:");
const finalState: any = {};
remoteMap.forEach((value: any, key: string) => {
  finalState[key] = value;
});
console.log(JSON.stringify(finalState, null, 2));