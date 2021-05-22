#!/bin/sh

set -x

cargo test

if [ "$COV" = "yes" ]
then
    cargo install cargo-tarpaulin
    cargo tarpaulin -v --ignore-tests \
        --ciserver travis-ci --coveralls "$TRAVIS_JOB_ID"
fi
