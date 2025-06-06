# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

$ set-arg-default single-replica-cluster=quickstart

#
# Make sure that the SNAPSHOT=FALSE option is observed
#

$ set-regex match=\d{13} replacement=<TIMESTAMP>
$ set int={"type": "record", "name": "field_int", "fields": [ {"name": "f1", "type": "int"} ] }

$ kafka-create-topic topic=tail-without-snapshot

$ kafka-ingest format=avro topic=tail-without-snapshot schema=${int} timestamp=1
{"f1": 123}
{"f1": 234}
{"f1": 345}

> CREATE CONNECTION kafka_conn
  TO KAFKA (BROKER '${testdrive.kafka-addr}', SECURITY PROTOCOL PLAINTEXT);

> CREATE SOURCE tail_without_snapshot
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-tail-without-snapshot-${testdrive.seed}')

> CREATE TABLE tail_without_snapshot_tbl FROM SOURCE tail_without_snapshot (REFERENCE "testdrive-tail-without-snapshot-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${int}'
  ENVELOPE NONE

> SELECT * FROM tail_without_snapshot_tbl;
123
234
345

> BEGIN

> DECLARE c CURSOR FOR SUBSCRIBE tail_without_snapshot_tbl WITH (SNAPSHOT = FALSE);

> FETCH 1 FROM c WITH (timeout = '2s')

$ kafka-ingest format=avro topic=tail-without-snapshot schema=${int} timestamp=1
{"f1": 567}

> FETCH 1 FROM c WITH (timeout = '60s')
<TIMESTAMP> 1 567
