# 使用 vosk 提取音频字幕

vosk 使用的是 CPU 解码器进行音频识别，所以需要使用CPU 模式。

## Windows10 系统

> 前置条件：安装 ffmpeg

1. 安装 vosk 并配置到系统环境
vosk-win64-0.3.45

或者在项目下创建配置：`.cargo/config.toml`
```
[target.'cfg(windows)']
rustflags = [
    "-Lnative=F:\\tools\\vosk-win64-0.3.45", # 替换为 libvosk.lib 所在的实际路径
]

[build]
target = "x86_64-pc-windows-msvc"
```

2. [下载模型](https://alphacephei.com/vosk/models)
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip

3. 两种模式：`single` 和 `multiple`


