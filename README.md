# 纯RUST + LLM 构建一个web聊天机器人  
(prev: candel_chat)  
Forked from [vinicius-ianni/candle_chat](https://github.com/vinicius-ianni/candle_chat) forked from [danielclough/fireside-chat](https://github.com/danielclough/fireside-chat). 

**视频介绍**   
> [B站](https://www.bilibili.com/video/BV1oAgKehEom)  
> [youtube原视频](https://www.youtube.com/watch?v=Jw1E3LnNG0o)  


### 实现框架
前端：[leptos](https://github.com/leptos-rs/leptos)  
后端：[HuggingFace/Candle](https://github.com/huggingface/candle/)  
前后端通信：[Axum](https://github.com/tokio-rs/axum)  


### 主要配置文件
backend/.env  
```
IPV4="127.0.0.1"
PORT="3000"
```

backend/config_model.yaml
模型文件已下载到本地，参考：
```
cpu: false
use_flash_attn: false

# repo_id: "DanielClough/Candle_dolphin-2.2.1-mistral-7b"
# model_name: 
# model_config: "ChatML"

repo_id: 
model_name: 
model_config:

revision: "main"
quantized: true
# tokenizer_file:
# weight_files:
tokenizer_file: "../../candle_mistral/tokenizer.json"
weight_files:  "../../candle_mistral/Candle_Mistral-7B-v0.1_q4k.gguf" # 

```

网速好无需把模型文件下载到本地，参考：
```
cpu: false
use_flash_attn: false
repo_id: "DanielClough/Candle_dolphin-2.2.1-mistral-7b"
model_name: 
model_config: "ChatML"
# repo_id: "DanielClough/Candle_SOLAR-10.7B-Instruct-v1.0"
# model_name: "Candle_SOLAR-10.7B-Instruct-v1.0_q2k.gguf"
# model_config: SolarInstruct
# repo_id: "DanielClough/Candle_MistralLite"
# model_name: "Candle_MistralLite_q2k.gguf"
# model_config: "Amazon"
revision: "main"
tokenizer_file:
weight_files:
quantized: true
```

frontend/.env  
```
BACKEND_URL="127.0.0.1"
BACKEND_PORT="3000"
```

frontend/Trunk.toml
```
address = "127.0.0.1"
port = 8080
```

### 运行
虽然可以直接按照原教程的:
```
make dev
make prod
```
但是正常会出现依赖版本的不一致导致的报错，建议按如下步骤：
 
```
1. clone 项目
git clone https://github.com/xlsay/rust_chat.git
cd ../rust_chat
git clone https://github.com/huggingface/candle.git

2. 中国大陆开发者要先配置cargo国内源。

3. cd rust_chat/backend
3.1 修改上文提到的这个目录的配置文件 
3.2 cargo build
祝一切顺利。如果不顺利那就是 Cargo.toml 里依赖的版本不对，根据报错修改。


4. cd rust_chat/frontend
4.1 修改上文提到的这个目录的配置文件 
4.2 cargo build
祝一切顺利。如果不顺利那就是 Cargo.toml 里依赖的版本不对，根据报错修改。

5. 以上步骤都成功后。
cd rust_chat
make prod

6. ENJOY!
```