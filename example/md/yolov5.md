# title
<br>

## Subscribe (Input)

| topic_name | message type | qos_profile | description |
| --- | --- | --- | --- |
| `image_raw` | `Image` | `self.image_callback` | 

<br>

## Publish (Output)

| topic_name | message type | qos_profile | description |
| --- | --- | --- | --- |
| `yolov5/bounding_boxes` | `BoundingBoxes` | `10)` | 
| `yolov5/image_raw` | `Image` | `10)` | 

<br>

## Parameter


| parameter_name | default_value | description |
| --- | --- | --- |
| `weights` | `str` | 
| `data` | `str` | 
| `imagez_height` | `640)` | 
| `imagez_width` | `640)` | 
| `conf_thres` | `0.25)` | 
| `iou_thres` | `0.45)` | 
| `max_det` | `1000)` | 
| `device` | `cpu)` | 
| `view_img` | `True)` | 
| `classes` | `None)` | 
| `agnostic_nms` | `False)` | 
| `line_thickness` | `2)` | 
| `half` | `False)` | 
| `dnn` | `False)` | 


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
