# MiMo

一个用于小米MiMo开放平台API的Rust客户端库。

[![Crates.io](https://img.shields.io/crates/v/mimo.svg)](https://crates.io/crates/mimo)
[![Documentation](https://docs.rs/mimo/badge.svg)](https://docs.rs/mimo)
[![CI](https://github.com/your-repo/mimo/workflows/CI/badge.svg)](https://github.com/your-repo/mimo/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 功能特性

- ✅ **基础对话** - 支持非流式和流式响应
- ✅ **多模型支持** - mimo-v2-pro, mimo-v2-omni, mimo-v2-tts, mimo-v2-flash
- ✅ **函数调用** - 完整的工具定义和调用支持
- ✅ **联网搜索** - 内置Web搜索工具
- ✅ **多模态输入** - 支持图片、音频、视频理解
- ✅ **语音合成 (TTS)** - 支持多种音频格式输出
- ✅ **深度思考** - 支持思维链模式
- ✅ **结构化输出** - 支持JSON格式响应
- ✅ **异步/异步流** - 基于Tokio和futures的异步实现

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
mimo = "0.1"
```

## 快速开始

### 基础对话

```rust
use mimo::{Client, ChatRequest};

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
use mimo::Client;

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
| `mimo-v2-pro` | 旗舰模型 | 复杂推理、专业任务 |
| `mimo-v2-omni` | 多模态模型 | 图像、音频、视频理解 |
| `mimo-v2-tts` | 语音合成模型 | 文本转语音 |
| `mimo-v2-flash` | 轻量模型 | 快速响应、简单任务 |

```rust
use mimo::{ChatRequest, Model};

// 使用模型常量
let request = ChatRequest::new(Model::MIMO_V2_PRO);

// 或使用快捷方法
let request = ChatRequest::pro();      // mimo-v2-pro
let request = ChatRequest::omni();     // mimo-v2-omni
let request = ChatRequest::tts();      // mimo-v2-tts
let request = ChatRequest::flash();    // mimo-v2-flash
```

## API 文档

完整的API文档请参考：[https://docs.rs/mimo](https://docs.rs/mimo)

## 示例

查看 `examples/` 目录获取更多示例：

- `basic_chat.rs` - 基础对话
- `streaming_chat.rs` - 流式响应
- `function_calling.rs` - 函数调用
- `multimodal.rs` - 多模态输入
- `thinking_mode.rs` - 深度思考模式
- `web_search.rs` - 联网搜索
- `tts_basic.rs` - 基础TTS
- `tts_chinese.rs` - 中文TTS
- `tts_mp3.rs` - MP3格式输出
- `tts_chat_styled.rs` - 聊天风格的TTS接口
- `tts_user_context.rs` - 用户上下文TTS

运行示例：

```bash
export XIAOMI_API_KEY=your_api_key
cargo run --example basic_chat
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
