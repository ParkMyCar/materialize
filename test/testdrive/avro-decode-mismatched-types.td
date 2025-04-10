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
# Make sure that reading an Avro buffer with a schema different from the one that was used
# when it was written does not cause panics or anything like that.
#
# Note that this test uses inline Avro schemas and not the schema registry, so any
# protections thus provided are not in force.
#

$ set null={"type": "record", "name": "field_null", "fields": [ {"name": "f1", "type": "null"} ] }
$ set boolean={"type": "record", "name": "field_boolean", "fields": [ {"name": "f1", "type": "boolean"} ] }
$ set int={"type": "record", "name": "field_int", "fields": [ {"name": "f1", "type": "int"} ] }
$ set long={"type": "record", "name": "field_long", "fields": [ {"name": "f1", "type": "long"} ] }
$ set float={"type": "record", "name": "field_float", "fields": [ {"name": "f1", "type": "float"} ] }
$ set double={"type": "record", "name": "field_double", "fields": [ {"name": "f1", "type": "double"} ] }
$ set bytes={"type": "record", "name": "field_bytes", "fields": [ {"name": "f1", "type": "bytes"} ] }
$ set string={"type": "record", "name": "field_string", "fields": [ {"name": "f1", "type": "string"} ] }

#
# Null -> int
#

$ kafka-create-topic topic=avro-types-null2int

$ kafka-ingest format=avro topic=avro-types-null2int schema=${null} timestamp=1
{"f1": null}

> CREATE CONNECTION kafka_conn
  TO KAFKA (BROKER '${testdrive.kafka-addr}', SECURITY PROTOCOL PLAINTEXT);

> CREATE SOURCE avro_types_null2int
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-null2int-${testdrive.seed}')

> CREATE TABLE avro_types_null2int_tbl FROM SOURCE avro_types_null2int (REFERENCE "testdrive-avro-types-null2int-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${int}'
  ENVELOPE NONE

! SELECT * FROM avro_types_null2int_tbl
contains:avro deserialization error: unable to decode row : IO error: UnexpectedEof

#
# boolean -> int
#

$ kafka-create-topic topic=avro-types-boolean2int

$ kafka-ingest format=avro topic=avro-types-boolean2int schema=${boolean} timestamp=1
{"f1": true}
{"f1": false}

> CREATE SOURCE avro_types_boolean2int
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-boolean2int-${testdrive.seed}')

> CREATE TABLE avro_types_boolean2int_tbl FROM SOURCE avro_types_boolean2int (REFERENCE "testdrive-avro-types-boolean2int-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${int}'
  ENVELOPE NONE

# Bogus result, but at least we do not panic
> SELECT f1 FROM avro_types_boolean2int_tbl
-1
0

#
# int -> long
#

$ kafka-create-topic topic=avro-types-int2long

$ kafka-ingest format=avro topic=avro-types-int2long schema=${int} timestamp=1
{"f1": 123}

> CREATE SOURCE avro_types_int2long
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-int2long-${testdrive.seed}')

> CREATE TABLE avro_types_int2long_tbl FROM SOURCE avro_types_int2long (REFERENCE "testdrive-avro-types-int2long-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${long}'
  ENVELOPE NONE

> SELECT * FROM avro_types_int2long_tbl
123

#
# int -> float
#

$ kafka-create-topic topic=avro-types-int2float

$ kafka-ingest format=avro topic=avro-types-int2float schema=${int} timestamp=1
{"f1": 123}

> CREATE SOURCE avro_types_int2float
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-int2float-${testdrive.seed}')

> CREATE TABLE avro_types_int2float_tbl FROM SOURCE avro_types_int2float (REFERENCE "testdrive-avro-types-int2float-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${float}'
  ENVELOPE NONE

! SELECT * FROM avro_types_int2float_tbl
contains:avro deserialization error: unable to decode row : IO error: UnexpectedEof

#
# long -> float
#

$ kafka-create-topic topic=avro-types-long2float

$ kafka-ingest format=avro topic=avro-types-long2float schema=${long} timestamp=1
{"f1": 992147483647}

> CREATE SOURCE avro_types_long2float
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-long2float-${testdrive.seed}')

> CREATE TABLE avro_types_long2float_tbl FROM SOURCE avro_types_long2float (REFERENCE "testdrive-avro-types-long2float-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${float}'
  ENVELOPE NONE

! SELECT * FROM avro_types_long2float_tbl
contains:Unexpected bytes remaining

#
# long -> int
#

$ kafka-create-topic topic=avro-types-long2int

$ kafka-ingest format=avro topic=avro-types-long2int schema=${long} timestamp=1
{"f1": 992147483647}

> CREATE SOURCE avro_types_long2int
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-long2int-${testdrive.seed}')

> CREATE TABLE avro_types_long2int_tbl FROM SOURCE avro_types_long2int (REFERENCE "testdrive-avro-types-long2int-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${int}'
  ENVELOPE NONE

! SELECT * FROM avro_types_long2int_tbl
contains:Decoding error: Expected i32, got: 992147483647

#
# float -> double
#

$ kafka-create-topic topic=avro-types-float2double

$ kafka-ingest format=avro topic=avro-types-float2double schema=${float} timestamp=1
{"f1": 123456789.123456789}

> CREATE SOURCE avro_types_float2double
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-float2double-${testdrive.seed}')

> CREATE TABLE avro_types_float2double_tbl FROM SOURCE avro_types_float2double (REFERENCE "testdrive-avro-types-float2double-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${double}'
  ENVELOPE NONE

! SELECT * FROM avro_types_float2double_tbl
contains:avro deserialization error: unable to decode row : IO error: UnexpectedEof

#
# avro-typestion string -> bytes
#

$ kafka-create-topic topic=avro-types-string2bytes

$ kafka-ingest format=avro topic=avro-types-string2bytes schema=${string} timestamp=1
{"f1": "abc абц"}

> CREATE SOURCE avro_types_string2bytes
  IN CLUSTER ${arg.single-replica-cluster}
  FROM KAFKA CONNECTION kafka_conn (TOPIC 'testdrive-avro-types-string2bytes-${testdrive.seed}')

> CREATE TABLE avro_types_string2bytes_tbl FROM SOURCE avro_types_string2bytes (REFERENCE "testdrive-avro-types-string2bytes-${testdrive.seed}")
  FORMAT AVRO USING SCHEMA '${bytes}'
  ENVELOPE NONE

> SELECT * FROM avro_types_string2bytes_tbl
"abc \\xd0\\xb0\\xd0\\xb1\\xd1\\x86"
