#!/bin/sh

#check to see if rust / cargo is installed
if ! command -v git &> /dev/null
then
    sudo apt update  # To get the latest package lists
    sudo apt install git -y
fi
if ! command -v cargo &> /dev/null
then 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -- --default-toolchain nightly --profile minimal
fi
git clone https://github.com/rpg2014/system-info-service.git

cd system-info-service

#need to build the secret key before building, and add to Rocket.toml
echo "secret_key = \"$(openssl rand -base64 32)\"" >> Rocket.toml

cargo build --release

cargo install --path .

sudo cp Rocket.toml /

sudo cp system-info.service /etc/systemd/system
sudo systemctl daemon-reload
sudo systemctl status system-info.service
sudo systemctl enable system-info.service
sudo systemctl start system-info.service