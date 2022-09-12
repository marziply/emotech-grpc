#!/bin/bash

# Generates a 2GB file of random data

head -c 2G < /dev/urandom > data.bin
