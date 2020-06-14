#!/bin/bash

cargo run tests/resources/input/site

echo "Open files at file://$(pwd)/out/index.html"
