#!/bin/sh
PROJECT_NAME=iopgen
SCRIPT_DIR=$(cd $(dirname $0); pwd)
cd $SCRIPT_DIR/$PROJECT_NAME

cargo build

# failed
if [ $? -ne 0 ]; then
    cd -
    exit 1
fi

echo "=============================="
echo ""

$SCRIPT_DIR/$PROJECT_NAME/target/debug/$PROJECT_NAME

echo ""
echo "=============================="

cd -
