use candle_core::{DType, Device};
use candle_nn::VarBuilder;
use candle_transformers::models::mistral::{Config as MistralConfig, Model as Mistral};
use candle_transformers::models::quantized_mistral::Model as QMistral;

use anyhow::{Error as E, Result};
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::tokenizer::Tokenizer;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum Model {
    Mistral(Mistral),
    Quantized(QMistral),
}

#[derive(Clone, Debug)]
pub struct ModelTokenizerDevice {
    pub model: Model,
    pub model_config: Option<String>,
    pub tokenizer: Tokenizer,
    pub device: Device,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct LoadModel {
    /// HuggingFace repo Id
    pub repo_id: String,
    /// HuggingFace model name
    pub model_name: String,
    /// Prompt Template (ChatML, MistralInstruct, Amazon, None)
    pub model_config: Option<String>,
    /// HuggingFace model revision
    pub revision: String,
    /// Optional tokenizer file
    pub tokenizer_file: Option<String>,
    /// Optional weight files
    pub weight_files: Option<String>,
    /// Use quantized model
    pub quantized: bool,
    /// Use FlashAttention to enhance memory efficiency
    pub use_flash_attn: bool,
    /// Run on CPU rather than on GPU.
    pub cpu: bool,
}
impl LoadModel {
    /// Save new './config_model.yaml'
    pub fn save_args(args: LoadModel) -> LoadModel {
        let yaml = serde_yaml::to_string(&args).expect("to string");
        std::fs::write("./config_model.yaml", yaml).expect("save file");
        LoadModel { ..args }
    }

    /// Default config to prevent failure.
    /// Will load "./config_model.yaml" if available.
    pub fn load_current_args() -> LoadModel {
        tracing::debug!("Loading './config_model.yaml' or Default Config.");

        let config_model_string = std::fs::read_to_string("./config_model.yaml");
        if config_model_string.is_ok() {
            println!("Loading './config_model.yaml'");
            let unwrapped = &config_model_string.unwrap();
            serde_yaml::from_str(unwrapped.as_str()).unwrap()
        } else {
            println!("Loading Default Config");
            LoadModel {
                cpu: false,
                use_flash_attn: false,
                repo_id: "DanielClough/Candle_Mistral-7B-Instruct-v0.1".to_string(),
                model_name: "Candle_Mistral-7B-Instruct-v0.1_q6k.gguf".to_string(),
                model_config: Some("MistralInstruct".to_string()),
                revision: "main".to_string(),
                tokenizer_file: None,
                weight_files: None,
                quantized: false,
            }
        }
    }
    pub fn load(args: LoadModel) -> Result<ModelTokenizerDevice> {
        println!("{:?}", args);
        let start = std::time::Instant::now();
        let api = Api::new()?;
        let repo = api.repo(Repo::with_revision(
            args.repo_id.clone(),
            RepoType::Model,
            args.revision,
        ));
        let tokenizer_filename = match args.tokenizer_file {
            Some(file) => std::path::PathBuf::from(file),
            None => repo.get("tokenizer.json")?,
        };
        let filenames = match args.weight_files {
            Some(files) => files
                .split(',')
                .map(std::path::PathBuf::from)
                .collect::<Vec<_>>(),
            None => {
                if args.quantized {
                    vec![repo.get(&args.model_name)?]
                } else {
                    match args.repo_id.as_str() {
                        "DanielClough/Candle_SOLAR-10.7B-Instruct-v1.0"
                        | "DanielClough/Candle_SOLAR-10.7B-v1.0" => vec![
                            repo.get("model-00001-of-00005.safetensors")?,
                            repo.get("model-00002-of-00005.safetensors")?,
                            repo.get("model-00003-of-00005.safetensors")?,
                            repo.get("model-00004-of-00005.safetensors")?,
                            repo.get("model-00005-of-00005.safetensors")?,
                        ],
                        "DanielClough/Candle_OrcaMini-3B" => vec![
                            repo.get("model-00001-of-00003.safetensors")?,
                            repo.get("model-00002-of-00003.safetensors")?,
                            repo.get("model-00002-of-00003.safetensors")?,
                        ],
                        _ => vec![
                            repo.get("model-00001-of-00002.safetensors")?,
                            repo.get("model-00002-of-00002.safetensors")?,
                        ],
                    }
                }
            }
        };

        println!("retrieved the files in {:?}", start.elapsed());

        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

        let start = std::time::Instant::now();

        let model_config = args.model_config.as_deref();
        let config = match model_config {
            Some("ChatML") =>
                MistralConfig::config_chat_ml(args.use_flash_attn),
            Some("Amazon") =>
                MistralConfig::config_amazon_mistral_lite(args.use_flash_attn),
            // Some("SolarInstruct") | Some("Solar") =>
            //     MistralConfig::config_upstage_solar(args.use_flash_attn),
            _ =>
                MistralConfig::config_7b_v0_1(args.use_flash_attn)
        };

        let (model, device) = if args.quantized {
            let device = candle_examples::device(args.cpu)?;
            let filename = &filenames[0];
            let vb = candle_transformers::quantized_var_builder::VarBuilder::from_gguf(filename, &device)?;
            let model = QMistral::new(&config, vb)?;
            // (Model::Quantized(model), Device::Cpu)
            (Model::Quantized(model), device)
        } else {
            let device = candle_examples::device(args.cpu)?;
            println!("Device: {:?}", device);

            let dtype = if device.is_cuda() {
                DType::BF16
            } else {
                DType::F32
            };
            println!("Dtype: {:?}", dtype);
            let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };
            let model = Mistral::new(&config, vb)?;
            println!("Model Loaded");
            (Model::Mistral(model), device)
        };

        println!("loaded the model in {:?}", start.elapsed());

        let model_args_out = ModelTokenizerDevice {
            model,
            model_config: args.model_config,
            tokenizer,
            device,
        };

        Ok(model_args_out)
    }
}
