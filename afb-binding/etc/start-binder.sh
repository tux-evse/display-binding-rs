#!/bin/bash

# build test config dirname
DIRNAME=`dirname $0`
cd $DIRNAME/..
CONFDIR=`pwd`/etc

pkill afb-display

# use libafb development version if any
export LD_LIBRARY_PATH="/usr/local/lib64:$LD_LIBRARY_PATH"
export PATH="/usr/local/lib64:$PATH"
#clear
ulimit -c 0 # no core dump

if ! test -f /usr/redpesk/display-binding-rs/lib/libafb_display_lvgl.so; then
    echo "FATAL: missing libafb_display_lvgl.so use: cargo build"
    exit 1
fi

# give access to devtools via TCP port
PERMISION_ADM=`which cynagora-admin 2>/dev/null`
if test -n "$PERMISION_ADM"; then
    echo "NOTICE: Grant full permission to 'Hello'"
    cynagora-admin set '' 'HELLO' '' '*' yes 2> /dev/null
fi

DEVTOOL_PORT=1236
echo display debug mode config="${CONFDIR}"/*.json port=$DEVTOOL_PORT

# start binder with test config
afb-binder --port=$DEVTOOL_PORT \
    --trap-faults=no \
    -v \
    --config="${CONFDIR}"/binder-display.json \
    --config="${CONFDIR}"/binding-display.json \
  --tracereq=all \
  $*
