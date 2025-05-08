use serde::{Deserialize, Serialize};
use ssi::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct MyClaims {
    name: String,
    email: String,
}
#[tokio::main]
async fn main() {
    // Create JWT claims from our custom ("private") claims.
    let claims = JWTClaims::from_private_claims(MyClaims {
        name: "John Smith".to_owned(),
        email: "john.smith@example.org".to_owned(),
    });

    // Create a random signing key, and turn its public part into a DID URL.
    let mut key = JWK::generate_p256(); // require the `p256` feature.
    let did = DIDJWK::generate_url(&key.to_public());
    key.key_id = Some(did.into());

    // Sign the claims
    let jwt = claims.sign(&key).await.expect("signature failed");

    // Create a verification method resolver, which will be in charge of
    // decoding the DID back into a public key.
    let vm_resolver = DIDJWK.into_vm_resolver::<AnyJwkMethod>();

    // Setup the verification parameters.
    let params = VerificationParameters::from_resolver(vm_resolver);

    // Verify the JWT.
    assert!(
        jwt.verify(&params)
            .await
            .expect("verification failed")
            .is_ok()
    );

    // Print the JWT.
    println!("{jwt}")
}
