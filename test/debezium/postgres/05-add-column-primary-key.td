# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

#
# Test that adding a primary key as a completely new column does not mess things up
# Currently rejected by the schema registry on the Debezium side. Replication stops
#

# TODO: Reenable when database-issues#2047 is fixed
$ skip-if
SELECT true

$ postgres-execute connection=postgres://postgres:postgres@postgres
CREATE TABLE alter_add_column_primary_key (f1 INTEGER);
INSERT INTO alter_add_column_primary_key VALUES (123);

> CREATE CONNECTION IF NOT EXISTS csr_conn TO CONFLUENT SCHEMA REGISTRY (
    URL '${testdrive.schema-registry-url}'
  );

> CREATE CONNECTION IF NOT EXISTS kafka_conn TO KAFKA (BROKER '${testdrive.kafka-addr}', SECURITY PROTOCOL PLAINTEXT);

> CREATE SOURCE alter_add_column_primary_key
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'postgres.public.alter_add_column_primary_key');

> CREATE TABLE alter_add_column_primary_key_tbl FROM SOURCE alter_add_column_primary_key (REFERENCE "postgres.public.alter_add_column_primary_key")
  FORMAT AVRO USING CONFLUENT SCHEMA REGISTRY CONNECTION csr_conn
  ENVELOPE DEBEZIUM;

> SELECT * FROM alter_add_column_primary_key_tbl;
123

$ postgres-execute connection=postgres://postgres:postgres@postgres
CREATE SEQUENCE pk_sequence;
ALTER TABLE alter_add_column_primary_key ADD COLUMN pk_column INTEGER PRIMARY KEY DEFAULT nextval('pk_sequence');
INSERT INTO alter_add_column_primary_key VALUES (123,2);

UPDATE alter_add_column_primary_key SET f1 = f1 * 10 WHERE pk_column = 1;
UPDATE alter_add_column_primary_key SET f1 = f1 * 100 WHERE pk_column = 2;

> SELECT * FROM alter_add_column_primary_key_tbl;
123
