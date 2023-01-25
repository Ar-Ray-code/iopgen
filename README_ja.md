# iopgen

Input-Output-Parameter parser for markdown generation.

ソースコード内の入出力とパラメータからMarkdownを生成するツールです。

ROS2を想定しています。

<br>

## インストール

```bash
cargo install --git https://github.com/Ar-Ray-code/iopgen.git
```

<br>

## 使い方

```bash
iopgen -c <example.cpp> -o <output.md> -y <ros2_iop.yaml>
```

<br>

### 引数

| 引数 | 説明 | 例 |
| --- | ----------- | ------- |
| -c | ソースコードのパス | example0.cpp, example1.cpp ... |
| -o | 出力ファイルのパス | output.md |
| -y | YAMLファイルのパス | rclcpp_iop.yaml/rclpy_iop.yaml |
| -t | Markdownファイルのタイトル | Title |
| -j | 日本語モード | (store_true) |

<br>