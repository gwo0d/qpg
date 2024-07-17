use pqc_dilithium::Keypair;
use base64::prelude::*;
use chrono::{DateTime, Utc};
use crate::constants::OUTPUT_WIDTH;

pub(crate) struct QPGKeypair {
    keypair: Keypair,
    user_name: String,
    comment: Option<String>,
    created_timestamp: DateTime<Utc>,
}

impl QPGKeypair {
    pub fn new(user_name: String, comment: Option<String>) -> Self {
        let keypair = gen_keypair();

        Self {
            keypair: keypair.clone(),
            user_name,
            comment,
            created_timestamp: Utc::now(),
        }
    }

    pub fn get_public_key(self) -> String {
        let mut pk = BASE64_STANDARD.encode(self.keypair.public);
        for i in 0..pk.len() + OUTPUT_WIDTH {
            if i % OUTPUT_WIDTH == 0 {
                pk.insert(i, '\n')
            }
        }

        format!("-----BEGIN QGP PUBLIC KEY BLOCK-----\n{}\n-----END QGP PUBLIC KEY BLOCK-----", pk)
    }
}

fn gen_keypair() -> Keypair {
    Keypair::generate()
}