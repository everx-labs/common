use adnl::{from_slice, common::KeyOption, node::{AdnlNodeConfig, AdnlNodeConfigJson}};
use sha2::Digest;
use std::{fs::{File, read_to_string}, io::Write, net::{IpAddr, SocketAddr}, path::Path};
use ton_types::{fail, Result};

pub async fn resolve_ip(ip: &str) -> Result<SocketAddr> {
    let mut ret = ip.parse::<SocketAddr>()?;
    if ret.ip().is_unspecified() {
        let ip = external_ip::ConsensusBuilder::new()
            .add_sources(external_ip::get_http_sources::<external_ip::Sources>())
            .build()
            .get_consensus().await;
        if let Some(IpAddr::V4(ip)) = ip {
            ret.set_ip(IpAddr::V4(ip))
        } else {
            fail!("Cannot obtain own external IP address")
        }
    }
    Ok(ret)
}

pub fn get_test_config_path(prefix: &str, addr: SocketAddr) -> Result<String> {
    if let IpAddr::V4(ip) = addr.ip() {
        Ok(
            format!(
                "{}_{}_{}.json", 
                prefix, 
                ip.to_string().as_str(), 
                addr.port().to_string().as_str()
            )
        )
    } else {
        fail!("Cannot generate config path for IP address that is not V4")
    }
} 

pub fn generate_adnl_configs(
    ip: &str, 
    tags: Vec<usize>,
    deterministic: bool
) -> Result<(AdnlNodeConfigJson, AdnlNodeConfig)> {
    if deterministic {
        let mut keys = Vec::new();
        let mut hash = sha2::Sha256::new();
        hash.input(ip.as_bytes());
        for tag in tags {
            let mut hash = hash.clone();
            hash.input(&tag.to_be_bytes());
            let key = hash.result();
            let key = key.as_slice();
            let key = from_slice!(key, 32);
            keys.push((key, tag));
        }
        AdnlNodeConfig::from_ip_address_and_private_keys(
            ip, 
            KeyOption::KEY_ED25519, 
            keys
        )
    } else {
        AdnlNodeConfig::with_ip_address_and_key_type(
            ip, 
            KeyOption::KEY_ED25519, 
            tags
        )
    }
}

// Is used only for protocol tests
#[allow(dead_code)]
pub async fn get_adnl_config(
    prefix: &str, 
    ip: &str, 
    tags: Vec<usize>,
    deterministic: bool
) -> Result<AdnlNodeConfig> {
    let resolved_ip = resolve_ip(ip).await?;
    let config = get_test_config_path(prefix, resolved_ip)?;
    let config = if Path::new(config.as_str()).exists() {
        let config = read_to_string(config)?;
        AdnlNodeConfig::from_json(config.as_str(), true)?
    } else {
        let (json, bin) = generate_adnl_configs(ip, tags, deterministic)?;
        File::create(config.as_str())?.write_all(
            serde_json::to_string_pretty(&json)?.as_bytes()
        )?;
        bin
    };
    Ok(config)
}
