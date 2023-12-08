#!/bin/bash

# move to projet base to get relative path to logo image
DIR=`dirname $0`
cd $DIR/../..

# use libafb development version if any
export LD_LIBRARY_PATH="/usr/local/lib64:$LD_LIBRARY_PATH"
export PATH="/usr/local/lib64:$PATH"
#clear
ulimit -c 0 # no core dump

if ! test -f $CARGO_TARGET_DIR/debug/libafb_lvgl.so; then
    echo "FATAL: missing libafb_lvgl.so use: cargo build"
    exit 1
fi

if ! test -w /dev/fb0; then
    echo "FATAL: missing permision to write /dev/fb0 (missing group video ???)"
    ls -l  /dev/fb0
    exit 1
fi

if ! test -r /dev/input/lvgl; then
    echo "FATAL: missing permision to read /dev/input/lvgl (missing group input ???)"
    ls -l /dev/input/lvgl
    exit 1
fi

# give access to devtools via TCP port
PERMISION_ADM=`which cynagora-admin 2>/dev/null`
if test -n "$PERMISION_ADM"; then
    echo "NOTICE: Grant full permission to 'Hello'"
    cynagora-admin set '' 'HELLO' '' '*' yes 2> /dev/null
fi

# start binder with test config
afb-binder --trap-faults=no -v --config=afb-binding/etc/binding-native-lvgl.json
