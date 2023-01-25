#!/bin/sh
PROJECT_NAME=iopgen
SRC0=`realpath $1`
SRC1=`realpath $2`

SCRIPT_DIR=$(cd $(dirname $0); pwd)
ENTRY_TERMINAL_POINT=$(pwd)
cd $SCRIPT_DIR/$PROJECT_NAME

# cargo build --release
cargo build

# failed
if [ $? -ne 0 ]; then
    cd -
    exit 1
fi

$SCRIPT_DIR//target/debug/$PROJECT_NAME \
    -c $SRC0 $SRC1 \
    -o ./EXPORT.md \
    -y example/yaml/rclpy_iop.yaml \
    -t "Title" \
    -j

cd -
