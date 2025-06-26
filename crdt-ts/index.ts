import * as Automerge from "@automerge/automerge";

// The initial state can be any JSON object
const initialState = {
  "@context": "https://schema.org",
  "@type": "Thing",
  name: "Foo",
  description: "Lorem ipsum",
  url: "https://example.com",
};

// 1. Create an Automerge document from the initial state.
// We use <any> because the schema is not known in advance.
let doc1 = Automerge.from<any>(initialState);

// 2. A user makes changes in a change block.
const doc2 = Automerge.change(doc1, "Applying user patch", (doc) => {
  // We can now modify the document as if it were a regular JavaScript object.
  doc.name = "Bar";
  doc.image = "https://example.com/cat.png";
  delete doc.url;
});

// 3. Get only the new binary changes that represent the modifications.
// This is the "patch" you would send over the network.
const changes = Automerge.getChanges(doc1, doc2);

console.log(`Generated ${changes.length} binary change object(s) to represent the patch.`);

// Log both hex and human-readable representation of the changes
changes.forEach((change, index) => {
  const hex = Buffer.from(change).toString('hex');
  console.log(`\nHex of change #${index + 1}:`);
  console.log(hex);
  
  // Get human-readable representation
  const changeInfo = Automerge.decodeChange(change);
  console.log(`\nHuman-readable change #${index + 1}:`);
  console.log('Message:', changeInfo.message);
  console.log('Actor:', changeInfo.actor);
  console.log('Time:', changeInfo.time);
  console.log('Operations:', JSON.stringify(changeInfo.ops, null, 2));
});

// 4. On another machine, which has the original document.
// Let's clone doc1 to simulate this.
let remoteDoc = Automerge.clone(doc1);

// 5. Apply the received changes.
// applyChanges returns a new document in an array: [newDoc]
[remoteDoc] = Automerge.applyChanges(remoteDoc, changes);

// Let's see the final, merged state
console.log("\nFinal merged document:");
console.log(JSON.stringify(remoteDoc, null, 2));