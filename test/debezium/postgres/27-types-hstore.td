# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

#
# Make sure that the HSTORE type is replicated correctly
#

$ postgres-execute connection=postgres://postgres:postgres@postgres
CREATE EXTENSION IF NOT EXISTS hstore;
CREATE TABLE hstore_type (f1 hstore, f2 INTEGER PRIMARY KEY);
ALTER TABLE hstore_type REPLICA IDENTITY FULL;
INSERT INTO hstore_type VALUES (NULL, 0), ('a=>1'::hstore, 1);

$ schema-registry-wait topic=postgres.public.hstore_type

> CREATE CONNECTION IF NOT EXISTS csr_conn TO CONFLUENT SCHEMA REGISTRY (
    URL '${testdrive.schema-registry-url}'
  );

> CREATE CONNECTION IF NOT EXISTS kafka_conn TO KAFKA (BROKER '${testdrive.kafka-addr}', SECURITY PROTOCOL PLAINTEXT);

> CREATE SOURCE hstore_type
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'postgres.public.hstore_type');

> CREATE TABLE hstore_type_tbl FROM SOURCE hstore_type (REFERENCE "postgres.public.hstore_type")
  FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_conn
  ENVELOPE DEBEZIUM;

> SELECT f1, f1 ->> 'a', pg_typeof(f1) FROM hstore_type_tbl;
<null> <null> jsonb
"{\"a\":\"1\"}" 1 jsonb
