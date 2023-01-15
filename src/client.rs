use libp2p::{
    floodsub::Topic,
    identity,
    multihash::{Code, MultihashDigest},
    PeerId,
};
use log::info;
use tokio::fs;

use crate::recipe::Recipe;

pub struct Client {
    pub name: String,
    pub keys: identity::Keypair,
    pub peer_id: PeerId,
    pub topic: Topic,
    pub storage_file_path: String,
}

type Recipes = Vec<Recipe>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

impl Client {
    pub fn new(name: &str) -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        info!("New peer with Id: {}", peer_id);
        Self {
            name: name.to_string(),
            keys: keypair,
            peer_id: peer_id,
            topic: Topic::new("recipes"),
            storage_file_path: "./recipes.json".to_string(),
        }
    }

    pub async fn read_local_recipes(&self) -> Result<Recipes> {
        let content = fs::read(self.storage_file_path.as_str()).await?;
        let result = serde_json::from_slice(&content)?;
        Ok(result)
    }
}
