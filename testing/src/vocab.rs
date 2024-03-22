use iref_enum::IriEnum;
pub use rdf_types::vocabulary::LiteralIndex;

pub type IriIndex = rdf_types::vocabulary::IriOrIndex<Vocab>;
pub type BlankIdIndex = rdf_types::vocabulary::BlankIdIndex;

pub type IndexTerm = rdf_types::Term<rdf_types::Id<IriIndex, BlankIdIndex>, LiteralIndex>;
pub type IndexQuad = rdf_types::Quad<IndexTerm>;

#[derive(Debug, IriEnum, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Vocab {
	Rdf(Rdf),
	Rdfs(Rdfs),
	Xsd(Xsd),
	Manifest(Manifest),
	Test(Test),
}

#[derive(Debug, IriEnum, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[iri_prefix("rdf" = "http://www.w3.org/1999/02/22-rdf-syntax-ns#")]
pub enum Rdf {
	#[iri("rdf:type")]
	Type,
}

#[derive(Debug, IriEnum, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[iri_prefix("rdfs" = "http://www.w3.org/2000/01/rdf-schema#")]
pub enum Rdfs {
	#[iri("rdfs:comment")]
	Comment,
}

#[derive(Debug, IriEnum, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[iri_prefix("xsd" = "http://www.w3.org/2001/XMLSchema#")]
pub enum Xsd {
	#[iri("xsd:boolean")]
	Boolean,

	#[iri("xsd:string")]
	String,
}

#[derive(Debug, IriEnum, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[iri_prefix("manifest" = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#")]
pub enum Manifest {
	#[iri("manifest:name")]
	Name,
	#[iri("manifest:entries")]
	Entries,
	#[iri("manifest:action")]
	Action,
	#[iri("manifest:result")]
	Result,
}

#[derive(Debug, IriEnum, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[iri_prefix("test" = "https://w3c.github.io/json-ld-api/tests/vocab#")]
pub enum Test {
	#[iri("test:PositiveEvaluationTest")]
	PositiveEval,
	#[iri("test:NegativeEvaluationTest")]
	NegativeEval,
	#[iri("test:context")]
	Context,
	#[iri("test:option")]
	Option,
	#[iri("test:base")]
	Base,
	#[iri("test:compactArrays")]
	CompactArrays,
	#[iri("test:processingMode")]
	ProcessingMode,
	#[iri("test:specVersion")]
	SpecVersion,
}
