#!/bin/sh
PROJECT_NAME=iopgen
SRC=`realpath $1`

SCRIPT_DIR=$(cd $(dirname $0); pwd)
ENTRY_TERMINAL_POINT=$(pwd)
cd $SCRIPT_DIR/$PROJECT_NAME

cargo build --release

# failed
if [ $? -ne 0 ]; then
    cd -
    exit 1
fi

echo "============= IOP Gen ================="
$SCRIPT_DIR/$PROJECT_NAME/target/release/$PROJECT_NAME -c $SRC -o ./EXPORT.md -y example/ros2/ros2.yaml

echo ""
echo "======================================"

cd -
