//! Seed E2E peer helper (Maestro 03/10)
//!
//! Registra um peer fixo no identity-server local para os flows E2E que
//! enviam mensagem para um contato (ex.: `03_enviar_mensagem`,
//! `10_media_gallery`). Usa o mesmo caminho do app (`Client::register_username`),
//! registrando com o peer ID libp2p real (`12D3KooW…`), não o ID de identidade
//! `zaplivre_…`. O peer fica offline — a mensagem é persistida como Pending no
//! core (regressão 4b) e não pode propagar erro.
//!
//! Idempotente: se o username já existir no servidor, mantém o registro atual
//! (o app resolve o username em runtime via lookup).
//!
//! Run with:
//!   IDENTITY_SERVER_URL=http://localhost:8083 cargo run --example seed_peer
//!   (usuario padrao: maestro_e2e_peer, override via PEER_USERNAME)

use std::error::Error;

use zaplivre_core::api::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local = tokio::task::LocalSet::new();
    local
        .run_until(async {
            let server_url = std::env::var("IDENTITY_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:8083".to_string());
            let username =
                std::env::var("PEER_USERNAME").unwrap_or_else(|_| "maestro_e2e_peer".to_string());

            let data_dir = std::env::temp_dir().join("zl-seed-peer");
            let client = ClientBuilder::new()
                .data_dir(data_dir)
                .identity_server_url(server_url.clone())
                .build()
                .await?;

            println!(
                "Seeding peer '{username}' ({}) on {server_url} ...",
                client.local_peer_id()
            );

            match client.register_username(&username).await {
                Ok(_) => {
                    println!(
                        "OK: registered username '{username}' with peer_id {}",
                        client.local_peer_id()
                    );
                }
                Err(error) if error.to_string().to_lowercase().contains("taken") => {
                    println!(
                        "SKIP: username '{username}' already registered — using existing record"
                    );
                }
                Err(error) => return Err(error.into()),
            }

            Ok(())
        })
        .await
}
