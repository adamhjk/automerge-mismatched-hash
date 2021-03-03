#!/bin/bash
set -e

# Setup the test harness
npm install

rm -rf ./automerge
git clone http://github.com/automerge/automerge.git 
pushd automerge
git checkout $1
npm install
npm run build
popd

rm -rf ./automerge-rs
git clone http://github.com/automerge/automerge-rs
pushd automerge-rs
git checkout $2
popd

