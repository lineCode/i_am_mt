use lazy_static::lazy_static;
use openssl::{
    bn::{BigNum, BigNumContext, BigNumRef},
    pkey::Public,
    rsa::Rsa,
};

lazy_static! {
    static ref TG_SERVER_PUBLIC_KEY: Rsa<Public> = telegram_server_public_key();
}

fn telegram_server_public_key() -> Rsa<Public> {
    let document = br#"-----BEGIN RSA PUBLIC KEY-----
MIIBCgKCAQEAwVACPi9w23mF3tBkdZz+zwrzKOaaQdr01vAbU4E1pvkfj4sqDsm6
lyDONS789sVoD/xCS9Y0hkkC3gtL1tSfTlgCMOOul9lcixlEKzwKENj1Yz/s7daS
an9tqw3bfUV/nqgbhGX81v/+7RFAEd+RwFnK7a+XYl9sluzHRyVVaTTveB2GazTw
Efzk2DWgkBluml8OREmvfraX3bkHZJTKX4EQSjBbbdJ2ZXIsRrYOXfaA+xayEGB+
8hdlLmAjbCVfaigxX0CDqWeR1yFL9kwd9P0NsZRPsmoqVwMbMu7mStFai6aIhc3n
Slv8kg9qv1m6XHVQY3PnEw+QQtqSIXklHwIDAQAB
-----END RSA PUBLIC KEY-----"#;
    openssl::rsa::Rsa::public_key_from_pem_pkcs1(document).unwrap()
}

/// Text book RSA, only work for AuthKey generator
pub fn rsa(data: &[u8; 255]) -> [u8; 256] {
    let mut context = BigNumContext::new().unwrap();

    let n: &BigNumRef = TG_SERVER_PUBLIC_KEY.n();
    let e: &BigNumRef = TG_SERVER_PUBLIC_KEY.e();
    let z = BigNum::from_slice(data).unwrap();
    let mut c = BigNum::new().unwrap();
    c.mod_exp(&z, e, n, &mut context).unwrap();

    let result = c.to_vec();
    let mut output = [0u8; 256];
    output.copy_from_slice(&result);
    output
}
