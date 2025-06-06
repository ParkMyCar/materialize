// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use std::hint::black_box;

use criterion::{Criterion, Throughput};
use mz_interchange::protobuf::{DecodedDescriptors, Decoder};
use mz_ore::cast::CastFrom;
use prost::Message;

use self::generated::benchmark::{Connector, Record, Value};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/benchproto.rs"));
}

pub fn bench_protobuf(c: &mut Criterion) {
    let value = Value {
        l_orderkey: 155_190,
        l_suppkey: 7706,
        l_linenumber: 1,
        l_quantity: 17.0,
        l_extendedprice: 21168.23,
        l_discount: 0.04,
        l_tax: 0.02,
        l_returnflag: "N".into(),
        l_linestatus: "O".into(),
        l_shipdate: 9567,
        l_commitdate: 9537,
        l_receiptdate: 9537,
        l_shipinstruct: "DELIVER IN PERSON".into(),
        l_shipmode: "TRUCK".into(),
        l_comment: "egular courts above the".into(),
        ..Default::default()
    };

    let connector = Connector {
        version: "0.9.5.Final".into(),
        connector: "mysql".into(),
        name: "tcph".into(),
        server_id: 0,
        ts_sec: 0,
        gtid: "".into(),
        file: "binlog.000004".into(),
        pos: 951_896_181,
        row: 0,
        snapshot: true,
        thread: 0,
        db: "tcph".into(),
        table: "lineitem".into(),
        query: "".into(),
    };

    let record = Record {
        tcph_tcph_lineitem_value: Some(value),
        source: Some(connector),
        op: "c".into(),
        ts_ms: 1_560_886_948_093,
    };

    let buf = record.encode_to_vec();
    let len = u64::cast_from(buf.len());
    let mut decoder = Decoder::new(
        DecodedDescriptors::from_bytes(
            &include_bytes!(concat!(env!("OUT_DIR"), "/file_descriptor_set.pb"))[..],
            ".benchmark.Record".to_string(),
        )
        .unwrap(),
        false,
    )
    .unwrap();

    let mut bg = c.benchmark_group("protobuf");
    bg.throughput(Throughput::Bytes(len));
    bg.bench_function("decode", move |b| {
        b.iter(|| black_box(decoder.decode(&buf).unwrap()))
    });
    bg.finish();
}
