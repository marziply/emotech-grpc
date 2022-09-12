#!/bin/bash

# Generates a file of random data

# Not enough time to implement streaming/chunking of data
# head -c 2G < /dev/urandom > data.bin

head -c 20M < /dev/urandom > data.bin
