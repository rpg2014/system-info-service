#!/bin/sh

git pull

cargo install --path . 
sudo systemctl restart system-info