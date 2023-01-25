# title
<br>

## Subscribe (Input)

| topic_name | message type | qos_profile | description |
| --- | --- | --- | --- |

<br>

## Publish (Output)

| topic_name | message type | qos_profile | description |
| --- | --- | --- | --- |
| `objectDetectorTopicName` | `darknet_ros_msgs::msg::ObjectCount` | `object_publisher_qos` | 
| `boundingBoxesTopicName` | `darknet_ros_msgs::msg::BoundingBoxes` | `bounding_boxes_publisher_qos` | 
| `detectionImageTopicName` | `sensor_msgs::msg::Image` | `detection_image_publisher_qos` | 

<br>

## Parameter


| parameter_name | default_value | description |
| --- | --- | --- |
| `image_view.enable_opencv` | `true` | 
| `image_view.wait_key_delay` | `3` | 
| `image_view.enable_console_output` | `false` | 
| `yolo_model.detection_classes.names` | `std::vector<std::string>` | 
| `yolo_model.threshold.value` | `0.3f` | 
| `yolo_model.weight_file.name` | `std::string` | 
| `weights_path` | `std::string` | 
| `yolo_model.config_file.name` | `std::string` | 
| `config_path` | `std::string` | 
| `subscribers.camera_reading.topic` | `std::string` | 
| `subscribers.camera_reading.queue_size` | `1` | 
| `publishers.object_detector.topic` | `std::string` | 
| `publishers.object_detector.queue_size` | `1` | 
| `publishers.object_detector.latch` | `false` | 
| `publishers.bounding_boxes.topic` | `std::string` | 
| `publishers.bounding_boxes.queue_size` | `1` | 
| `publishers.bounding_boxes.latch` | `false` | 
| `publishers.detection_image.topic` | `std::string` | 
| `publishers.detection_image.queue_size` | `1` | 
| `publishers.detection_image.latch` | `true` | 
| `yolo_model.window_name` | `std::string` | 
| `actions.camera_reading.topic` | `std::string` | 


<br>


## Operation check status

| OS | Arch | Distro | status |
| --- | --- | --- | --- |
| Ubuntu20 | amd64 | Galactic | |

<br>

## Requirement

- ROS2

- Laptop

<br>

## How to build

```bash
# write here
```

<br>

## Run

```bash
# write here
```

<br>

Add a picture of the operation result.

<br>

## Reference

Add a reference to the source code.

- [Ar-Ray-code/iopgen](https://github.com/Ar-Ray-code/iopgen)

<br>
