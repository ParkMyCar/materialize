// Copyright Materialize, Inc. and contributors. All rights reserved.
//
// Use of this software is governed by the Business Source License
// included in the LICENSE file.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0.

use std::env;

fn main() -> Result<(), anyhow::Error> {
    println!("cargo:rustc-env=TARGET_TRIPLE={}", env::var("TARGET")?);

    cc::Build::new()
        .file("src/environmentd/sys.c")
        .compile("environmentd_sys");

    let out_dir = std::env::var("OUT_DIR").ok().map(std::path::PathBuf::from);
    mz_npm::ensure(out_dir)
}
