#!/bin/bash

cd report
rm -rf target
cargo build --release
cp ./target/release/report /usr/local/bin/rpifanpwm
cd ../
cp ./rpifanpwm.service /etc/systemd/system/rpifanpwm.service
systemctl enable rpifanpwm.service
systemctl start rpifanpwm.service
