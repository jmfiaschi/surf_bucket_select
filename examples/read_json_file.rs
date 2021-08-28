use futures::TryStreamExt;
use rusoto_s3::{
    InputSerialization, JSONInput, JSONOutput, OutputSerialization, SelectObjectContentRequest,
};
use std::io;
use surf_bucket_select::model::{
    event_stream::EventStream, select_object_content::SelectObjectContentEventStreamItem,
};

#[async_std::main]
async fn main() -> io::Result<()> {
    let client = surf::client();

    let input_serialization = InputSerialization {
        json: Some(JSONInput {
            type_: Some("DOCUMENT".to_string()),
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
        key: "data/multi_lines.json".to_owned(),
        expression: "select * from s3object[*].results[*] r where r.number = 20".to_owned(),
        expression_type: "SQL".to_owned(),
        input_serialization,
        output_serialization,
        ..Default::default()
    };

    // define custom key
    // let credentials_provider =
    //     rusoto_core::credential::StaticProvider::new_minimal("minio_access_key".to_owned(), "minio_secret_key".to_owned());
    // or
    // use default local credentials
    let credentials_provider = rusoto_core::credential::DefaultCredentialsProvider::new()
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;

    // Sign the request header
    let request_builder = surf_bucket_select::select_object_content(
        "http://localhost:9000".to_string(),
        select_object_content_request,
        Some(Box::new(credentials_provider)),
        "eu-east-3".to_string(),
        None,
    )
    .await
    .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;

    let req = request_builder.build();

    let mut res = client
        .send(req)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))?;

    let body_bytes = res
        .body_bytes()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    if !res.status().is_success() {
        return Err(io::Error::new(
            io::ErrorKind::Interrupted,
            format!(
                "Curl failed with status code '{}' and response body: {}",
                res.status(),
                String::from_utf8(body_bytes)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
            ),
        ));
    }

    if body_bytes.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionAborted,
            "The response body is empty or surf can't read the body.",
        ));
    }

    let mut event_stream =
        EventStream::<SelectObjectContentEventStreamItem>::new(body_bytes.clone());

    let mut data = String::default();

    while let Ok(Some(item)) = event_stream.try_next().await {
        match item {
            SelectObjectContentEventStreamItem::Records(records_event) => {
                data.push_str(
                    String::from_utf8(
                        records_event
                            .payload
                            .expect("Failed to return the event payload")
                            .slice(0..)
                            .to_vec(),
                    )
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
                    .as_str(),
                );
            }
            SelectObjectContentEventStreamItem::End(_end_event) => break,
            _ => {}
        }
    }

    println!("{}", data);

    Ok(())
}
