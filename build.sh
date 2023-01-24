#!/bin/sh
PROJECT_NAME=iopgen
SCRIPT_DIR=$(cd $(dirname $0); pwd)
cd $SCRIPT_DIR/$PROJECT_NAME
# arg
SRC=$1

cargo build

# failed
if [ $? -ne 0 ]; then
    cd -
    exit 1
fi

echo "=============================="
echo "$SCRIPT_DIR/$PROJECT_NAME/target/debug/$PROJECT_NAME example/cpp/$SRC"
echo ""

$SCRIPT_DIR/$PROJECT_NAME/target/debug/$PROJECT_NAME example/cpp/$SRC example/ros2/ros2.yaml

echo ""
echo "=============================="

cd -
