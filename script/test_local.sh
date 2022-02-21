#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo +stable test -- --nocapture
