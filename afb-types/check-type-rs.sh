#!/usr/bin/bash

for f in ./src/*-types.rs ;  do
    echo "-----------"
    find ../../ -name "$(basename "${f}")" -exec md5sum {} \;
done