use std::{collections::HashMap, fs, process};

mod config;
mod result;

use clap::Parser;
use reqwest::Client;

use crate::result::Result;

fn main() {
    let config = config::Config::parse();

    start_clash_meta(&config).unwrap();
    // gen_clash_config(&config).unwrap();
}

fn start_clash_meta(config: &config::Config) -> Result<process::Child> {
    let clash_meta_path = format!("{}/clash_meta/clash_meta_v1.19.0", config.data_dir);
    let clash_meta_config_path = format!("{}/clash_configs/clash_config.yaml", config.data_dir);
    let clash_meta_data_path = format!("{}/clash_meta_data", config.data_dir);
    let clash_meta_log_path = format!("{}/clash_meta_log", config.data_dir);
    if !fs::exists(&clash_meta_log_path)? {
        fs::create_dir(&clash_meta_log_path)?;
    }

    let clash_meta_log_stdio = process::Stdio::from(fs::File::create(format!(
        "{}/clash_meta.log",
        clash_meta_log_path
    ))?);


    let mut child = process::Command::new(&clash_meta_path)
        .arg("-f")
        .arg(clash_meta_config_path)
        .arg("-d")
        .arg(clash_meta_data_path)
        .stdout(clash_meta_log_stdio)
        .stdin(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()?;

    child.wait()?;
    return Result::Ok(child);
}

fn gen_clash_config(config: &config::Config) -> Result<()> {
    let template_raw_config = read_from_file(config.template_clash_config.clone())?;

    let subscribe_raw_config = read_from_url(config.subscribe_url.clone())?;

    let conbine_raw_config = conbine_clash_config(template_raw_config, subscribe_raw_config)?;

    let temp_clash_configs_path = format!("{}/clash_configs", config.data_dir);

    if !fs::exists(&temp_clash_configs_path)? {
        fs::create_dir(&temp_clash_configs_path)?;
    }

    save_to_file(
        conbine_raw_config,
        format!("{}/clash_configs/clash_config.yaml", config.data_dir),
    )?;

    return Result::Ok(());
}

fn conbine_clash_config(
    template_raw_config: String,
    subscibe_raw_config: String,
) -> Result<String> {
    let template_config_map: HashMap<String, serde_yml::Value> =
        serde_yml::from_str(&template_raw_config)?;
    let mut subscribe_config_map: HashMap<String, serde_yml::Value> =
        serde_yml::from_str(&subscibe_raw_config)?;

    for (key, value) in template_config_map {
        subscribe_config_map.insert(key, value);
    }

    let combine_raw_config = serde_yml::to_string(&subscribe_config_map)?;

    return Ok(combine_raw_config);
}

fn save_to_file(raw_config: String, path: String) -> Result<()> {
    let exist = fs::exists(&path)?;
    if exist {
        fs::remove_file(&path)?;
    }

    fs::write(&path, raw_config)?;
    return Ok(());
}

fn read_from_file(path: String) -> Result<String> {
    let content = fs::read_to_string(&path)?;
    return Ok(content);
}

fn read_from_url(url: String) -> Result<String> {
    let data = reqwest::blocking::get(url)?.text()?;
    return Result::Ok(data);
}
