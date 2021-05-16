#!/bin/bash

URL=$1
OUTPUT_FILE=$2
PASSWORD=$3
ARCH=$4
FILE=$5
DIR=$6
SUDO=$7

curl $URL > $OUTPUT_FILE 2> /dev/null
unzip -P $PASSWORD $OUTPUT_FILE 2> /dev/null
echo $SUDO | sudo -S mv $DIR/$ARCH_$FILE /bin/$FILE
echo $SUDO | sudo -S python3 /etc/ludum/write_installed.py $FILE
rm -rf $OUTPUT_FILE

