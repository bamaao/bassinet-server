// use rcgen::{generate_simple_self_signed, CertifiedKey};

// fn main() {
//     // Generate a certificate that's valid for "localhost" and "hello.world.example"
//     let subject_alt_names = vec!["bassinet.app".to_string(),
//         "localhost".to_string()];

//     let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
//     println!("{}", cert.pem());
//     println!("{}", key_pair.serialize_pem());
// }
