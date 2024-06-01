#!/bin/bash

image_path=$1
test_binary=$2

qemu-system-x86_64 -drive format=raw,file=$image_path
# qemu-system-x86_64 -drive format=raw,file=/Users/jurkokri/www/kukios/target/x86_64-kukios/debug/bootimage-kukios.bin
