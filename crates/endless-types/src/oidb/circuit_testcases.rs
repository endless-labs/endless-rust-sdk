// Copyright © Endless
// Copyright © Aptos Foundation

//^ This file stores the details associated with a sample OIDB ZK proof. The constants are outputted by
//^ `input_gen.py` in the `oidb-circuit` repo (or can be derived implicitly from that code).

use crate::{
    jwks::rsa::RSA_JWK,
    oidb::{
        base64url_encode_str,
        bn254_circom::{G1Bytes, G2Bytes},
        Claims, Configuration, Groth16Zkp, IdCommitment, OidbPublicKey, OpenIdSig, Pepper,
    },
    transaction::authenticator::EphemeralPublicKey,
};
use endless_crypto::{ed25519::Ed25519PrivateKey, PrivateKey, Uniform};
use once_cell::sync::Lazy;
use ring::signature::RsaKeyPair;
use rsa::{pkcs1::EncodeRsaPrivateKey, pkcs8::DecodePrivateKey};

/// The JWT header, decoded
pub(crate) static SAMPLE_JWT_HEADER_DECODED: Lazy<String> = Lazy::new(|| {
    format!(
        r#"{{"alg":"{}","kid":"{}","typ":"JWT"}}"#,
        SAMPLE_JWK.alg.as_str(),
        SAMPLE_JWK.kid.as_str()
    )
});

/// The JWT header, base64url-encoded
pub(crate) static SAMPLE_JWT_HEADER_B64: Lazy<String> =
    Lazy::new(|| base64url_encode_str(SAMPLE_JWT_HEADER_DECODED.as_str()));

static SAMPLE_NONCE: Lazy<String> = Lazy::new(|| {
    let config = Configuration::new_for_testing();
    OpenIdSig::reconstruct_oauth_nonce(
        SAMPLE_EPK_BLINDER.as_slice(),
        SAMPLE_EXP_DATE,
        &SAMPLE_EPK,
        &config,
    )
    .unwrap()
});

/// TODO(oidb): Use a multiline format here, for diff-friendliness
pub(crate) static SAMPLE_JWT_PAYLOAD_DECODED: Lazy<String> = Lazy::new(|| {
    format!(
        r#"{{"iss":"https://accounts.google.com","azp":"407408718192.apps.googleusercontent.com","aud":"407408718192.apps.googleusercontent.com","sub":"113990307082899718775","hd":"endlesslabs.com","email":"michael@endlesslabs.com","email_verified":true,"at_hash":"bxIESuI59IoZb5alCASqBg","name":"Michael Straka","picture":"https://lh3.googleusercontent.com/a/ACg8ocJvY4kVUBRtLxe1IqKWL5i7tBDJzFp9YuWVXMzwPpbs=s96-c","given_name":"Michael","family_name":"Straka","locale":"en","iat":1700255944,"nonce":"{}","exp":2700259544}}"#,
        SAMPLE_NONCE.as_str()
    )
});

/// Consistent with what is in `SAMPLE_JWT_PAYLOAD_DECODED`
pub(crate) const SAMPLE_JWT_EXTRA_FIELD: &str = r#""family_name":"Straka","#;

/// The JWT parsed as a struct
pub(crate) static SAMPLE_JWT_PARSED: Lazy<Claims> =
    Lazy::new(|| serde_json::from_str(SAMPLE_JWT_PAYLOAD_DECODED.as_str()).unwrap());

/// The JWK under which the JWT is signed, taken from https://token.dev
pub(crate) static SAMPLE_JWK: Lazy<RSA_JWK> = Lazy::new(|| {
    RSA_JWK {
    kid: "test_jwk".to_owned(),
    kty: "RSA".to_owned(),
    alg: "RS256".to_owned(),
    e: "AQAB".to_owned(),
    n: "6S7asUuzq5Q_3U9rbs-PkDVIdjgmtgWreG5qWPsC9xXZKiMV1AiV9LXyqQsAYpCqEDM3XbfmZqGb48yLhb_XqZaKgSYaC_h2DjM7lgrIQAp9902Rr8fUmLN2ivr5tnLxUUOnMOc2SQtr9dgzTONYW5Zu3PwyvAWk5D6ueIUhLtYzpcB-etoNdL3Ir2746KIy_VUsDwAM7dhrqSK8U2xFCGlau4ikOTtvzDownAMHMrfE7q1B6WZQDAQlBmxRQsyKln5DIsKv6xauNsHRgBAKctUxZG8M4QJIx3S6Aughd3RZC4Ca5Ae9fd8L8mlNYBCrQhOZ7dS0f4at4arlLcajtw".to_owned(),
}
});

/// This is the SK from https://token.dev/.
/// To convert it into a JSON, you can use https://irrte.ch/jwt-js-decode/pem2jwk.html
pub(crate) static SAMPLE_JWK_SK: Lazy<RsaKeyPair> = Lazy::new(|| {
    let sk = r#"-----BEGIN PRIVATE KEY-----
MIIEwAIBADANBgkqhkiG9w0BAQEFAASCBKowggSmAgEAAoIBAQDpLtqxS7OrlD/d
T2tuz4+QNUh2OCa2Bat4bmpY+wL3FdkqIxXUCJX0tfKpCwBikKoQMzddt+ZmoZvj
zIuFv9eploqBJhoL+HYOMzuWCshACn33TZGvx9SYs3aK+vm2cvFRQ6cw5zZJC2v1
2DNM41hblm7c/DK8BaTkPq54hSEu1jOlwH562g10vcivbvjoojL9VSwPAAzt2Gup
IrxTbEUIaVq7iKQ5O2/MOjCcAwcyt8TurUHpZlAMBCUGbFFCzIqWfkMiwq/rFq42
wdGAEApy1TFkbwzhAkjHdLoC6CF3dFkLgJrkB7193wvyaU1gEKtCE5nt1LR/hq3h
quUtxqO3AgMBAAECggEBANX6C+7EA/TADrbcCT7fMuNnMb5iGovPuiDCWc6bUIZC
Q0yac45l7o1nZWzfzpOkIprJFNZoSgIF7NJmQeYTPCjAHwsSVraDYnn3Y4d1D3tM
5XjJcpX2bs1NactxMTLOWUl0JnkGwtbWp1Qq+DBnMw6ghc09lKTbHQvhxSKNL/0U
C+YmCYT5ODmxzLBwkzN5RhxQZNqol/4LYVdji9bS7N/UITw5E6LGDOo/hZHWqJsE
fgrJTPsuCyrYlwrNkgmV2KpRrGz5MpcRM7XHgnqVym+HyD/r9E7MEFdTLEaiiHcm
Ish1usJDEJMFIWkF+rnEoJkQHbqiKlQBcoqSbCmoMWECgYEA/4379mMPF0JJ/EER
4VH7/ZYxjdyphenx2VYCWY/uzT0KbCWQF8KXckuoFrHAIP3EuFn6JNoIbja0NbhI
HGrU29BZkATG8h/xjFy/zPBauxTQmM+yS2T37XtMoXNZNS/ubz2lJXMOapQQiXVR
l/tzzpyWaCe9j0NT7DAU0ZFmDbECgYEA6ZbjkcOs2jwHsOwwfamFm4VpUFxYtED7
9vKzq5d7+Ii1kPKHj5fDnYkZd+mNwNZ02O6OGxh40EDML+i6nOABPg/FmXeVCya9
Vump2Yqr2fAK3xm6QY5KxAjWWq2kVqmdRmICSL2Z9rBzpXmD5o06y9viOwd2bhBo
0wB02416GecCgYEA+S/ZoEa3UFazDeXlKXBn5r2tVEb2hj24NdRINkzC7h23K/z0
pDZ6tlhPbtGkJodMavZRk92GmvF8h2VJ62vAYxamPmhqFW5Qei12WL+FuSZywI7F
q/6oQkkYT9XKBrLWLGJPxlSKmiIGfgKHrUrjgXPutWEK1ccw7f10T2UXvgECgYEA
nXqLa58G7o4gBUgGnQFnwOSdjn7jkoppFCClvp4/BtxrxA+uEsGXMKLYV75OQd6T
IhkaFuxVrtiwj/APt2lRjRym9ALpqX3xkiGvz6ismR46xhQbPM0IXMc0dCeyrnZl
QKkcrxucK/Lj1IBqy0kVhZB1IaSzVBqeAPrCza3AzqsCgYEAvSiEjDvGLIlqoSvK
MHEVe8PBGOZYLcAdq4YiOIBgddoYyRsq5bzHtTQFgYQVK99Cnxo+PQAvzGb+dpjN
/LIEAS2LuuWHGtOrZlwef8ZpCQgrtmp/phXfVi6llcZx4mMm7zYmGhh2AsA9yEQc
acgc4kgDThAjD7VlXad9UHpNMO8=
-----END PRIVATE KEY-----"#;

    // TODO(oidb): Hacking around the difficulty of parsing PKCS#8-encoded PEM files with the `pem` crate
    let der = rsa::RsaPrivateKey::from_pkcs8_pem(sk)
        .unwrap()
        .to_pkcs1_der()
        .unwrap();
    RsaKeyPair::from_der(der.as_bytes()).unwrap()
});

pub(crate) const SAMPLE_UID_KEY: &str = "sub";

/// The nonce-committed expiration date (not the JWT `exp`), 12/21/5490
pub(crate) const SAMPLE_EXP_DATE: u64 = 111_111_111_111;

/// ~31,710 years
pub(crate) const SAMPLE_EXP_HORIZON_SECS: u64 = 999_999_999_999;

pub(crate) static SAMPLE_PEPPER: Lazy<Pepper> = Lazy::new(|| Pepper::from_number(76));

pub(crate) static SAMPLE_ESK: Lazy<Ed25519PrivateKey> =
    Lazy::new(Ed25519PrivateKey::generate_for_testing);

pub(crate) static SAMPLE_EPK: Lazy<EphemeralPublicKey> =
    Lazy::new(|| EphemeralPublicKey::ed25519(SAMPLE_ESK.public_key()));

pub(crate) static SAMPLE_EPK_BLINDER: Lazy<Vec<u8>> = Lazy::new(|| vec![42u8]);

pub(crate) static SAMPLE_OIDB_PK: Lazy<OidbPublicKey> = Lazy::new(|| {
    assert_eq!(SAMPLE_UID_KEY, "sub");

    OidbPublicKey {
        iss_val: SAMPLE_JWT_PARSED.oidc_claims.iss.to_owned(),
        idc: IdCommitment::new_from_preimage(
            &SAMPLE_PEPPER,
            SAMPLE_JWT_PARSED.oidc_claims.aud.as_str(),
            SAMPLE_UID_KEY,
            SAMPLE_JWT_PARSED.oidc_claims.sub.as_str(),
        )
        .unwrap(),
    }
});

/// A valid Groth16 proof for the JWT under `SAMPLE_JWK`, where the public inputs have:
///  - uid_key set to `sub`
///  - no override aud
///  - the extra field enabled
///
/// https://github.com/endless-labs/devnet-groth16-keys/commit/02e5675f46ce97f8b61a4638e7a0aaeaa4351f76
pub(crate) static SAMPLE_PROOF: Lazy<Groth16Zkp> = Lazy::new(|| {
    Groth16Zkp::new(
        G1Bytes::new_unchecked(
            "16651743221257656354090458037841396818028112464043092288036348958788946361791",
            "11531873417107922255736984330189413670625492017963151004635379748264109785175",
        )
        .unwrap(),
        G2Bytes::new_unchecked(
            [
                "15841955233281235044985794940553291096287035544527740079985658800770777180211",
                "11782259030192856375021716335600007744831250123506289089655941631693693616189",
            ],
            [
                "17248649131574607527764530278962464654534429229831661532586492853209423291769",
                "7551633097687506616830451831310446552708395586699276024181636099039139142186",
            ],
        )
        .unwrap(),
        G1Bytes::new_unchecked(
            "1366394845425093474391217132694819087133337439129761955205827196688359137647",
            "6817846699374411409872087543089207707409369286515942902469598397112442303399",
        )
        .unwrap(),
    )
});
