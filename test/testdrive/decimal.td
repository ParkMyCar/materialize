# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

$ set-arg-default single-replica-cluster=quickstart

# This test exercises decimals at the boundary (e.g., by sending them through
# pgwire). Operations on decimals are more thoroughly tested in decimal.slt.

$ set schema={
    "name": "row",
    "type": "record",
    "fields": [
      {
        "name": "a",
        "type": {
          "type": "bytes",
          "scale": 2,
          "precision": 15,
          "logicalType": "decimal"
        }
      }
    ]
  }

$ kafka-create-topic topic=data

$ kafka-ingest format=avro topic=data schema=${schema}
{"a": [7, 2]}
{"a": [186]}

> CREATE CONNECTION kafka_conn
  TO KAFKA (BROKER '${testdrive.kafka-addr}', SECURITY PROTOCOL PLAINTEXT);

> CREATE SOURCE data
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-data-${testdrive.seed}')

> CREATE TABLE data_tbl FROM SOURCE data (REFERENCE "testdrive-data-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${schema}'

> SELECT * FROM data_tbl
17.94
-0.7
