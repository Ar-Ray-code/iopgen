#!/bin/sh
PROJECT_NAME=iopgen
SRC=`realpath $1`

SCRIPT_DIR=$(cd $(dirname $0); pwd)
ENTRY_TERMINAL_POINT=$(pwd)
cd $SCRIPT_DIR/$PROJECT_NAME

cargo build

# failed
if [ $? -ne 0 ]; then
    cd -
    exit 1
fi

echo "=============================="
echo "src: $SRC"

$SCRIPT_DIR/$PROJECT_NAME/target/debug/$PROJECT_NAME $SRC example/ros2/ros2.yaml ./EXPORT.md

echo ""
echo "=============================="

cd -
