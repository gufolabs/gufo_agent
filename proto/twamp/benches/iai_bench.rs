// ------------------------------------------------------------------------
// Gufo Agent: iai benches for TWAMP protocol implementation
// ------------------------------------------------------------------------
// Copyright (C) 2023, Gufo Labs
// See LICENSE.md for details
// ------------------------------------------------------------------------

use bytes::BytesMut;
use frame::{FrameReader, FrameWriter};
use iai_callgrind::{black_box, main};
use twamp::{NtpTimeStamp, TestRequest, TestResponse};

//Utc.with_ymd_and_hms(2021, 2, 12, 10, 0, 2).unwrap()
const REF_TIMESTAMP: u64 = 16415849486212923392;

#[export_name = "bench::get_buffer"]
#[inline(never)]
pub fn get_buffer() -> BytesMut {
    BytesMut::with_capacity(1024)
}

pub fn get_timestamp() -> NtpTimeStamp {
    REF_TIMESTAMP.into()
}

#[inline(never)]
pub fn write_test_request() {
    let msg = TestRequest {
        seq: 0x01020304,
        timestamp: get_timestamp(),
        err_estimate: 0,
        pad_to: 64,
    };
    let mut buf = get_buffer();
    msg.write_bytes(black_box(&mut buf));
}

#[inline(never)]
pub fn parse_test_request() {
    static TEST_REQUEST: &[u8] = &[
        0x00, 0x00, 0x04, 0x00, // Sequence, 4 octets
        0xe3, 0xd0, 0xd0, 0x20, 0x00, 0x00, 0x00, 0x00, // Timestamp, 8 octets
        0x00, 0x0f, // Err estimate, 2 octets
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding, 6 octets, up to 20 octets
    ];
    let mut buf = BytesMut::from(TEST_REQUEST);
    TestRequest::parse(&mut buf);
}

#[inline(never)]
pub fn write_test_response() {
    let msg = TestResponse {
        seq: 1024,
        timestamp: get_timestamp(),
        err_estimate: 15,
        recv_timestamp: get_timestamp(),
        sender_seq: 1025,
        sender_timestamp: get_timestamp(),
        sender_err_estimate: 14,
        sender_ttl: 250,
        pad_to: 50,
    };
    let mut buf = get_buffer();
    msg.write_bytes(black_box(&mut buf));
}

#[inline(never)]
pub fn parse_test_response() {
    static TEST_RESPONSE: &[u8] = &[
        0x00, 0x00, 0x04, 0x00, // Sequence, 4 octets
        0xe3, 0xd0, 0xd0, 0x22, 0x00, 0x00, 0x00, 0x00, // Timestamp, 8 octets
        0x00, 0x0f, // Err estimate, 2 octets
        0x00, 0x00, // MBZ, 2 octets
        0xe3, 0xd0, 0xd0, 0x21, 0x00, 0x00, 0x00, 0x00, // Receive timestamp, 8 octets
        0x00, 0x00, 0x04, 0x01, // Sender Sequence, 4 octets
        0xe3, 0xd0, 0xd0, 0x20, 0x00, 0x00, 0x00, 0x00, // Sender Timestamp, 8 octets
        0x00, 0x0e, // Sender Err estimate, 2 octets
        0x00, 0x00, // MBZ, 2 octets
        0xfa, // Sender TTL
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Padding, 9 octets
    ];
    let mut buf = BytesMut::from(TEST_RESPONSE);
    TestResponse::parse(&mut buf);
}

main!(
    callgrind_args = "toggle-collect=bench::bench::get_buffer";
    functions = write_test_request, parse_test_request, write_test_response, parse_test_response
);
