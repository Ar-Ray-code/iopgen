# iopgen

[日本語](./README_ja.md)

Input-Output-Parameter parser for markdown generation.

<br>

## Install

```bash
cargo install --git https://github.com/Ar-Ray-code/iopgen.git
```

<br>

## Usage

```bash
iopgen -c <example.cpp> -o <output.md> -y <ros2_iop.yaml>
```

<br>

### Args

| Arg | Description | Example |
| --- | ----------- | ------- |
| -c | Path to the C++/Python files | example0.cpp, example1.cpp ... |
| -o | Path to the output file | output.md |
| -y | Path to the YAML file | rclcpp_iop.yaml/rclpy_iop.yaml |
| -t | Title of the markdown file | Title |
| -j | Japanese mode | (store_true) |

<br>


## About author
- author : [Ar-Ray](https://github.com/Ar-Ray-code)
- [Twitter](https://twitter.com/Ray255Ar)
