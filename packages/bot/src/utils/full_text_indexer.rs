use tantivy::{
    collector::TopDocs, 
    query::QueryParser, 
    schema::{NumericOptions, OwnedValue, Schema, TextOptions}, 
    Index, 
    IndexReader, 
    ReloadPolicy, 
    TantivyDocument
};
use crate::utils::tardir::TarDirectory;

pub enum FieldOptions {
    Text(TextOptions),
    Numeric(NumericOptions),
}

pub struct Field {
    pub name: String,
    pub opts: FieldOptions,
}

#[derive(Clone)]
pub struct FullTextIndexer {
    schema: Schema,
    reader: IndexReader,
    query_parser: QueryParser,
}

impl FullTextIndexer {
    pub fn from_tar(
        bytes: Box<[u8]>,
        root: String,
        fields: &Vec<Field>,
        id_field_name: &str
    ) -> Result<Self, String> {
        let schema = FullTextIndexer::_gen_schema(fields);
        
        let index = Index::open_or_create(
            TarDirectory::from_bytes(bytes, root).unwrap(), 
            schema.clone()
        ).map_err(|e| e.to_string())?;

        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .map_err(|e| e.to_string())?;

        let query_parser = QueryParser::for_index(
            &index, 
            schema.fields()
                .filter(|e| e.1.name() != id_field_name)
                .map(|e| e.0)
                .collect()
        );

        Ok(Self {
            schema,
            reader,
            query_parser,
        })
    }

    fn _gen_schema(
        schema: &Vec<Field>
    ) -> Schema {
        let mut schema_builder = Schema::builder();
        for field in schema {
            match &field.opts {
                FieldOptions::Text(options) => {
                    schema_builder.add_text_field(&field.name, options.clone());
                },
                FieldOptions::Numeric(options) => {
                    schema_builder.add_u64_field(&field.name, options.clone());
                },
            }
        }
        schema_builder.build()
    }

    pub fn search<T, F>(
        &self,
        query_str: &str,
        id_field_name: &str,
        limit: usize,
        to_type: F
    ) -> Result<Vec<T>, String>
        where F: Fn(&OwnedValue) -> T {
        let id_field = self.schema.get_field(id_field_name).map_err(|e| e.to_string())?;
        let searcher = self.reader.searcher();
        let query = self.query_parser.parse_query(query_str).unwrap();
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit)).map_err(|e| e.to_string())?;
        let mut res = vec![];
        for (_, address) in top_docs {
            let doc: TantivyDocument = searcher.doc(address).unwrap();
            let doc_id = to_type(doc.get_first(id_field).unwrap());
            res.push(doc_id);
        }
        Ok(res)
    }
}
