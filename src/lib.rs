extern crate rusoto_core;
extern crate serde;

use std::collections::BTreeMap;
use std::time::Duration;

use rusoto_core::Region;
use rusoto_core::credential::{Anonymous, CredentialsError};
use rusoto_core::encoding::ContentEncoding;
use rusoto_core::param::ServiceParams;
use rusoto_core::{credential::ProvideAwsCredentials, signature::SignedRequest};
use rusoto_s3::{SelectObjectContentRequest, SelectObjectContentRequestSerializer};
use surf::http::Method;
use surf::{RequestBuilder, Url};
use xml::EventWriter;
use serde::{Serialize, Deserialize};

pub type Params = BTreeMap<String, Option<String>>;

pub mod model;

#[derive(Serialize, Deserialize)]
struct QueryParam {
    select: String,
    #[serde(rename = "select-type")]
    select_type: u32
}

pub async fn select_object_content(hostname: String, 
        select_object_content_request: SelectObjectContentRequest,
        credentials_provider: Option<Box<dyn ProvideAwsCredentials + Send + Sync>>,
        region: String,
        timeout: Option<Duration>) -> surf::Result<RequestBuilder> {
    let uri: Url = format!("{}/{}/{}", hostname, select_object_content_request.bucket, select_object_content_request.key).parse().unwrap();    

    let region = Region::Custom {
        name: region.to_owned(),
        endpoint: uri.host_str().unwrap_or("").to_owned(),
    };

    let mut signed_request = SignedRequest::new("POST", "s3", &region, uri.path());
    
    signed_request.add_optional_header(
        "x-amz-server-side-encryption-customer-algorithm",
        select_object_content_request
            .sse_customer_algorithm
            .as_ref(),
    );
    signed_request.add_optional_header(
        "x-amz-server-side-encryption-customer-key",
        select_object_content_request.sse_customer_key.as_ref(),
    );
    signed_request.add_optional_header(
        "x-amz-server-side-encryption-customer-key-MD5",
        select_object_content_request.sse_customer_key_md5.as_ref(),
    );
    let mut params = Params::new();
    params.put_key("select");
    params.put("select-type", "2");
    signed_request.set_params(params);

    let mut writer = EventWriter::new(Vec::new());
    SelectObjectContentRequestSerializer::serialize(
        &mut writer,
        "SelectObjectContentRequest",
        &select_object_content_request,
        "http://s3.amazonaws.com/doc/2006-03-01/",
    )?;

    let paylaod = writer.into_inner();

    signed_request.set_payload(Some(paylaod.clone()));
    signed_request.set_content_type("application/xml; charset=utf-8".to_string());

    let encoding = ContentEncoding::default();
    encoding.encode(&mut signed_request);

    if let Some(provider) = credentials_provider {
        let credentials = if let Some(to) = timeout {
            async_std::future::timeout(to, provider.credentials())
                .await
                .map_err(|_| CredentialsError {
                    message: "Timeout getting credentials".to_owned(),
                })
                .and_then(std::convert::identity)
        } else {
            provider.credentials().await
        }
        .map_err(|err| CredentialsError {
            message: format!("Couldn't connect to credentials provider: {}", err).to_owned(),
        })?;
        if credentials.is_anonymous() {
            signed_request.complement();
        } else {
            signed_request.sign(&credentials);
        }
    } else {
        signed_request.complement();
    }

    let mut request_builder = surf::post(uri);

    for (key, value) in signed_request.headers() {
        request_builder = request_builder.header(key.clone().as_str(), canonical_values(value));
    }

    let query = QueryParam {
        select: "".to_string(),
        select_type: 2
    };
    request_builder = request_builder.query(&query)?;
    request_builder = request_builder.body(paylaod);

    Ok(request_builder)
}

/// Canonicalizes values into the AWS Canonical Form.
///
/// Read more about it: [HERE](http://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html)
fn canonical_values(values: &[Vec<u8>]) -> String {
    let mut st = String::new();
    for v in values {
        let s = std::str::from_utf8(v).unwrap();
        if !st.is_empty() {
            st.push(',')
        }
        if s.starts_with('\"') {
            st.push_str(s);
        } else {
            st.push_str(s.replace("  ", " ").trim());
        }
    }
    st
}
