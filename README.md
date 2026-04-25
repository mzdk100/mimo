# MiMo

一个用于小米MiMo开放平台API的Rust客户端库。

[![Crates.io](https://img.shields.io/crates/v/mimo-api.svg)](https://crates.io/crates/mimo-api)
[![Documentation](https://docs.rs/mimo-api/badge.svg)](https://docs.rs/mimo-api)
[![CI](https://github.com/your-repo/mimo/workflows/CI/badge.svg)](https://github.com/your-repo/mimo/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 功能特性

- ✅ **基础对话** - 支持非流式和流式响应
- ✅ **多模型支持** - mimo-v2.5-pro, mimo-v2.5, mimo-v2-pro, mimo-v2-omni, mimo-v2-flash
- ✅ **函数调用** - 完整的工具定义和调用支持
- ✅ **联网搜索** - 内置Web搜索工具
- ✅ **多模态输入** - 支持图片、音频、视频理解
- ✅ **语音合成 (TTS)** - 支持多种音频格式输出（WAV/MP3/PCM/PCM16）
- ✅ **音色设计** - 通过文本描述设计自定义音色
- ✅ **语音克隆** - 通过音频样本克隆声音
- ✅ **风格控制** - 支持语速、情感、角色扮演等风格控制
- ✅ **深度思考** - 支持思维链模式
- ✅ **结构化输出** - 支持JSON格式响应
- ✅ **异步/异步流** - 基于Tokio和futures的异步实现

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
mimo-api = "0.2"
```

## 快速开始

### 基础对话

```rust
use mimo_api::{Client, ChatRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量 XIAOMI_API_KEY 创建客户端
    let client = Client::from_env()?;

    // 创建请求
    let request = ChatRequest::flash()
        .system("You are a helpful assistant.")
        .user("What is the capital of France?");

    // 发送请求
    let response = client.chat(request).await?;

    println!("{}", response.choices[0].message.content);

    Ok(())
}
```

## 配置

### 环境变量

```bash
XIAOMI_API_KEY=your_api_key_here
```

### 自定义配置

```rust
use mimo_api::Client;

// 从环境变量
let client = Client::from_env()?;

// 自定义API密钥
let client = Client::new("your-api-key")?;

// 自定义基础URL和超时
let client = Client::builder()
    .api_key("your-api-key")
    .base_url("https://custom-api.example.com/v1")
    .timeout(std::time::Duration::from_secs(60))
    .build()?;
```

## 模型

| 模型 | 描述 | 最佳用途 |
|------|------|----------|
| `mimo-v2.5-pro` | 最新旗舰模型 | 复杂推理、专业任务 |
| `mimo-v2.5` | 均衡性能模型 | 通用任务 |
| `mimo-v2-pro` | Agent导向旗舰模型 | Agent应用 |
| `mimo-v2-omni` | 多模态模型 | 图像、音频、视频理解 |
| `mimo-v2-flash` | 轻量模型 | 快速响应、简单任务 |
| `mimo-v2.5-tts` | TTS模型 | 文本转语音（预置音色） |
| `mimo-v2.5-tts-voicedesign` | TTS音色设计模型 | 通过文本描述设计音色 |
| `mimo-v2.5-tts-voiceclone` | TTS语音克隆模型 | 通过音频样本克隆声音 |
| `mimo-v2-tts` | TTS模型（旧版） | 文本转语音（3种基础音色） |

```rust
use mimo_api::{ChatRequest, Model};

// 使用模型常量
let request = ChatRequest::new(Model::MiMoV25Pro);

// 或使用快捷方法
let request = ChatRequest::v25_pro();           // mimo-v2.5-pro
let request = ChatRequest::v25();               // mimo-v2.5
let request = ChatRequest::pro();               // mimo-v2-pro
let request = ChatRequest::omni();             // mimo-v2-omni
let request = ChatRequest::flash();             // mimo-v2-flash
let request = ChatRequest::v25_tts();          // mimo-v2.5-tts
let request = ChatRequest::v25_tts_voicedesign();  // mimo-v2.5-tts-voicedesign
let request = ChatRequest::v25_tts_voiceclone();   // mimo-v2.5-tts-voiceclone
let request = ChatRequest::tts();              // mimo-v2-tts
```

## 语音合成 (TTS)

### 基础TTS

```rust
use mimo_api::{Client, Voice, AudioFormat};

let client = Client::from_env()?;

// 使用 mimo-v2-tts 模型（支持3种音色）
let response = client
    .tts("Hello, world!")
    .voice(Voice::MimoDefault)
    .format(AudioFormat::Wav)
    .send()
    .await?;

let audio = response.audio()?;
let audio_bytes = audio.decode_data()?;
std::fs::write("output.wav", audio_bytes)?;
```

### 使用 mimo-v2.5-tts 模型（支持11种预置音色）

```rust
// 预置音色包括：
// 中文女声：冰糖、茉莉
// 中文男声：苏打、白桦
// 英文女声：Mia、Chloe
// 英文男声：Milo、Dean
// 基础音色：MimoDefault、DefaultEn、DefaultZh

let response = client
    .v25_tts("你好，我是茉莉。")
    .voice(Voice::Moli)
    .send()
    .await?;
```

### 音色设计

通过文本描述设计自定义音色（使用 `mimo-v2.5-tts-voicedesign` 模型）：

```rust
let response = client
    .v25_tts_voicedesign("Hello, I'm a customized voice.")
    .user_message("Give me a young male tone, energetic and friendly.")
    .send()
    .await?;
```

**注意**：音色设计模型不支持 `voice()` 参数，音色完全通过 `user_message()` 描述。

### 语音克隆

通过音频样本克隆声音（使用 `mimo-v2.5-tts-voiceclone` 模型）：

```rust
// 方法1：从音频文件创建自定义音色
let voice = Voice::from_audio_file("voice_sample.mp3")?;

let response = client
    .v25_tts_voiceclone("This is a cloned voice.")
    .voice(voice)
    .send()
    .await?;

// 方法2：直接使用 base64 编码的音频数据
let voice = Voice::custom("data:audio/mpeg;base64,<base64_data>");
```

### 风格控制

使用 `<style>` 标签控制语音风格：

```rust
// 方法1：使用 tts_styled（非流式）
let response = client
    .tts_styled("开心", "明天就是周五了，真开心！")
    .voice(Voice::DefaultZh)
    .send()
    .await?;

// 方法2：使用流式API
let mut stream = client
    .tts_styled_stream("变快", "这个示例展示了流式语音合成。")
    .voice(Voice::DefaultZh)
    .send()
    .await?;

while let Some(chunk) = stream.next().await {
    let audio_bytes = chunk?;
    // 处理音频数据...
}
```

**可用风格**：
- **语速控制**："变快"、"变慢"
- **情感**："开心"、"悲伤"、"生气"
- **角色扮演**："孙悟空"、"林黛玉"
- **风格变化**："悄悄话"、"夹子音"、"台湾腔"
- **方言**："东北话"、"四川话"、"河南话"、"粤语"
- **唱歌**："唱歌"

### 流式TTS

```rust
use futures::StreamExt;

let mut stream = client
    .tts_stream("Hello, this is streaming TTS.")
    .voice(Voice::DefaultEn)
    .send()
    .await?;

let mut file = tokio::fs::File::create("output.pcm").await?;

while let Some(chunk) = stream.next().await {
    let audio_bytes = chunk?;
    file.write_all(&audio_bytes).await?;
}
```

## API 文档

完整的API文档请参考：[https://docs.rs/mimo-api](https://docs.rs/mimo-api)

## 示例

查看 `examples/` 目录获取更多示例：

### 基础对话
- `basic_chat.rs` - 基础对话
- `streaming_chat.rs` - 流式响应
- `function_calling.rs` - 函数调用
- `multimodal.rs` - 多模态输入
- `thinking_mode.rs` - 深度思考模式
- `web_search.rs` - 联网搜索

### 语音合成 (TTS)
- `tts_basic.rs` - 基础TTS（演示所有预置音色）
- `tts_chinese.rs` - 中文TTS
- `tts_mp3.rs` - MP3格式输出
- `tts_chat_styled.rs` - 聊天风格的TTS接口
- `tts_user_context.rs` - 用户上下文TTS
- `streaming_tts.rs` - 流式TTS
- `streaming_tts_styled.rs` - 带风格控制的流式TTS
- `streaming_tts_advanced.rs` - 高级流式TTS示例
- `voice_design.rs` - 音色设计示例
- `voice_cloning.rs` - 语音克隆示例

运行示例：

```bash
export XIAOMI_API_KEY=your_api_key
cargo run --example basic_chat
cargo run --example tts_basic
cargo run --example voice_design
cargo run --example voice_cloning
```

## 开发

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy --all-targets
cargo fmt --check
```

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 贡献

欢迎贡献！请随时提交 Pull Request。

## 相关链接

- [小米MiMo开放平台](https://platform.xiaomimimo.com/#/docs/welcome)

## 致谢

感谢小米MiMo团队提供优秀的AI服务。
