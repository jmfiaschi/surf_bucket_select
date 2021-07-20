use http::HeaderMap;
use rusoto_core::event_stream::{DeserializeEvent, EventStream};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse, find_start_element, deserialize_elements, deserialize_primitive, skip_tree, write_characters_element};
use rusoto_core::request::HttpResponse;
use rusoto_core::{ByteStream, RusotoError};
use serde::{Deserialize, Serialize};
use serde::de;
use xml::reader::EventReader;
use std::str::FromStr;
use xml::EventWriter;
use std::io::Write;

/// <p>Request to filter the contents of an Amazon S3 object based on a simple Structured Query Language (SQL) statement. In the request, along with the SQL expression, you must specify a data serialization format (JSON or CSV) of the object. Amazon S3 uses this to parse object data into records. It returns only records that match the specified SQL expression. You must also specify the data serialization format for the response. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectSELECTContent.html">S3Select API Documentation</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct SelectObjectContentRequest {
    /// <p>The S3 bucket.</p>
    pub bucket: String,
    /// <p>The account id of the expected bucket owner. If the bucket is owned by a different account, the request will fail with an HTTP <code>403 (Access Denied)</code> error.</p>
    pub expected_bucket_owner: Option<String>,
    /// <p>The expression that is used to query the object.</p>
    pub expression: String,
    /// <p>The type of the provided expression (for example, SQL).</p>
    pub expression_type: String,
    /// <p>Describes the format of the data in the object that is being queried.</p>
    pub input_serialization: InputSerialization,
    /// <p>The object key.</p>
    pub key: String,
    /// <p>Describes the format of the data that you want Amazon S3 to return in response.</p>
    pub output_serialization: OutputSerialization,
    /// <p>Specifies if periodic request progress information should be enabled.</p>
    pub request_progress: Option<RequestProgress>,
    /// <p>The SSE Algorithm used to encrypt the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_algorithm: Option<String>,
    /// <p>The SSE Customer Key. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_key: Option<String>,
    /// <p>The SSE Customer Key MD5. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ServerSideEncryptionCustomerKeys.html">Server-Side Encryption (Using Customer-Provided Encryption Keys</a>. </p>
    pub sse_customer_key_md5: Option<String>,
    /// <p><p>Specifies the byte range of the object to get the records from. A record is processed when its first byte is contained by the range. This parameter is optional, but when specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the start and end of the range.</p> <p> <code>ScanRange</code>may be used in the following ways:</p> <ul> <li> <p> <code>&lt;scanrange&gt;&lt;start&gt;50&lt;/start&gt;&lt;end&gt;100&lt;/end&gt;&lt;/scanrange&gt;</code> - process only the records starting between the bytes 50 and 100 (inclusive, counting from zero)</p> </li> <li> <p> <code>&lt;scanrange&gt;&lt;start&gt;50&lt;/start&gt;&lt;/scanrange&gt;</code> - process only the records starting after the byte 50</p> </li> <li> <p> <code>&lt;scanrange&gt;&lt;end&gt;50&lt;/end&gt;&lt;/scanrange&gt;</code> - process only the records within the last 50 bytes of the file.</p> </li> </ul></p>
    pub scan_range: Option<ScanRange>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct InputSerialization {
    /// <p>Describes the serialization of a CSV-encoded object.</p>
    pub csv: Option<CSVInput>,
    /// <p>Specifies object's compression format. Valid values: NONE, GZIP, BZIP2. Default Value: NONE.</p>
    pub compression_type: Option<String>,
    /// <p>Specifies JSON as object's input serialization format.</p>
    pub json: Option<JSONInput>,
    /// <p>Specifies Parquet as object's input serialization format.</p>
    pub parquet: Option<ParquetInput>,
}

/// <p>Container for Parquet.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct ParquetInput {}

/// <p>Specifies JSON as object's input serialization format.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct JSONInput {
    /// <p>The type of JSON. Valid values: Document, Lines.</p>
    pub type_: Option<String>,
}

/// <p>Describes how an uncompressed comma-separated values (CSV)-formatted input object is formatted.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct CSVInput {
    /// <p>Specifies that CSV field values may contain quoted record delimiters and such records should be allowed. Default value is FALSE. Setting this value to TRUE may lower performance.</p>
    pub allow_quoted_record_delimiter: Option<bool>,
    /// <p>A single character used to indicate that a row should be ignored when the character is present at the start of that row. You can specify any character to indicate a comment line.</p>
    pub comments: Option<String>,
    /// <p>A single character used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub field_delimiter: Option<String>,
    /// <p><p>Describes the first line of input. Valid values are:</p> <ul> <li> <p> <code>NONE</code>: First line is not a header.</p> </li> <li> <p> <code>IGNORE</code>: First line is a header, but you can&#39;t use the header values to indicate the column in an expression. You can use column position (such as _1, <em>2, …) to indicate the column (<code>SELECT s.</em>1 FROM OBJECT s</code>).</p> </li> <li> <p> <code>Use</code>: First line is a header, and you can use the header value to identify a column in an expression (<code>SELECT &quot;name&quot; FROM OBJECT</code>). </p> </li> </ul></p>
    pub file_header_info: Option<String>,
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p> <p>Type: String</p> <p>Default: <code>"</code> </p> <p>Ancestors: <code>CSV</code> </p>
    pub quote_character: Option<String>,
    /// <p>A single character used for escaping the quotation mark character inside an already escaped value. For example, the value """ a , b """ is parsed as " a , b ".</p>
    pub quote_escape_character: Option<String>,
    /// <p>A single character used to separate individual records in the input. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub record_delimiter: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct OutputSerialization {
    /// <p>Describes the serialization of CSV-encoded Select results.</p>
    pub csv: Option<CSVOutput>,
    /// <p>Specifies JSON as request's output serialization format.</p>
    pub json: Option<JSONOutput>,
}

/// <p>Container for specifying if periodic <code>QueryProgress</code> messages should be sent.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct RequestProgress {
    /// <p>Specifies whether periodic QueryProgress frames should be sent. Valid values: TRUE, FALSE. Default value: FALSE.</p>
    pub enabled: Option<bool>,
}

/// <p>Specifies the byte range of the object to get the records from. A record is processed when its first byte is contained by the range. This parameter is optional, but when specified, it must not be empty. See RFC 2616, Section 14.35.1 about how to specify the start and end of the range.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct ScanRange {
    /// <p>Specifies the end of the byte range. This parameter is optional. Valid values: non-negative integers. The default value is one less than the size of the object being queried. If only the End parameter is supplied, it is interpreted to mean scan the last N bytes of the file. For example, <code>&lt;scanrange&gt;&lt;end&gt;50&lt;/end&gt;&lt;/scanrange&gt;</code> means scan the last 50 bytes.</p>
    pub end: Option<i64>,
    /// <p>Specifies the start of the byte range. This parameter is optional. Valid values: non-negative integers. The default value is 0. If only start is supplied, it means scan from that point to the end of the file.For example; <code>&lt;scanrange&gt;&lt;start&gt;50&lt;/start&gt;&lt;/scanrange&gt;</code> means scan from byte 50 until the end of the file.</p>
    pub start: Option<i64>,
}

#[derive(Debug, Default)]
pub struct SelectObjectContentOutput {
    /// <p>The array of results.</p>
    pub payload: Option<EventStream<SelectObjectContentEventStreamItem>>,
}

// This is the trait that informs Serde how to deserialize MyMap.
impl<'de> de::Deserialize<'de> for SelectObjectContentOutput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        let byte_stream = ByteStream::from(value.to_string().as_bytes().to_vec());

        let http_response = HttpResponse {
            status: http::StatusCode::OK,
            body: byte_stream,
            headers: HeaderMap::default(),
        };

        let event_stream = EventStream::new(http_response);

        Ok(match value {
            serde_json::Value::Null => SelectObjectContentOutput {
                payload: None
            },
            _ => SelectObjectContentOutput {
                payload: Some(event_stream)
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SelectObjectContentEventStreamItem {
    /// <p>The Continuation Event.</p>
    Cont(ContinuationEvent),
    /// <p>The End Event.</p>
    End(EndEvent),
    /// <p>The Progress Event.</p>
    Progress(ProgressEvent),
    /// <p>The Records Event.</p>
    Records(RecordsEvent),
    /// <p>The Stats Event.</p>
    Stats(StatsEvent),
}

impl DeserializeEvent for SelectObjectContentEventStreamItem {
    fn deserialize_event(event_type: &str, data: &[u8]) -> Result<Self, RusotoError<()>> {
        let deserialized = match event_type {
            "Cont" => {
                let reader = EventReader::new(data.as_ref());
                let mut stack = XmlResponse::new(reader.into_iter().peekable());
                find_start_element(&mut stack);
                SelectObjectContentEventStreamItem::Cont(ContinuationEvent {})
            }
            "End" => {
                let reader = EventReader::new(data.as_ref());
                let mut stack = XmlResponse::new(reader.into_iter().peekable());
                find_start_element(&mut stack);
                SelectObjectContentEventStreamItem::End(EndEvent {})
            }
            "Progress" => {
                let reader = EventReader::new(data.as_ref());
                let mut stack = XmlResponse::new(reader.into_iter().peekable());
                find_start_element(&mut stack);
                SelectObjectContentEventStreamItem::Progress(ProgressEvent {
                    details: Some(ProgressDeserializer::deserialize("Details", &mut stack)?),
                })
            }
            "Records" => {
                let reader = EventReader::new(data.as_ref());
                let mut stack = XmlResponse::new(reader.into_iter().peekable());
                find_start_element(&mut stack);
                SelectObjectContentEventStreamItem::Records(RecordsEvent {
                    payload: Some(data.to_owned().into()),
                })
            }
            "Stats" => {
                let reader = EventReader::new(data.as_ref());
                let mut stack = XmlResponse::new(reader.into_iter().peekable());
                find_start_element(&mut stack);
                SelectObjectContentEventStreamItem::Stats(StatsEvent {
                    details: Some(StatsDeserializer::deserialize("Details", &mut stack)?),
                })
            }
            _ => Err(RusotoError::ParseError(format!(
                "Invalid event type: {}",
                event_type
            )))?,
        };
        Ok(deserialized)
    }
}

#[allow(dead_code)]
struct ProgressDeserializer;
impl ProgressDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Progress, XmlParseError> {
        deserialize_elements::<_, Progress, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "BytesProcessed" => {
                    obj.bytes_processed = Some(BytesProcessedDeserializer::deserialize(
                        "BytesProcessed",
                        stack,
                    )?);
                }
                "BytesReturned" => {
                    obj.bytes_returned = Some(BytesReturnedDeserializer::deserialize(
                        "BytesReturned",
                        stack,
                    )?);
                }
                "BytesScanned" => {
                    obj.bytes_scanned = Some(BytesScannedDeserializer::deserialize(
                        "BytesScanned",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct BytesProcessedDeserializer;
impl BytesProcessedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct BytesReturnedDeserializer;
impl BytesReturnedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct BytesScannedDeserializer;
impl BytesScannedDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}

#[allow(dead_code)]
struct StatsDeserializer;
impl StatsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Stats, XmlParseError> {
        deserialize_elements::<_, Stats, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "BytesProcessed" => {
                    obj.bytes_processed = Some(BytesProcessedDeserializer::deserialize(
                        "BytesProcessed",
                        stack,
                    )?);
                }
                "BytesReturned" => {
                    obj.bytes_returned = Some(BytesReturnedDeserializer::deserialize(
                        "BytesReturned",
                        stack,
                    )?);
                }
                "BytesScanned" => {
                    obj.bytes_scanned = Some(BytesScannedDeserializer::deserialize(
                        "BytesScanned",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// <p>Describes how uncompressed comma-separated values (CSV)-formatted results are formatted.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct CSVOutput {
    /// <p>The value used to separate individual fields in a record. You can specify an arbitrary delimiter.</p>
    pub field_delimiter: Option<String>,
    /// <p>A single character used for escaping when the field delimiter is part of the value. For example, if the value is <code>a, b</code>, Amazon S3 wraps this field value in quotation marks, as follows: <code>" a , b "</code>.</p>
    pub quote_character: Option<String>,
    /// <p>The single character used for escaping the quote character inside an already escaped value.</p>
    pub quote_escape_character: Option<String>,
    /// <p><p>Indicates whether to use quotation marks around output fields. </p> <ul> <li> <p> <code>ALWAYS</code>: Always use quotation marks for output fields.</p> </li> <li> <p> <code>ASNEEDED</code>: Use quotation marks for output fields when needed.</p> </li> </ul></p>
    pub quote_fields: Option<String>,
    /// <p>A single character used to separate individual records in the output. Instead of the default value, you can specify an arbitrary delimiter.</p>
    pub record_delimiter: Option<String>,
}

/// <p>Specifies JSON as request's output serialization format.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct JSONOutput {
    /// <p>The value used to separate individual records in the output. If no value is specified, Amazon S3 uses a newline character ('\n').</p>
    pub record_delimiter: Option<String>,
}

/// <p><p/></p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct ContinuationEvent {}

/// <p>A message that indicates the request is complete and no more messages will be sent. You should not assume that the request is complete until the client receives an <code>EndEvent</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct EndEvent {}

/// <p>This data type contains information about the progress event of an operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct ProgressEvent {
    /// <p>The Progress event details.</p>
    pub details: Option<Progress>,
}

/// <p>This data type contains information about progress of an operation.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Progress {
    /// <p>The current number of uncompressed object bytes processed.</p>
    pub bytes_processed: Option<i64>,
    /// <p>The current number of bytes of records payload data returned.</p>
    pub bytes_returned: Option<i64>,
    /// <p>The current number of object bytes scanned.</p>
    pub bytes_scanned: Option<i64>,
}

/// <p>The container for the records event.</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecordsEvent {
    /// <p>The byte array of partial, one or more result records.</p>
    pub payload: Option<bytes::Bytes>,
}

/// <p>Container for the Stats Event.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct StatsEvent {
    /// <p>The Stats event details.</p>
    pub details: Option<Stats>,
}

/// <p>Container for the stats details.</p>
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Stats {
    /// <p>The total number of uncompressed object bytes processed.</p>
    pub bytes_processed: Option<i64>,
    /// <p>The total number of bytes of records payload data returned.</p>
    pub bytes_returned: Option<i64>,
    /// <p>The total number of object bytes scanned.</p>
    pub bytes_scanned: Option<i64>,
}

pub struct SelectObjectContentRequestSerializer;
impl SelectObjectContentRequestSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &SelectObjectContentRequest,
        xmlns: &str,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name).default_ns(xmlns))?;
        ExpressionSerializer::serialize(&mut writer, "Expression", &obj.expression)?;
        ExpressionTypeSerializer::serialize(&mut writer, "ExpressionType", &obj.expression_type)?;
        InputSerializationSerializer::serialize(
            &mut writer,
            "InputSerialization",
            &obj.input_serialization,
        )?;
        OutputSerializationSerializer::serialize(
            &mut writer,
            "OutputSerialization",
            &obj.output_serialization,
        )?;
        if let Some(ref value) = obj.request_progress {
            &RequestProgressSerializer::serialize(&mut writer, "RequestProgress", value)?;
        }
        if let Some(ref value) = obj.scan_range {
            &ScanRangeSerializer::serialize(&mut writer, "ScanRange", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct RequestProgressSerializer;
impl RequestProgressSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &RequestProgress,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.enabled {
            write_characters_element(writer, "Enabled", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ScanRangeSerializer;
impl ScanRangeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ScanRange,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.end {
            write_characters_element(writer, "End", &value.to_string())?;
        }
        if let Some(ref value) = obj.start {
            write_characters_element(writer, "Start", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct ExpressionSerializer;
impl ExpressionSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

pub struct ExpressionTypeSerializer;
impl ExpressionTypeSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &String,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        write_characters_element(writer, name, obj)
    }
}

pub struct InputSerializationSerializer;
impl InputSerializationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &InputSerialization,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.csv {
            &CSVInputSerializer::serialize(&mut writer, "CSV", value)?;
        }
        if let Some(ref value) = obj.compression_type {
            write_characters_element(writer, "CompressionType", &value.to_string())?;
        }
        if let Some(ref value) = obj.json {
            &JSONInputSerializer::serialize(&mut writer, "JSON", value)?;
        }
        if let Some(ref value) = obj.parquet {
            &ParquetInputSerializer::serialize(&mut writer, "Parquet", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
pub struct ParquetInputSerializer;
impl ParquetInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &ParquetInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct CSVInputSerializer;
impl CSVInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CSVInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.allow_quoted_record_delimiter {
            write_characters_element(writer, "AllowQuotedRecordDelimiter", &value.to_string())?;
        }
        if let Some(ref value) = obj.comments {
            write_characters_element(writer, "Comments", &value.to_string())?;
        }
        if let Some(ref value) = obj.field_delimiter {
            write_characters_element(writer, "FieldDelimiter", &value.to_string())?;
        }
        if let Some(ref value) = obj.file_header_info {
            write_characters_element(writer, "FileHeaderInfo", &value.to_string())?;
        }
        if let Some(ref value) = obj.quote_character {
            write_characters_element(writer, "QuoteCharacter", &value.to_string())?;
        }
        if let Some(ref value) = obj.quote_escape_character {
            write_characters_element(writer, "QuoteEscapeCharacter", &value.to_string())?;
        }
        if let Some(ref value) = obj.record_delimiter {
            write_characters_element(writer, "RecordDelimiter", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct JSONInputSerializer;
impl JSONInputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &JSONInput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.type_ {
            write_characters_element(writer, "Type", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct OutputSerializationSerializer;
impl OutputSerializationSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &OutputSerialization,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.csv {
            &CSVOutputSerializer::serialize(&mut writer, "CSV", value)?;
        }
        if let Some(ref value) = obj.json {
            &JSONOutputSerializer::serialize(&mut writer, "JSON", value)?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct CSVOutputSerializer;
impl CSVOutputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &CSVOutput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.field_delimiter {
            write_characters_element(writer, "FieldDelimiter", &value.to_string())?;
        }
        if let Some(ref value) = obj.quote_character {
            write_characters_element(writer, "QuoteCharacter", &value.to_string())?;
        }
        if let Some(ref value) = obj.quote_escape_character {
            write_characters_element(writer, "QuoteEscapeCharacter", &value.to_string())?;
        }
        if let Some(ref value) = obj.quote_fields {
            write_characters_element(writer, "QuoteFields", &value.to_string())?;
        }
        if let Some(ref value) = obj.record_delimiter {
            write_characters_element(writer, "RecordDelimiter", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}

pub struct JSONOutputSerializer;
impl JSONOutputSerializer {
    #[allow(unused_variables, warnings)]
    pub fn serialize<W>(
        mut writer: &mut EventWriter<W>,
        name: &str,
        obj: &JSONOutput,
    ) -> Result<(), xml::writer::Error>
    where
        W: Write,
    {
        writer.write(xml::writer::XmlEvent::start_element(name))?;
        if let Some(ref value) = obj.record_delimiter {
            write_characters_element(writer, "RecordDelimiter", &value.to_string())?;
        }
        writer.write(xml::writer::XmlEvent::end_element())
    }
}
