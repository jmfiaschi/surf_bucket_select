use rusoto_core::event_stream::{DeserializeEvent};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse, find_start_element, deserialize_elements, deserialize_primitive, skip_tree};
use rusoto_core::{RusotoError};
use rusoto_s3::{ContinuationEvent, EndEvent, Progress, ProgressEvent, RecordsEvent, Stats, StatsEvent};
use xml::reader::EventReader;
use std::str::FromStr;

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
