# required ubuntu dependencies

## Before doing anything else
sudo apt update && sudo apt upgrade

## clang
sudo apt install clang

## mold
To install mold linker in ubuntu CLI, you can follow these steps:

Add the following line to /etc/apt/sources.list: # deb [^1^][9] jammy-proposed universe
Update the package index: sudo apt-get update
Install mold deb package: sudo apt-get install mold

## ALSA
sudo apt install --reinstall alsa-base alsa-utils linux-sound-base libasound2 libasound2-dev
