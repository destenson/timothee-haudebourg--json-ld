//! This bit of code is used to generate the toRdf tests for the crate. It it also a good
//! example of what the crate is capable of.
#[macro_use]
extern crate log;
extern crate iref;
extern crate stderrlog;
#[macro_use]
extern crate static_iref;
#[macro_use]
extern crate iref_enum;
extern crate json_ld;

use iref::Iri;
use json_ld::{context, object::*, Document, ErrorCode, FsLoader, Lexicon, Loader, ProcessingMode};
use serde_json::Value;
use std::convert::TryInto;

const URL: Iri = iri!("https://w3c.github.io/json-ld-api/tests/toRdf-manifest.jsonld");
const VERBOSITY: usize = 2;

/// Vocabulary of the test manifest
#[derive(IriEnum, Clone, Copy, PartialEq, Eq, Hash)]
#[iri_prefix("rdfs" = "http://www.w3.org/2000/01/rdf-schema#")]
#[iri_prefix("manifest" = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#")]
#[iri_prefix("vocab" = "https://w3c.github.io/json-ld-api/tests/vocab#")]
pub enum Vocab {
	#[iri("rdfs:comment")]
	Comment,

	#[iri("manifest:name")]
	Name,
	#[iri("manifest:entries")]
	Entries,
	#[iri("manifest:action")]
	Action,
	#[iri("manifest:result")]
	Result,

	#[iri("vocab:PositiveEvaluationTest")]
	PositiveEvalTest,
	#[iri("vocab:PositiveSyntaxTest")]
	PositiveSyntaxTest,
	#[iri("vocab:NegativeEvaluationTest")]
	NegativeEvalTest,
	#[iri("vocab:option")]
	Option,
	#[iri("vocab:specVersion")]
	SpecVersion,
	#[iri("vocab:normative")]
	Normative,
	#[iri("vocab:processingMode")]
	ProcessingMode,
	#[iri("vocab:expandContext")]
	ExpandContext,
	#[iri("vocab:base")]
	Base,
	#[iri("vocab:rdfDirection")]
	RdfDirection,
	#[iri("vocab:produceGeneralizedRdf")]
	ProduceGeneralizedRdf,
}

pub type Id = Lexicon<Vocab>;

#[async_std::main]
async fn main() {
	stderrlog::new().verbosity(VERBOSITY).init().unwrap();

	let mut loader = FsLoader::<Value>::new(|s| serde_json::from_str(s));
	loader.mount(iri!("https://w3c.github.io/json-ld-api"), "json-ld-api");

	let doc = loader
		.load(URL)
		.await
		.expect("unable to load the test suite");

	let expanded_doc = doc
		.expand::<context::Json<Value, Id>, _>(&mut loader)
		.await
		.expect("expansion failed");

	println!(include_str!("../tests/templates/to_rdf-header.rs"));

	for item in &expanded_doc {
		if let Object::Node(item) = item.as_ref() {
			for entries in item.get(Vocab::Entries) {
				if let Object::List(entries) = entries.as_ref() {
					for entry in entries {
						if let Object::Node(entry) = entry.as_ref() {
							generate_test(entry);
						}
					}
				}
			}
		}
	}

	info!("done.");
}

fn func_name(id: &str) -> String {
	let mut name = "to_rdf_".to_string();

	for c in id.chars() {
		match c {
			'.' | '-' => break,
			_ => name.push(c),
		}
	}

	name
}

fn generate_test(entry: &Node<Value, Id>) {
	let name = entry.get(Vocab::Name).next().unwrap().as_str().unwrap();
	let url = entry.get(Vocab::Action).next().unwrap().as_iri().unwrap();
	let mut base_url = url;

	if url == "https://w3c.github.io/json-ld-api/tests/toRdf/e122-in.jsonld" {
		warn!(
			"skipping invalid test {} (https://github.com/w3c/json-ld-api/issues/480#issuecomment-911718610)",
			url
		);
		return;
	}

	if url == "https://w3c.github.io/json-ld-api/tests/toRdf/li12-in.jsonld" {
		warn!(
			"skipping invalid test {} (https://github.com/w3c/json-ld-api/issues/533)",
			url
		);
		return;
	}

	let func_name = func_name(url.path().file_name().unwrap());

	let mut processing_mode = ProcessingMode::JsonLd1_1;
	let mut context_url = "None".to_string();
	let mut rdf_direction = "None".to_string();
	let mut produce_generalized_rdf = false;

	for option in entry.get(Vocab::Option) {
		if let Object::Node(option) = option.as_ref() {
			for normative in option.get(Vocab::Normative) {
				if let Some(false) = normative.inner().as_bool() {
					info!("skipping test {} (non normative)", url);
					return;
				}
			}

			for spec_version in option.get(Vocab::SpecVersion) {
				if let Some(spec_version) = spec_version.as_str() {
					if spec_version != "json-ld-1.1" {
						info!(
							"skipping test {} (unsupported json-ld version {})",
							url, spec_version
						);
						return;
					}
				}
			}

			for mode in option.get(Vocab::ProcessingMode) {
				processing_mode = mode.as_str().unwrap().try_into().unwrap();
			}

			for expand_context in option.get(Vocab::ExpandContext) {
				if let Some(url) = expand_context.as_iri() {
					context_url = format!("Some(iri!(\"{}\"))", url)
				}
			}

			for base in option.get(Vocab::Base) {
				if let Some(url) = base.as_iri() {
					base_url = url
				}
			}

			for direction in option.get(Vocab::RdfDirection) {
				match direction.inner().as_str() {
					Some("i18n-datatype") => {
						rdf_direction = "Some(RdfDirection::I18nDatatype)".to_string()
					}
					Some("compound-literal") => {
						rdf_direction = "Some(RdfDirection::CompoundLiteral)".to_string()
					}
					_ => warn!("invalid `rdfDirection` option"),
				}
			}

			for b in option.get(Vocab::ProduceGeneralizedRdf) {
				if let Some(b) = b.inner().as_bool() {
					produce_generalized_rdf = b
				}
			}
		}
	}

	let mut comments = String::new();
	for comment in entry.get(Vocab::Comment) {
		comments += format!("\n\tprintln!(\"{}\");", comment.as_str().unwrap()).as_str()
	}

	if entry.has_type(&Vocab::PositiveEvalTest) {
		let output_url = entry.get(Vocab::Result).next().unwrap().as_iri().unwrap();

		println!(
			include_str!("../tests/templates/to_rdf-test-positive.rs"),
			func_name,
			url,
			base_url,
			output_url,
			name,
			comments,
			processing_mode,
			context_url,
			rdf_direction,
			produce_generalized_rdf
		);
	} else if entry.has_type(&Vocab::NegativeEvalTest) {
		let error_code: ErrorCode = entry
			.get(Vocab::Result)
			.next()
			.unwrap()
			.as_str()
			.unwrap()
			.try_into()
			.unwrap();

		println!(
			include_str!("../tests/templates/to_rdf-test-negative.rs"),
			func_name,
			url,
			base_url,
			name,
			comments,
			processing_mode,
			context_url,
			rdf_direction,
			produce_generalized_rdf,
			error_code
		);
	} else if entry.has_type(&Vocab::PositiveSyntaxTest) {
		println!(
			include_str!("../tests/templates/to_rdf-test-syntax.rs"),
			func_name
		)
	} else {
		panic!("cannot decide how to evaluate test result")
	}
}