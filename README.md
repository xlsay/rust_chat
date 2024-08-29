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
weight_files:  "../../candle_mistral/Candle_Mistral-7B-v0.1_q4k.gguf" # not support _q8k 

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

### For rust Newbies
- [Rust 程序设计语言](https://rustwiki.org/zh-CN/book/ch01-01-installation.html)
    ```shell
    Hello rust.
    1. 安装：
        curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh  
        安装完成后测试： rustc --version  
    2. 代码 main.rs
        fn main() {
            println!("Hello, world!");
        }
    3. 编译&运行
        rustc main.rs
        ./main

    ```
- Cargo [管理项目管理包的工具]
    - see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    ```
        - 可以使用 cargo build 构建项目。
        - 可以使用 cargo run 一步构建并运行项目。
        - 可以使用 cargo check 构建项目而无需生成二进制文件来检查错误。(比build速度快)
        - Cargo 会将构建结果保存到 target/debug 目录，而不是源码所在的目录。
    ```
