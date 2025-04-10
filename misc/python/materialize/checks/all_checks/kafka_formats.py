# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.
from textwrap import dedent

from materialize.checks.actions import Testdrive
from materialize.checks.checks import Check, externally_idempotent

PROTOBUF = dedent(
    """
    $ file-append path=test.proto
    syntax = "proto3";

    message Key {
        string key1 = 1;
        string key2 = 2;
    }

    message Value {
        string value1 = 1;
        string value2 = 2;
    }

    $ protobuf-compile-descriptors inputs=test.proto output=test.proto set-var=test-schema
    """
)


@externally_idempotent(False)
class KafkaFormats(Check):
    def initialize(self) -> Testdrive:
        return Testdrive(
            PROTOBUF
            + dedent(
                """
                > CREATE CLUSTER kafka_formats REPLICAS (kafka_formats_r1 (SIZE '4'))

                > SET cluster=kafka_formats

                $ kafka-create-topic topic=format-bytes

                $ kafka-ingest format=bytes key-format=bytes key-terminator=: topic=format-bytes
                key1A,key1B:value1A,value1B

                $ kafka-create-topic topic=format-protobuf partitions=1

                $ kafka-ingest topic=format-protobuf
                  key-format=protobuf key-descriptor-file=test.proto key-message=Key
                  format=protobuf descriptor-file=test.proto message=Value
                {"key1": "key1A", "key2": "key1B"} {"value1": "value1A", "value2": "value1B"}

                > CREATE SOURCE format_bytes1_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_bytes1 FROM SOURCE format_bytes1_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT BYTES
                  VALUE FORMAT BYTES
                  ENVELOPE UPSERT

                > CREATE SOURCE format_text1_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_text1 FROM SOURCE format_text1_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT TEXT
                  VALUE FORMAT TEXT
                  ENVELOPE UPSERT

                > CREATE SOURCE format_csv1_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_csv1 (key1, key2, value1, value2)
                  FROM SOURCE format_csv1_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT CSV WITH 2 COLUMNS DELIMITED BY ','
                  VALUE FORMAT CSV WITH 2 COLUMNS DELIMITED BY ','
                  ENVELOPE UPSERT

                > CREATE SOURCE format_regex1_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_regex1 (key1, key2, value1, value2)
                  FROM SOURCE format_regex1_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT REGEX '(?P<key1>[^,]+),(?P<key2>\\w+)'
                  VALUE FORMAT REGEX '(?P<value1>[^,]+),(?P<value2>\\w+)'
                  ENVELOPE UPSERT

                > CREATE SOURCE format_protobuf1_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-protobuf-${testdrive.seed}')
                > CREATE TABLE format_protobuf1 FROM SOURCE format_protobuf1_src (REFERENCE "testdrive-format-protobuf-${testdrive.seed}")
                  KEY FORMAT PROTOBUF MESSAGE '.Key' USING SCHEMA '${test-schema}'
                  VALUE FORMAT PROTOBUF MESSAGE '.Value' USING SCHEMA '${test-schema}'
                  INCLUDE KEY
                  ENVELOPE UPSERT
            """
            )
        )

    def manipulate(self) -> list[Testdrive]:
        return [
            Testdrive(PROTOBUF + dedent(s))
            for s in [
                """
                > SET cluster=kafka_formats

                > CREATE SOURCE format_bytes2_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_bytes2 FROM SOURCE format_bytes2_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT BYTES
                  VALUE FORMAT BYTES
                  ENVELOPE UPSERT

                > CREATE SOURCE format_text2_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_text2 FROM SOURCE format_text2_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT TEXT
                  VALUE FORMAT TEXT
                  ENVELOPE UPSERT

                > CREATE SOURCE format_csv2_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_csv2 (key1, key2, value1, value2)
                  FROM SOURCE format_csv2_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT CSV WITH 2 COLUMNS DELIMITED BY ','
                  VALUE FORMAT CSV WITH 2 COLUMNS DELIMITED BY ','
                  ENVELOPE UPSERT

                > CREATE SOURCE format_regex2_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-bytes-${testdrive.seed}')
                > CREATE TABLE format_regex2
                  FROM SOURCE format_regex2_src (REFERENCE "testdrive-format-bytes-${testdrive.seed}")
                  KEY FORMAT REGEX '(?P<key1>[^,]+),(?P<key2>\\w+)'
                  VALUE FORMAT REGEX '(?P<value1>[^,]+),(?P<value2>\\w+)'
                  ENVELOPE UPSERT

                > CREATE SOURCE format_protobuf2_src
                  IN CLUSTER kafka_formats
                  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-format-protobuf-${testdrive.seed}')
                > CREATE TABLE format_protobuf2 FROM SOURCE format_protobuf2_src (REFERENCE "testdrive-format-protobuf-${testdrive.seed}")
                  KEY FORMAT PROTOBUF MESSAGE '.Key' USING SCHEMA '${test-schema}'
                  VALUE FORMAT PROTOBUF MESSAGE '.Value' USING SCHEMA '${test-schema}'
                  INCLUDE KEY
                  ENVELOPE UPSERT

                $ kafka-ingest format=bytes key-format=bytes key-terminator=: topic=format-bytes
                key2A,key2B:value2A,value2B

                $ kafka-ingest topic=format-protobuf
                  key-format=protobuf key-descriptor-file=test.proto key-message=Key
                  format=protobuf descriptor-file=test.proto message=Value
                {"key1": "key2A", "key2": "key2B"} {"value1": "value2A", "value2": "value2B"}
                """,
                """
                $ kafka-ingest format=bytes key-format=bytes key-terminator=: topic=format-bytes
                key3A,key3B:value3A,value3B

                $ kafka-ingest topic=format-protobuf
                  key-format=protobuf key-descriptor-file=test.proto key-message=Key
                  format=protobuf descriptor-file=test.proto message=Value
                {"key1": "key3A", "key2": "key3B"} {"value1": "value3A", "value2": "value3B"}
                """,
            ]
        ]

    def validate(self) -> Testdrive:
        return Testdrive(
            dedent(
                r"""
                > SELECT COUNT(*) FROM format_bytes1
                3

                > SELECT * FROM format_text1
                key1A,key1B value1A,value1B
                key2A,key2B value2A,value2B
                key3A,key3B value3A,value3B

                > SELECT * FROM format_csv1
                key1A key1B value1A value1B
                key2A key2B value2A value2B
                key3A key3B value3A value3B

                > SELECT * FROM format_regex1
                key1A key1B value1A value1B
                key2A key2B value2A value2B
                key3A key3B value3A value3B

                > SELECT * FROM format_protobuf1
                key1A key1B value1A value1B
                key2A key2B value2A value2B
                key3A key3B value3A value3B

                > SELECT * FROM format_text2
                key1A,key1B value1A,value1B
                key2A,key2B value2A,value2B
                key3A,key3B value3A,value3B

                > SELECT * FROM format_csv2
                key1A key1B value1A value1B
                key2A key2B value2A value2B
                key3A key3B value3A value3B

                > SELECT * FROM format_regex2
                key1A key1B value1A value1B
                key2A key2B value2A value2B
                key3A key3B value3A value3B

                > SELECT * FROM format_protobuf2
                key1A key1B value1A value1B
                key2A key2B value2A value2B
                key3A key3B value3A value3B

                $ set-regex match=testdrive-format-bytes-\d+ replacement=<TOPIC>

                >[version>=14000] SHOW CREATE SOURCE format_bytes1_src;
                materialize.public.format_bytes1_src "CREATE SOURCE materialize.public.format_bytes1_src\nIN CLUSTER kafka_formats\nFROM KAFKA CONNECTION materialize.public.kafka_conn (TOPIC = '<TOPIC>')\nEXPOSE PROGRESS AS materialize.public.format_bytes1_src_progress;"

                >[version<14000] SHOW CREATE SOURCE format_bytes1_src;
                materialize.public.format_bytes1_src "CREATE SOURCE \"materialize\".\"public\".\"format_bytes1_src\" IN CLUSTER \"kafka_formats\" FROM KAFKA CONNECTION \"materialize\".\"public\".\"kafka_conn\" (TOPIC = '<TOPIC>') EXPOSE PROGRESS AS \"materialize\".\"public\".\"format_bytes1_src_progress\""
                """
            )
        )
