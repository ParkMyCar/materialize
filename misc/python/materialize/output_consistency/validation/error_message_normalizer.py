# Copyright Materialize, Inc. and contributors. All rights reserved.
#
# Use of this software is governed by the Business Source License
# included in the LICENSE file at the root of this repository.
#
# As of the Change Date specified in that file, in accordance with
# the Business Source License, use of this software will be governed
# by the Apache License, Version 2.0.

import re


class ErrorMessageNormalizer:
    def normalize(self, error_message: str) -> str:
        # replace source prefix in column
        normalized_message = error_message
        normalized_message = re.sub(
            'column "[^.]*\\.', 'column "<source>.', normalized_message
        )
        normalized_message = normalized_message.replace("Evaluation error: ", "")

        # This will replace ln, log, and log10 mentions with log
        # see https://github.com/MaterializeInc/materialize/issues/19815
        normalized_message = re.sub(
            "(?<=function )(ln|log|log10)(?= is not defined for zero)",
            "log",
            normalized_message,
        )

        if "mz_timestamp out of range (" in normalized_message:
            # tracked with https://github.com/MaterializeInc/materialize/issues/19822
            normalized_message = normalized_message[0 : normalized_message.index(" (")]

        if "invalid base64 end sequence (" in normalized_message:
            # tracked with https://github.com/MaterializeInc/materialize/issues/23497
            normalized_message = normalized_message[0 : normalized_message.index(" (")]

        if (
            "not found in data type record" in normalized_message
            or (
                re.search(
                    r"function .*?record\(.*\) does not exist", normalized_message
                )
            )
            or (re.search(r"operator does not exist: .*?record", normalized_message))
            or "CAST does not support casting from record" in normalized_message
        ):
            # tracked with https://github.com/MaterializeInc/materialize/issues/28129
            normalized_message = normalized_message.replace("?", "")

        return normalized_message
