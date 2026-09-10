use crate::error::{BackendError, ErrorKind};
use keyring::Entry;

const SERVICE: &str = "com.imagen.studio";

fn entry(provider_id: &str) -> Result<Entry, BackendError> {
    Entry::new(SERVICE, provider_id).map_err(|error| {
        BackendError::new(
            ErrorKind::Server,
            format!("Could not open the system credential store: {error}"),
        )
    })
}

/// Returns the stored key, or the provided key when the store has none.
pub fn resolve(provider_id: &str, provided: &str) -> Result<Option<String>, BackendError> {
    let provided = provided.trim();
    if !provided.is_empty() {
        return Ok(Some(provided.to_string()));
    }
    match entry(provider_id)?.get_password() {
        Ok(value) if !value.trim().is_empty() => Ok(Some(value)),
        Ok(_) => Ok(None),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(error) => Err(BackendError::new(
            ErrorKind::Server,
            format!("Could not read the saved API key: {error}"),
        )),
    }
}

pub fn store(provider_id: &str, key: &str) -> Result<(), BackendError> {
    entry(provider_id)?.set_password(key).map_err(|error| {
        BackendError::new(
            ErrorKind::Server,
            format!("Could not save the API key to the system store: {error}"),
        )
    })
}

pub fn delete(provider_id: &str) -> Result<(), BackendError> {
    match entry(provider_id)?.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(error) => Err(BackendError::new(
            ErrorKind::Server,
            format!("Could not remove the saved API key: {error}"),
        )),
    }
}

pub fn has(provider_id: &str) -> bool {
    matches!(entry(provider_id), Ok(entry) if matches!(entry.get_password(), Ok(value) if !value.trim().is_empty()))
}
