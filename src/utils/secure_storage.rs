use keyring::Entry;
use turbo::{auth::AuthResponse, errors::ResponseError, types::PublicUser};

pub enum SecureStoreError {
    EntryCreation(keyring::Error),
    CredentialRetrieving(keyring::Error),
    CredentialWriting(keyring::Error),
    CredentialDeletion(keyring::Error),
}

pub fn get_token_from_secure_storage() -> Result<String, SecureStoreError> {
    let entry = Entry::new("fictional-potato", "refresh_token")
        .map_err(|e| SecureStoreError::EntryCreation(e))?;
    let token = entry
        .get_password()
        .map_err(|e| SecureStoreError::CredentialRetrieving(e))?;
    Ok(token)
}

pub fn set_token_from_secure_storage(token: Option<String>) -> Result<(), SecureStoreError> {
    let entry = Entry::new("fictional-potato", "refresh_token")
        .map_err(|e| SecureStoreError::EntryCreation(e))?;
    if token.is_none() {
        entry
            .delete_credential()
            .map_err(|e| SecureStoreError::CredentialDeletion(e))?;
    } else {
        entry
            .set_password(token.unwrap().as_str())
            .map_err(|e| SecureStoreError::CredentialWriting(e))?;
    }
    Ok(())
}

pub async fn get_user_with_token() -> Result<PublicUser, String> {
    println!("Attempting to get user from a refresh_token");
    let token = get_token_from_secure_storage().map_err(|e| "No!")?;

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/auth/refresh")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if res.status().is_success() {
        let success_body = res
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse successful login: {}", e))?;

        println!("res: {success_body:?}");
        set_token_from_secure_storage(Some(success_body.refresh_token)).map_err(|e| "Errrrrr")?;
        Ok(success_body.user)
    } else {
        let error_body = res
            .json::<ResponseError>()
            .await
            .map_err(|e| format!("Failed to parse error response: {}", e))?;
        Err(error_body.error.message)
    }
}
