#!/bin/bash

cd service
rm -rf ./target
cargo build --release
cp ./target/release/service /usr/local/bin/radiator
cd ../
cp ./radiator.service /etc/systemd/system/radiator.service
systemctl enable radiator.service
systemctl start radiator.service
