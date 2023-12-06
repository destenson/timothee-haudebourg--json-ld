use json_ld_context_processing::ContextLoader;
use json_ld_core::{
	future::{BoxFuture, FutureExt},
	ExpandedDocument, FlattenedDocument, Term,
};
use json_ld_syntax::{IntoJson, IntoJsonMeta, Keyword};
use locspan::Meta;
use rdf_types::{vocabulary, Vocabulary};
use std::hash::Hash;

use crate::{
	iri::{compact_iri, IriConfusedWithPrefix},
	CompactFragmentMeta,
};

pub type CompactDocumentResult<I, M, L> = Result<
	json_syntax::MetaValue<M>,
	crate::MetaError<M, <L as ContextLoader<I, M>>::ContextError>,
>;

/// Context embeding method.
///
/// This trait provides the `embed_context` method that can be used
/// to include a JSON-LD context to a JSON-LD document.
/// It is used at the end of compaction algorithm to embed to
/// context used to compact the document into the compacted output.
pub trait EmbedContext<I, B, M> {
	/// Embeds the given context into the document.
	fn embed_context<N>(
		&mut self,
		vocabulary: &N,
		context: json_ld_context_processing::ProcessedRef<I, B, M>,
		options: crate::Options,
	) -> Result<(), Meta<IriConfusedWithPrefix, M>>
	where
		N: Vocabulary<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq,
		B: Clone + Hash + Eq,
		M: Clone;
}

/// Compaction with metadata.
pub trait CompactMeta<I, B, M> {
	/// Compacts the input document with full options.
	fn compact_full_meta<
		'a,
		N,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		meta: &'a M,
		vocabulary: &'a mut N,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
		options: crate::Options,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		N: Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync;
}

/// Compaction function.
pub trait Compact<I, B, M> {
	/// Compacts the input document with full options.
	fn compact_full<
		'a,
		N,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		vocabulary: &'a mut N,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
		options: crate::Options,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		N: Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync;

	/// Compacts the input document with the given `vocabulary` to
	/// interpret identifiers.
	fn compact_with<
		'a,
		N,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		vocabulary: &'a mut N,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		N: Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync,
	{
		self.compact_full(vocabulary, context, loader, crate::Options::default())
	}

	/// Compacts the input document.
	fn compact<
		'a,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		(): Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync,
	{
		self.compact_with(vocabulary::no_vocabulary_mut(), context, loader)
	}
}

impl<T: CompactMeta<I, B, M>, I, B, M> Compact<I, B, M> for Meta<T, M> {
	fn compact_full<
		'a,
		N,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		vocabulary: &'a mut N,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
		options: crate::Options,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		N: Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync,
	{
		self.value()
			.compact_full_meta(self.metadata(), vocabulary, context, loader, options)
	}
}

impl<I, B, M> CompactMeta<I, B, M> for ExpandedDocument<I, B, M> {
	fn compact_full_meta<
		'a,
		N,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		meta: &'a M,
		vocabulary: &'a mut N,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
		options: crate::Options,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		N: Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync,
	{
		async move {
			let mut compacted_output = self
				.objects()
				.compact_fragment_full_meta(
					meta,
					vocabulary,
					context.processed(),
					context.processed(),
					None,
					loader,
					options,
				)
				.await?;

			compacted_output
				.embed_context(vocabulary, context, options)
				.map_err(Meta::cast)?;

			Ok(compacted_output)
		}
		.boxed()
	}
}

impl<I, B, M> CompactMeta<I, B, M> for FlattenedDocument<I, B, M> {
	fn compact_full_meta<
		'a,
		N,
		L: json_ld_core::Loader<I, M> + json_ld_context_processing::ContextLoader<I, M>,
	>(
		&'a self,
		meta: &'a M,
		vocabulary: &'a mut N,
		context: json_ld_context_processing::ProcessedRef<'a, 'a, I, B, M>,
		loader: &'a mut L,
		options: crate::Options,
	) -> BoxFuture<'a, CompactDocumentResult<I, M, L>>
	where
		N: Send + Sync + rdf_types::VocabularyMut<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq + Send + Sync,
		B: Clone + Hash + Eq + Send + Sync,
		M: Clone + Send + Sync,
		L: Send + Sync,
	{
		async move {
			let mut compacted_output = self
				.compact_fragment_full_meta(
					meta,
					vocabulary,
					context.processed(),
					context.processed(),
					None,
					loader,
					options,
				)
				.await?;

			compacted_output
				.embed_context(vocabulary, context, options)
				.map_err(Meta::cast)?;

			Ok(compacted_output)
		}
		.boxed()
	}
}

impl<I, B, M> EmbedContext<I, B, M> for json_syntax::MetaValue<M> {
	fn embed_context<N>(
		&mut self,
		vocabulary: &N,
		context: json_ld_context_processing::ProcessedRef<I, B, M>,
		options: crate::Options,
	) -> Result<(), Meta<IriConfusedWithPrefix, M>>
	where
		N: Vocabulary<Iri = I, BlankId = B>,
		I: Clone + Hash + Eq,
		B: Clone + Hash + Eq,
		M: Clone,
	{
		let value = self.value_mut().take();

		let obj = match value {
			json_syntax::Value::Array(array) => {
				let mut obj = json_syntax::Object::new();

				if !array.is_empty() {
					let key = compact_iri(
						vocabulary,
						context.processed(),
						Meta(&Term::Keyword(Keyword::Graph), self.metadata()),
						true,
						false,
						options,
					)
					.map_err(Meta::cast)?;

					obj.insert(
						key.unwrap().cast(),
						Meta(array.into(), self.metadata().clone()),
					);
				}

				Some(obj)
			}
			json_syntax::Value::Object(obj) => Some(obj),
			_null => None,
		};

		if let Some(mut obj) = obj {
			let json_context = IntoJson::into_json(context.unprocessed().cloned());

			if !obj.is_empty()
				&& !json_context.is_null()
				&& !json_context.is_empty_array_or_object()
			{
				obj.insert(
					Meta("@context".into(), json_context.metadata().clone()),
					json_context,
				);
			}

			*self.value_mut() = obj.into()
		};

		Ok(())
	}
}
