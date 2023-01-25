#!/bin/bash

SCRIPT_DIR=$(cd $(dirname $0); pwd)
cd $SCRIPT_DIR

wget https://raw.githubusercontent.com/Ar-Ray-code/YOLOv5-ROS/main/yolov5_ros/yolov5_ros/main.py -O python/yolox_ros.py
wget https://raw.githubusercontent.com/Ar-Ray-code/YOLOX-ROS/main/yolox_ros_cpp/yolox_ros_cpp/src/yolox_ros_cpp.cpp -O cpp/yolox_ros.cpp

wget https://raw.githubusercontent.com/Ar-Ray-code/darknet_ros/master/darknet_ros/src/YoloObjectDetector.cpp -O cpp/darknet_ros.cpp
wget https://raw.githubusercontent.com/HarvestX/PlayStation-JoyInterface-ROS2/main/p9n_node/src/teleop_twist_joy_node.cpp -O cpp/teleop_twist_joy_node.cpp
