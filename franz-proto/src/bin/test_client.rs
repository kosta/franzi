use byteorder::ByteOrder;
use bytes::BytesMut;
use franz_base::types::KafkaString;
use franz_base::{FromBytes, ToBytes};
use franz_proto::messages::api_versions::{ApiVersionsRequest2, ApiVersionsResponse2};
use franz_proto::messages::list_offsets::*;
use franz_proto::header::{RequestHeader, ResponseHeader};
use futures::future::Future;
use std::net::SocketAddr;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let fut = tokio::net::tcp::TcpStream::connect(&"127.0.0.1:9092".parse::<SocketAddr>().unwrap())
        .and_then(|tcp| {
            let req_header = RequestHeader {
                api_key: franz_proto::api_keys::ApiKey::ApiVersions as i16,
                api_version: 2,
                correlation_id: 42,
                client_id: Some(KafkaString(String::from("franzi").into())),
            };
            // eprintln!("Sending req_header {:?}", req_header);
            let req = ApiVersionsRequest2 {};
            let len = req_header.len_to_write() + req.len_to_write();
            let mut buf = BytesMut::with_capacity(4 + len);

            // write size (i32)
            ToBytes::write(&(len as i32), &mut buf);
            // write header
            req_header.write(&mut buf);
            // write request
            req.write(&mut buf);

            tokio::io::write_all(tcp, buf)
        })
        .and_then(|(tcp, mut buf)| {
            // make room for response
            buf.resize(4, 0);
            tokio::io::read_exact(tcp, buf)
        })
        .and_then(|(tcp, mut buf)| {
            // read i32 for size
            let len = byteorder::NetworkEndian::read_i32(buf.as_ref());
            eprintln!("Received response of len {:?}", len);
            buf.resize(len as usize, 0);
            tokio::io::read_exact(tcp, buf)
        })
        .map(move |(tcp, buf)| {
            let mut buf = std::io::Cursor::new(buf.freeze());
            // read header
            let header = ResponseHeader::read(&mut buf).expect("response header");
            eprintln!("Received response header {:?}", header);
            // read response
            let response = ApiVersionsResponse2::read(&mut buf).expect("response body");
            eprintln!("Took {:?}", start.elapsed());
            eprintln!(
                "Received response: error_code {:?}, throttle_time_ms: {:?}",
                response.error_code, response.throttle_time_ms
            );
            for api_version in &response.api_versions {
                eprintln!("{:?}", api_version);
            }
            tcp
        })
        .and_then(|tcp| {
            let req_header = RequestHeader {
                api_key: franz_proto::api_keys::ApiKey::ListOffsets as i16,
                api_version: 4,
                correlation_id: 43,
                client_id: Some(KafkaString(String::from("franzi").into())),
            };
            // eprintln!("Sending req_header {:?}", req_header);
            let req = ListOffsetsRequest4 {
                replica_id: -1,
                isolation_level: 0,
                topics: Some(vec![
                    ListOffsetsRequest4Topic {
                        topic: KafkaString(String::from("dcz_admitad_rawfeed").into()),
                        partitions: Some(vec![
                            ListOffsetsRequest4Partition {
                                partition: 0,
                                current_leader_epoch: -1, //?
                                timestamp: 0, //?
                            },
                        ]),
                    }
                ]),
            };
            let len = req_header.len_to_write() + req.len_to_write();
            let mut buf = BytesMut::with_capacity(4 + len);

            // write size (i32)
            ToBytes::write(&(len as i32), &mut buf);
            // write header
            req_header.write(&mut buf);
            // write request
            req.write(&mut buf);

            tokio::io::write_all(tcp, buf)
        })
        .and_then(|(tcp, mut buf)| {
            // make room for response
            buf.resize(4, 0);
            tokio::io::read_exact(tcp, buf)
        })
        .and_then(|(tcp, mut buf)| {
            // read i32 for size
            let len = byteorder::NetworkEndian::read_i32(buf.as_ref());
            eprintln!("Received response of len {:?}", len);
            buf.resize(len as usize, 0);
            tokio::io::read_exact(tcp, buf)
        })
        .map(|(_, buf)| {
            let mut buf = std::io::Cursor::new(buf.freeze());
            // read header
            let header = ResponseHeader::read(&mut buf).expect("listoffets response header");
            eprintln!("Received response header {:?}", header);
            // read response
            eprintln!("ListOffets response: {:?}", buf.get_ref().as_ref());
            let response = ListOffsetsResponse4::read(&mut buf).expect("listoffets response body");
            eprintln!(
                "Received ListOffets response: {:?}", response
            );
        })
        .map_err(|e| panic!("Error: {}", e));
    tokio::run(fut);
}
