#!/bin/bash
diesel setup --database-url=postgres://postgres:postgres123@spidercontainerdb/rust_server
cargo run
