use axum::http::StatusCode;
use hex::FromHex;
use ed25519_dalek::{Signature, VerifyingKey};

/// 校验request_idrequest_id
pub fn validate_request_id(request_id: &String) -> Result<bool, (StatusCode, String)> {
    if request_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing request_id".to_owned()));
    }
    // TODO Redis校验
    Ok(true)
}

/// 校验签名
pub fn validate_signature(pub_key: &String, request_id: &String, signature: &String) -> Result<bool, (StatusCode, String)> {
    if pub_key.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    }
    if signature.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    }
    if request_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Missing request_id".to_owned()));
    }

    let pub_key_result = <[u8;32]>::from_hex(&pub_key);
    if pub_key_result.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    }
    let verifying_key = VerifyingKey::from_bytes(&pub_key_result.unwrap()).unwrap();

    let signature_result = <[u8;64]>::from_hex(&signature);
    if signature_result.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    }
    let signature = Signature::from_bytes(&signature_result.unwrap());

    let verify_result = verifying_key.verify_strict(request_id.as_bytes(), &signature);

    // 校验错误
    if verify_result.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    }
    Ok(true)
}