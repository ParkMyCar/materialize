# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

# For Debezium versions <1.5, Materialize had no way of determining the complete order
# of messages coming from a Postgres source. (Details in database-issues#1744.) We fixed this upstream
# in Debezium (details in DBZ-2911) by adding a new "sequence" field to the source
# metadata. This test uses that new sequence field to show that we can now handle messages
# that seem to go backwards in time (by LSN).

$ nop

# [btv] uncomment if we bring back classic debezium mode
#
# $ set-arg-default single-replica-cluster=quickstart
#
# $ set pg-dbz-schema={
#     "type": "record",
#     "name": "envelope",
#     "fields": [
#       {
#         "name": "before",
#         "type": [
#           {
#             "name": "row",
#             "type": "record",
#             "fields": [
#               {"name": "val", "type": "string"}
#             ]
#           },
#           "null"
#         ]
#       },
#       { "name": "after", "type": ["row", "null"] },
#       { "name": "op", "type": "string" },
#       {
#         "name": "source",
#         "type": {
#           "type": "record",
#           "name": "Source",
#           "namespace": "whatever",
#           "fields": [
#             {
#               "name": "snapshot",
#               "type": [
#                 {
#                   "type": "string",
#                   "connect.version": 1,
#                   "connect.parameters": {
#                     "allowed": "true,last,false"
#                   },
#                   "connect.default": "false",
#                   "connect.name": "io.debezium.data.Enum"
#                 },
#                 "null"
#               ],
#               "default": "false"
#             },
#             {
#               "name": "lsn",
#               "type": ["long", "null"]
#             },
#             {
#               "name": "sequence",
#               "type": ["string", "null"]
#             }
#           ]
#         }
#       }
#     ]
#   }

# $ kafka-create-topic topic=pg-dbz-data partitions=1

# $ kafka-ingest format=avro topic=pg-dbz-data schema=${pg-dbz-schema} timestamp=1
# {"before": null, "after": {"row":{"val": "foo"}}, "source": {"lsn": {"long": 3}, "sequence": {"string": "[\"1\", \"3\"]"}, "snapshot": {"string": "false"}}, "op": "c"}
# {"before": {"row":{"val": "foo"}}, "after": {"row":{"val": "bar"}}, "source": {"lsn": {"long": 5}, "sequence": {"string": "[\"1\", \"5\"]"}, "snapshot": {"string": "false"}}, "op": "u"}
# {"before": {"row":{"val": "bar"}}, "after": {"row":{"val": "baz"}}, "source": {"lsn": {"long": 7}, "sequence": {"string": "[\"1\", \"7\"]"}, "snapshot": {"string": "false"}}, "op": "u"}

# {"before": null, "after": {"row":{"val": "hello,"}}, "source": {"lsn": {"long": 4}, "sequence": {"string": "[\"8\", \"4\"]"}, "snapshot": {"string": "false"}}, "op": "c"}
# {"before": null, "after": {"row":{"val": "world!"}}, "source": {"lsn": {"long": 6}, "sequence": {"string": "[\"8\", \"6\"]"}, "snapshot": {"string": "false"}}, "op": "c"}

# {"before": null, "after": {"row":{"val": "too late"}}, "source": {"lsn": {"long": 1}, "sequence": {"string": "[\"1\", \"1\"]"}, "snapshot": {"string": "false"}}, "op": "c"}

# > CREATE CONNECTION kafka_conn
#   TO KAFKA (BROKER '${testdrive.kafka-addr}', SECURITY PROTOCOL PLAINTEXT);

# > CREATE SOURCE pg_dbz
#   IN CLUSTER ${arg.single-replica-cluster}
#   FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-pg-dbz-data-${testdrive.seed}')

# > CREATE TABLE pg_dbz_tbl FROM SOURCE pg_dbz (REFERENCE "testdrive-pg-dbz-data-${testdrive.seed}")
#   FORMAT AVRO USING SCHEMA '${pg-dbz-schema}'
#   ENVELOPE DEBEZIUM

# > SELECT * FROM pg_dbz_tbl
# val
# ---
# baz
# hello,
# world!
