use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::env;
use std::fs::File;
use std::io::BufReader;

pub fn certs_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let cert_env = env::var("CERT_FILE")
        .map_err(|e| format!("Failed to read CERT_FILE environment variable: {}", e))?;

    let key_env = env::var("KEY_FILE")
        .map_err(|e| format!("Failed to read KEY_FILE environment variable: {}", e))?;

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let mut certs_file = BufReader::new(File::open(cert_env)?);
    let mut key_file = BufReader::new(File::open(key_env)?);

    let tls_certs = certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| "Failed to parse certificates")?;

    let tls_key = pkcs8_private_keys(&mut key_file)
        .next()
        .ok_or("No private key found")??;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, PrivateKeyDer::Pkcs8(tls_key))?;

    log::info!("Successfully loaded TLS certificates.");
    Ok(config)
}
