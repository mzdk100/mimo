[](/#/)中文
[](/#/)[联系我们](/#/contact)[文档](/#/docs/welcome)[控制台](/#/console/balance)[博客](https://mimo.xiaomi.com)中文
[](https://huggingface.co/XiaomiMiMo)Ctrl[欢迎使用
](/#/docs/welcome)快速开始
[首次调用 API
](/#/docs/quick-start/first-api-call)[模型超参
](/#/docs/quick-start/model-hyperparameters)[错误码
](/#/docs/quick-start/error-codes)[定价与限速
](/#/docs/pricing)新闻
[Xiaomi MiMo-V2-Pro 发布：面向 Agent 时代的旗舰基座
](/#/docs/news/v2-pro-release)[Xiaomi MiMo-V2-Omni 发布：看得清，听得懂，能动手的全模态 Agent 基座
](/#/docs/news/v2-omni-release)[Xiaomi MiMo-V2-TTS 发布：能说会唱的语音合成大模型
](/#/docs/news/v2-tts-release)[MiMo-V2-Flash 更新日志 2026/03/03
](/#/docs/news/news20260303)[MiMo-V2-Flash 更新日志 2026/02/04
](/#/docs/news/news20260212)[Xiaomi MiMo API 开放平台计费即将启动
](/#/docs/news/billing)[Xiaomi MiMo API 开放平台充值功能开放通知
](/#/docs/news/recharge)[MiMo-V2-Flash 更新日志 2026/01/12
](/#/docs/news/news20260112)[MiMo 模型公测限免延长公告
](/#/docs/news/beta-free)[MiMo-V2-Flash 发布 2025/12/16
](/#/docs/news/news20251216)API 文档
对话
[OpenAI API
](/#/docs/api/chat/openai-api)[Anthropic API
](/#/docs/api/chat/anthropic-api)集成扩展
[Claude Code 配置
](/#/docs/integration/claude-code)[Cline、Kilo Code、Roo Code 配置
](/#/docs/integration/cline-kilo-roo)使用指南
工具调用
[联网搜索
](/#/docs/usage-guide/tool-calling/web-search)多模态理解
[图片理解
](/#/docs/usage-guide/multimodal-understanding/image-understanding)[语音理解
](/#/docs/usage-guide/multimodal-understanding/audio-understanding)[视频理解
](/#/docs/usage-guide/multimodal-understanding/video-understanding)[语音合成
](/#/docs/usage-guide/speech-synthesis)[常见问题
](/#/docs/faq)更新日志
[模型发布
](/#/docs/updates/model)[功能更新
](/#/docs/updates/feature)条款与协议
[服务协议
](/#/docs/terms/user-agreement)[隐私政策
](/#/docs/terms/privacy-policy)开发者交流群
[网页版免费对话
](https://aistudio.xiaomimimo.com/open-apis/v1/genLoginUrl)- [文档](/#/docs)- - 使用指南- - 语音合成# 语音合成
语音合成（文本转语音）支持将输入的文本自动转换为自然流畅的语音输出。您可通过配置发音风格等参数，生成表达丰富、效果生动的语音内容。
**核心能力**
- 
**提供预置音色**：内置默认音色，满足快速使用需求。
- 
**多样化发音风格**：支持指定发音风格，语音更生动自然。
## 支持的模型列表
当前仅支持 `mimo-v2-tts` 模型。
## 准备工作
获取 API Key 等准备工作，请参考 [首次调用 API](https://platform.xiaomimimo.com/#/docs/quick-start/first-api-call)。
## 可选预置音色
使用时，可在 `{"audio": {"voice": "mimo_default"}}` 中设置预置音色。
| **音色名** | **voice参数** 
| MiMo-默认 | mimo_default 
| MiMo-中文女声 | default_zh 
| MiMo-英文女声 | default_en 
当前不支持音色克隆
## 风格控制
### 语音整体风格控制
将 `<style>style</style>` 置于转换目标文本开头，其中 `style` 为需要生成的音频风格。如需设置多种风格，请将多个风格名称置于同一个 `<style>` 标签内，分隔符不限。
**格式示例：**`<style>风格1 风格2</style>待合成内容`。
以下是一些推荐使用的风格，支持使用不在列表中的风格。
| **风格类型** | **风格示例** 
| 语速控制 | *变快/变慢* 
| 情绪变化 | *开心/悲伤/生气* 
| 角色扮演 | *孙悟空/林黛玉* 
| 风格变化 | *悄悄话/夹子音/台湾腔* 
| 方言 | *东北话/四川话/河南话/粤语* 
**样例：**
- `<style>开心</style>明天就是周五了，真开心！`
- `<style>东北话</style>哎呀妈呀，这天儿也忒冷了吧！你说这风，嗖嗖的，跟刀子似的，割脸啊！`
- `<style>粤语</style>呢个真係好正啊！食过一次就唔会忘记！`
### 音频标签细粒度控制
通过 [音频标签] ，你可以对声音进行细粒度控制，精准调节语气、情绪和表达风格——无论是低声耳语、放声大笑，还是带点小情绪的小吐槽，也可以灵活插入呼吸声，停顿，咳嗽等，都能轻松实现。语速同样可以灵活调整，让每句话都有它该有的节奏。
**样例：**
- （紧张，深呼吸）呼……冷静，冷静。不就是一个面试吗……（语速加快，碎碎念）自我介绍已经背了五十遍了，应该没问题的。加油，你可以的……（小声）哎呀，领带歪没歪？
- （极其疲惫，有气无力）师傅……到地方了叫我一声……（长叹一口气）我先眯一会儿，这班加得我魂儿都要散了。
- 如果我当时……（沉默片刻）哪怕再坚持一秒钟，结果是不是就不一样了？（苦笑）呵，没如果了。
- （寒冷导致的急促呼吸）呼——呼——这、这大兴安岭的雪……（咳嗽）简直能把人骨头冻透了……别、别停下，走，快走。
- （提高音量喊话）大姐！这鱼新鲜着呢！早上刚捞上来的！哎！那个谁，别乱翻，压坏了你赔啊？！
## 调用示例
**注意事项**
- 语音合成的目标文本需填写在 `role` 为 `assistant` 的消息中，不可放在 `user` 角色的消息内。
- `user` 角色的消息为可选参数，但建议用户携带，可在部分场景下调整语音合成的语气与风格。
- 指定语音风格时，需将 `<style>style</style>` 置于目标文本开头。
- 如需体验更佳的唱歌风格，必须在目标文本最开头仅添加 `<style>唱歌</style>` 标签，格式为：`<style>唱歌</style>目标文本`。
**Curl**
```
curl --location --request POST 'https://api.xiaomimimo.com/v1/chat/completions' \
--header "api-key: $MIMO_API_KEY" \
--header 'Content-Type: application/json' \
--data-raw '{
"model": "mimo-v2-tts",
"messages": [
{
"role": "user",
"content": "Hello, MiMo, have you had lunch?"
},
{
"role": "assistant",
"content": "Yes, I had a sandwich."
}
],
"audio": {
"format": "wav",
"voice": "mimo_default"
}
}'
```
**Python**
```
import os
from openai import OpenAI
import base64
client = OpenAI(
api_key=os.environ.get("MIMO_API_KEY"),
base_url="https://api.xiaomimimo.com/v1"
)
completion = client.chat.completions.create(
model="mimo-v2-tts",
messages=[
{
"role": "user",
"content": "Hello, MiMo, have you had lunch?"
},
{
"role": "assistant",
"content": "Yes, I had a sandwich."
}
],
audio={
"format": "wav",
"voice": "mimo_default"
}
)
message = completion.choices[0].message
audio_bytes = base64.b64decode(message.audio.data)
with open("audio_file.wav", "wb") as f:
f.write(audio_bytes)
```
## 计费说明
- 
计费：限时免费。
- 
查看账单：您可以在控制台的 [账单明细](https://platform.xiaomimimo.com/#/console/usage) 页面查看用量。
更新时间 2026 年 03 月 19 日[视频理解](/#/docs/usage-guide/multimodal-understanding/video-understanding)[常见问题](/#/docs/faq)
### 目录
支持的模型列表准备工作可选预置音色风格控制语音整体风格控制音频标签细粒度控制调用示例计费说明回到顶部[Xiaomi MiMo 开放平台服务协议](/#/docs/terms/user-agreement)&nbsp;|&nbsp;[Xiaomi MiMo 开放平台隐私政策](/#/docs/terms/privacy-policy)
Copyright©2025 Xiaomi. All Rights Reserved&nbsp;|&nbsp;[Xiaomi MiMo : 备案号 Beijing-XiaomiMiMo-202601050182](https://mp.weixin.qq.com/s/67LmJeFdhi58UpPk9Wbssg)&nbsp;|&nbsp;[小米大语言模型算法 : 备案号 网信算备110108916280901240011号](https://cdn.cnbj3-fusion.fds.api.mi-img.com/chatbot-prod/system/%E5%85%AC%E7%A4%BA%E5%86%85%E5%AE%B9_%E7%BD%91%E4%BF%A1%E7%AE%97%E5%A4%87110108916280901240011%E5%8F%B7.pdf)&nbsp;|&nbsp;京ICP备17028681号-55