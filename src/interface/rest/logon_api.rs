use axum::{http::StatusCode, Json};
use jsonwebtoken::{encode, Header};
// use hex::FromHex;
use crate::{application::command_service::account_application_service, domain::repository::account_repository, infrastructure::jwt::{AuthBody, AuthError, Claims, KEYS}, interface::rest::validate::validate_signature, utils};
// use ed25519_dalek::{Signature, VerifyingKey};
use super::dto::logon::{SignInPayload, SignUpPayload};

/// 用户登录
pub async fn sign_in(Json(payload): Json<SignInPayload>) -> Result<Json<AuthBody>, (StatusCode, String)> {
    tracing::debug!("{}", serde_json::to_string(&payload).unwrap());

    // if payload.pub_key.is_empty() {
    //     return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    // }
    // if payload.sig.is_empty() {
    //     return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    // }
    // if payload.request_id.is_empty() {
    //     return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    // }

    // let pub_key_result = <[u8;32]>::from_hex(&payload.pub_key);
    // if pub_key_result.is_err() {
    //     return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    // }
    // let verifying_key = VerifyingKey::from_bytes(&pub_key_result.unwrap()).unwrap();

    // let signature_result = <[u8;64]>::from_hex(&payload.sig);
    // if signature_result.is_err() {
    //     return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    // }
    // let signature = Signature::from_bytes(&signature_result.unwrap());

    // let verify_result = verifying_key.verify_strict(payload.request_id.as_bytes(), &signature);

    // // 校验错误
    // if verify_result.is_err() {
    //     return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    // }

    let validate_result = validate_signature(&payload.pub_key, &payload.request_id, &payload.sig);
    if validate_result.is_err() {
        let (code, message) = validate_result.err().unwrap();
        return Err((code, message));
    }

    // 数据库查询账户
    let accounts = account_repository::find_by_pubkey(&payload.pub_key).await;
    if accounts.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid Account".to_owned()));
    }
    
    // 登录成功，返回登录成功信息
    let claims = Claims {
        pubkey: payload.pub_key.clone(),
        // Mandatory expiry time as UTC timestamp
        exp: utils::current_seconds() + 24 * 60 * 60
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation);

    if token.is_err() {
        return Err( (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_owned()));
    }

    // Send the authorized token
    Ok(Json(AuthBody::new(token.unwrap())))
}

/// 用户注册
pub async fn sign_up(Json(payload): Json<SignUpPayload>) -> Result<Json<AuthBody>, (StatusCode, String)> {
    tracing::debug!("{}", serde_json::to_string(&payload).unwrap());

    // if payload.pub_key.is_empty() {
    //     return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    // }
    // if payload.sig.is_empty() {
    //     return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    // }
    // if payload.request_id.is_empty() {
    //     return Err((StatusCode::BAD_REQUEST, "Missing credentials".to_owned()));
    // }

    // let pub_key_result = <[u8;32]>::from_hex(&payload.pub_key);
    // if pub_key_result.is_err() {
    //     return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    // }
    // let verifying_key = VerifyingKey::from_bytes(&pub_key_result.unwrap()).unwrap();

    // let signature_result = <[u8;64]>::from_hex(&payload.sig);
    // if signature_result.is_err() {
    //     return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    // }
    // let signature = Signature::from_bytes(&signature_result.unwrap());

    // let verify_result = verifying_key.verify_strict(payload.request_id.as_bytes(), &signature);

    // // 校验错误
    // if verify_result.is_err() {
    //     return Err((StatusCode::UNAUTHORIZED, "Wrong credentials".to_owned()));
    // }

    let validate_result = validate_signature(&payload.pub_key, &payload.request_id, &payload.sig);
    if validate_result.is_err() {
        let (code, message) = validate_result.err().unwrap();
        return Err((code, message));
    }

    // 账户信息校验并入库
    let account_id = account_application_service::register_account(&payload).await;
    if account_id.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, account_id.err().unwrap().to_string()));
    }
    
    // 注册成功，返回登录成功信息
    let claims = Claims {
        pubkey: payload.pub_key,
        // Mandatory expiry time as UTC timestamp
        exp: utils::current_seconds() + 24 * 60 * 60
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation);

    if token.is_err() {
        return Err( (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error".to_owned()));
    }

    // Send the authorized token
    Ok(Json(AuthBody::new(token.unwrap())))
}