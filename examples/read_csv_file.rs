#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;

use bucket_select::model::{event_stream::EventStream, select_object_content::{CSVInput, InputSerialization, JSONOutput, OutputSerialization, SelectObjectContentEventStreamItem, SelectObjectContentRequest}};
use futures::TryStreamExt;
use rusoto_core::{
    credential::StaticProvider,
};
use slog::{Drain, FnValue};
use std::io;

#[async_std::main]
async fn main() -> io::Result<()> {
    // Init logger.
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_envlogger::new(drain);
    let drain = slog_async::Async::default(drain).fuse();
    let logger = slog::Logger::root(
        drain.fuse(),
        o!("file" => FnValue(move |info| {format!("{}:{}",info.file(),info.line())})),
    );
    let _scope_guard = slog_scope::set_global_logger(logger);

    let client = surf::client();

    let input_serialization = InputSerialization {
        csv: Some(CSVInput {
            ..Default::default()
        }),
        ..Default::default()
    };

    let output_serialization = OutputSerialization {
        json: Some(JSONOutput {
            ..Default::default()
        }),
        ..Default::default()
    };

    let select_object_content_request = SelectObjectContentRequest {
        bucket: "my-bucket".to_owned(),
        key: "data/multi_lines.csv".to_owned(),
        expression: "select * from s3object".to_owned(),
        expression_type: "SQL".to_owned(),
        input_serialization,
        output_serialization,
        ..Default::default()
    };

    let credentials_provider =
        StaticProvider::new_minimal("minio_access_key".to_owned(), "minio_secret_key".to_owned());

    let request_builder = bucket_select::select_object_content(
        "http://localhost:9000".to_string(),
        select_object_content_request,
        Some(Box::new(credentials_provider)),
        "eu-east-3".to_string(),
        None,
    )
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    let req = request_builder.build();

    let mut res = client
        .send(req)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;

    let payload = res
        .body_bytes()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if !res.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            format!(
                "Curl failed with status code '{}' and response body: {}",
                res.status(),
                String::from_utf8(payload)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            ),
        ));
    }

    let mut event_stream = EventStream::<SelectObjectContentEventStreamItem>::new(payload.clone());
    
    let mut data = String::default();

    while let Ok(Some(item)) = event_stream.try_next().await {
        match item {
            SelectObjectContentEventStreamItem::Records(records_event) => {
                data = String::from_utf8(records_event
                        .payload
                        .expect("Failed to return the event payload")
                        .slice(0..).to_vec()).unwrap();
            },
            _ => {},
        }
    }

    println!("{:?}", data);

    Ok(())
}
