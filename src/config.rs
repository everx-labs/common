use adnl::{from_slice, common::KeyOption, node::AdnlNodeConfig};
use sha2::Digest;
use std::{fs::{File, read_to_string}, io::Write, net::{IpAddr, SocketAddr}, path::Path};
use ton_types::{fail, Result};

pub async fn get_test_config_path(prefix: &str, ip: &str) -> Result<String> {
    let socket = ip.parse::<SocketAddr>()?;
    let ip = if socket.ip().is_unspecified() {
        external_ip::ConsensusBuilder::new()
            .add_sources(external_ip::get_http_sources::<external_ip::Sources>())
            .build()
            .get_consensus().await
    } else {
        Some(socket.ip())
    };
    if let Some(IpAddr::V4(ip)) = ip {
        Ok(
            format!(
                "{}_{}_{}.json", 
                prefix, 
                ip.to_string().as_str(), 
                socket.port().to_string().as_str()
            )
        )
    } else {
        fail!("Cannot obtain own external IP address")
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
    let config = get_test_config_path(prefix, ip).await?;
    let config = if Path::new(config.as_str()).exists() {
        let config = read_to_string(config)?;
        AdnlNodeConfig::from_json(config.as_str(), true)?
    } else {
        let (json, bin) = if deterministic {
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
            )?
        } else {
            AdnlNodeConfig::with_ip_address_and_key_type(
                ip, 
                KeyOption::KEY_ED25519, 
                tags
            )?
        };
        File::create(config.as_str())?.write_all(
            serde_json::to_string_pretty(&json)?.as_bytes()
        )?;
        bin
    };
    Ok(config)
}

