//! This example show how to simplify the `custom-ids.rs` example by using the `iref-enum` crate
//! to automatically generate the conversions to/from `Iri` using the `IriEnum` derive macro.
//#![feature(proc_macro_hygiene)]

extern crate async_std;
extern crate iref;
#[macro_use]
extern crate iref_enum;
extern crate json_ld;

use json_ld::{context, Document, Lexicon, NoLoader, Object};
use serde_json::Value;

// Parts of the FOAF vocabulary will need.
#[derive(IriEnum, Clone, Copy, PartialEq, Eq, Hash)]
#[iri_prefix("foaf" = "http://xmlns.com/foaf/0.1/")]
pub enum Foaf {
	#[iri("foaf:name")]
	Name,
	#[iri("foaf:mbox")]
	Mbox,
}

type Id = Lexicon<Foaf>;

#[async_std::main]
async fn main() {
	// The JSON-LD document to expand.
	let doc: Value = serde_json::from_str(
		r#"
		{
			"@context": {
				"name": "http://xmlns.com/foaf/0.1/name",
				"email": "http://xmlns.com/foaf/0.1/mbox"
			},
			"@id": "timothee.haudebourg.net",
			"name": "Timothée Haudebourg",
			"email": "author@haudebourg.net"
		}
	"#,
	)
	.unwrap();

	// JSON document loader.
	let mut loader = NoLoader::<Value>::new();

	// Expansion.
	let expanded_doc = doc
		.expand::<context::Json<Value, Id>, _>(&mut loader)
		.await
		.unwrap();

	// Iterate through the expanded objects.
	for object in expanded_doc {
		if let Object::Node(node) = object.as_ref() {
			println!("node: {}", node.id().unwrap()); // print the `@id`
			for name in node.get(Foaf::Name) {
				// <- Note how we can directly use `Foaf` here.
				println!("name: {}", name.as_str().unwrap());
			}

			for name in node.get(Foaf::Mbox) {
				// <- Note how we can directly use `Foaf` here.
				println!("email: {}", name.as_str().unwrap());
			}
		}
	}
}
