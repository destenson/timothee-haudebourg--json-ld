use iref::IriBuf;
use json_ld::Expand;
use rdf_types::BlankIdBuf;
use static_iref::iri;

#[async_std::test]
async fn expand() {
	let json = json_ld::syntax::from_str(
		r##"
		{
			"@graph": [
				{
				"http://example.org/vocab#a": {
					"@graph": [
					{
						"http://example.org/vocab#b": "Chapter One"
					}
					]
				}
				}
			]
		}
	"##,
	)
	.unwrap();

	let document_url: IriBuf =
		iri!("https://w3c.github.io/json-ld-api/tests/0020-in.jsonld").into();
	let mut loader: json_ld::NoLoader<IriBuf, json_ld::syntax::Value<_, _>, _> =
		json_ld::NoLoader::new();
	let _: json_ld::ExpandedDocument<IriBuf, BlankIdBuf, _> =
		json.expand(Some(&document_url), &mut loader).await.unwrap();
}
