use tantivy::{doc, Index, IndexWriter, schema::{Schema, TEXT, STORED}, Term};
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use tantivy::{TantivyDocument, ReloadPolicy, schema::*, Searcher};
use std::{sync::mpsc::{self, Receiver, Sender}, path::Path, thread};

type FullTextErr = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct IndexW {
    request_tx: Sender::<(TantivyDocument, oneshot::Sender::<usize>)>,
    query_tx:   Sender::<(String, oneshot::Sender::<usize>)>
}

impl IndexW {

    fn idx_writer(idx_dir: String) -> Result< (IndexWriter::<TantivyDocument>, Searcher, QueryParser, Schema), FullTextErr>  {
        let index_dir = Path::new(&idx_dir);
        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT);
        let content = schema_builder.add_text_field("content", TEXT | STORED);
        let schema = schema_builder.build();
        let index = Index::create_in_dir(index_dir, schema.clone())?;
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;
        let searcher = reader.searcher();
        let query_parser = QueryParser::for_index(&index, vec![title, content]);
        Ok( (
            index.writer::<TantivyDocument>(30_000_000)?, searcher, 
            query_parser, schema
        ) )
    }

    pub fn new(idx_dir: String) -> Result<Self, FullTextErr> {
        let (request_tx, request_rx) = mpsc::channel::<(TantivyDocument, oneshot::Sender<usize>)>();
        let (mut idx_writer, searcher, qry_parser, schema) = 
            Self::idx_writer(idx_dir)?;

        thread::spawn(move || {
            for (d, ch) in request_rx.iter() {
                let _ = idx_writer.add_document(d);
                let _ = ch.send(10);
            }
            let _ = idx_writer.commit()?;
            Ok::<(), FullTextErr>(())
        });

        let (query_tx, query_rx) = mpsc::channel::<(String, oneshot::Sender<usize>)>();
        thread::spawn(move || {
            for (qry, snd) in query_rx.iter() {
                let query = qry_parser.parse_query(&qry).unwrap();
                let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
                    for (score, doc_addr) in top_docs {
                        println!("Score: {}, Document ID: {:?}", score, doc_addr);
                        // let stored_fields = searcher.doc(doc_addr)?.stored_fields();
                        // println!("Content: {}", stored_fields.get_first(content).unwrap());
                        let retrieved_doc: TantivyDocument = searcher.doc(doc_addr).unwrap();
                        println!("{}", retrieved_doc.to_json(&schema));
                    }
            }
        });
        Ok( IndexW { request_tx,  query_tx} )
    }

    pub fn write_document(&self, doc: TantivyDocument) -> impl std::future::Future<Output = oneshot::Receiver::<usize> > {
        let sender = self.request_tx.clone();
        let (resp_tx, resp_rx) = oneshot::channel();
        async move {
            let _ = sender.send((doc, resp_tx));
            resp_rx
        }
    }

    pub fn search_document(&self, query: String) -> impl std::future::Future<Output = oneshot::Receiver::<usize> > {
        let sender = self.query_tx.clone();
        let (resp_tx, resp_rx) = oneshot::channel();
        async move {
            let _ = sender.send((query, resp_tx));
            resp_rx
        }
    }


}

pub use tantivy;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_create_search_index() {
        let idx_writer = IndexW::new("./index".to_string()).unwrap();
        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT);
        let content = schema_builder.add_text_field("content", TEXT | STORED);
        let _schema = schema_builder.build();
        let resp_rx = idx_writer.write_document(
            doc!(
               title => "Concurrency in Rust",
                content => "Concurrency in Rust is managed via tasks and channels."
            )
        ).await;
        let _ = resp_rx.await;
    }
}
