#!/usr/bin/bash

TMPFILE=$(mktemp)
PWD="$(pwd)/data"
wget "https://www.fec.gov/files/bulk-downloads/2018/indiv18.zip" -O $TMPFILE
unzip -d $PWD $TMPFILE
rm $TMPFILE
