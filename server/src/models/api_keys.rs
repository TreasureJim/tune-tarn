use sha2::Digest;
use sqlx::PgPool;

#[derive(sqlx::FromRow)]
pub struct ApiKey {
    id: i16,
    user_id: i16,
    description: Option<String>,
    prefix: String,
    hash_info: String,
    hash: String,
}

impl ApiKey {
    pub async fn get_user(
        pool: &PgPool,
        api_key: &RawApiKey,
    ) -> Result<super::users::User, ApiKeyError> {
        let hash = api_key.hash()?;
        let key = sqlx::query_as::<_, ApiKey>("SELECT * FROM api_keys where hash like $1 LIMIT 1")
            .bind(&hash)
            .fetch_optional(pool)
            .await?
            .ok_or(ApiKeyError::NotFound)?;

        let user = sqlx::query_as::<_, super::users::User>("SELECT * FROM users where id == $1 LIMIT 1")
            .bind(key.id)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiKeyError {
    #[error("Invalid API key format: {0}")]
    ParseError(#[from] ParseError),
    
    #[error("Unknown hash algorithm: {0}")]
    HashError(#[from] HashError),
    
    #[error("API key not found")]
    NotFound,
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("API key is invalid or expired")]
    InvalidKey,
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

const PREFIX_SIZE: u8 = 7;
const API_KEY_SIZE: u8 = 53;
const WHOLE_KEY_SIZE: u8 = PREFIX_SIZE + 1 + API_KEY_SIZE;

#[derive(Debug, Clone)]
pub struct RawApiKey {
    prefix: String,
    hash_info: String,
    key: String,
}

impl RawApiKey {
    /// Creates a new RawApiKey from a full API key string.
    ///
    /// Expected format: "prefix.hash_info:key"
    /// Where:
    /// - `prefix`: 7 characters (e.g., "sk_123ab")
    /// - `.`: literal dot separator
    /// - `hash_info`: algorithm identifier (e.g., "SHA256")
    /// - `:`: literal colon separator
    /// - `key`: remaining characters (typically 45 chars)
    ///
    /// Example: "sk_123.SHA256:abcdefghijklmnopqrstuvwxyz0123456789ABCDEF"
    ///          └──┬──┘ └─┬──┘ └────────────────────┬────────────────────┘
    ///           prefix   │                    secret key
    ///                 algorithm
    pub fn parse(full_key: &str) -> Result<Self, ParseError> {
        if !full_key.contains('.') {
            return Err(ParseError::MissingSeparator);
        }

        let parts: Vec<&str> = full_key.splitn(2, '.').collect();
        let prefix = parts[0].to_string();
        let parts: Vec<&str> = parts[1].splitn(2, ':').collect();
        let hash_info = parts[0].to_string();
        let key = parts[1].to_string();

        if prefix.len() != PREFIX_SIZE as usize {
            return Err(ParseError::InvalidPrefixLength(prefix.len()));
        }

        if key.len() != API_KEY_SIZE as usize {
            return Err(ParseError::EmptyKey);
        }

        Ok(Self {
            prefix,
            key,
            hash_info,
        })
    }

    /// Returns the full API key in the format: "prefix.key"
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.prefix, self.key)
    }

    /// Returns just the prefix (first 7 characters)
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Returns just the secret key part (after the dot)
    pub fn secret(&self) -> &str {
        &self.key
    }

    /// Generates a new random API key with the given prefix
    pub fn generate(prefix: Option<String>) -> Self {
        use rand::{Rng, distr::Alphanumeric};

        let prefix = prefix.unwrap_or_else(|| {
            // Generate a random 7-char prefix if none provided
            let prefix: String = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(PREFIX_SIZE.into())
                .map(char::from)
                .collect();
            prefix
        });

        // Generate 45-character secret key
        let key: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(API_KEY_SIZE.into())
            .map(char::from)
            .collect();

        Self {
            prefix,
            key,
            hash_info: "SHA256".to_string(),
        }
    }

    fn hash(&self) -> Result<String, HashError> {
        let hash = match self.hash_info.as_str() {
            "SHA256" => format!("{:x}", sha2::Sha256::digest(self.key.clone())),
            _ => return Err(HashError::UnknownHashAlgorithm(self.hash_info.to_string())),
        };

        Ok(hash)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HashError {
    #[error("Unknown hash algorithm: {0}")]
    UnknownHashAlgorithm(String),
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("API key must contain a dot separator")]
    MissingSeparator,
    #[error("Prefix must be exactly {PREFIX_SIZE} characters, got {0}")]
    InvalidPrefixLength(usize),
    #[error("Key part cannot be empty")]
    EmptyKey,
}
