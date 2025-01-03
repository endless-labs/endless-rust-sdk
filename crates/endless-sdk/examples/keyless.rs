use endless_crypto::{
    ed25519::{Ed25519PrivateKey, Ed25519Signature},
    PrivateKey, SigningKey,
};
use endless_ledger::AuthenticationKey;
use endless_rest_client::error::RestError;
use endless_sdk::{rest_client, transaction_builder::TransactionBuilder, types::LocalAccount};
use endless_types::{
    account_address::AccountAddress,
    chain_id::ChainId,
    oidb::{
        self, G1Bytes, G2Bytes, Groth16Zkp, IdCommitment, OidbPublicKey, OidbSignature, OpenIdSig,
        Pepper, SignedGroth16Zkp,
    },
    transaction::{
        authenticator::{
            AccountAuthenticator, AnyPublicKey, AnySignature, EphemeralPublicKey,
            EphemeralSignature, SingleKeyAuthenticator, TransactionAuthenticator,
        },
        EntryFunction, RawTransaction, RawTransactionWithData, SignedTransaction,
        TransactionPayload,
    },
};
use move_core_types::{identifier::Identifier, language_storage::ModuleId};
use rand::{thread_rng, RngCore};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    google_example().await;
    Ok(())
}

#[allow(dead_code)]
async fn google_example() {
    // let client = rest_client::Client::new(Url::parse("https://rpc-testnet.endless.link").unwrap());
    let client = rest_client::Client::new_testnet();
    let mut rng = thread_rng();
    // Blinder is a random 31-bytes
    let mut blinder = [0u8; 31];
    rng.fill_bytes(&mut blinder);
    println!("blinder: {}", hex::encode(blinder));
    // Expiry for ephemeral key pair
    let expiry = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 14 * 24 * 60 * 60; // 14 days
    println!("expiry: {expiry}");
    // Generate ephemeral key pair
    let mut esk_bytes = [0u8; 32];
    rng.fill_bytes(&mut esk_bytes);
    let esk = Ed25519PrivateKey::try_from(&esk_bytes[..]).unwrap();
    println!("ephemeral private key: {}", hex::encode(esk_bytes));
    let epk = EphemeralPublicKey::ed25519(esk.public_key());
    println!("ephemeral public key: {}", hex::encode(epk.to_bytes()));
    // Calculate nonce
    let nonce = OpenIdSig::reconstruct_oauth_nonce(
        &blinder,
        expiry,
        &epk,
        &oidb::Configuration::new_for_devnet(),
    )
    .unwrap();
    println!("nonce: {nonce}");
    let url = format!("https://accounts.google.com/o/oauth2/v2/auth/oauthchooseaccount?redirect_uri=https%3A%2F%2Fdevelopers.google.com%2Foauthplayground&prompt=consent&response_type=code&client_id=407408718192.apps.googleusercontent.com&scope=profile&access_type=offline&service=lso&o2v=2&theme=glif&flowName=GeneralOAuthFlow&nonce={nonce}");
    println!("Visit the following link in your browser:");
    println!("\n{url}\n");
    println!("Click `Exchange authorization code for tokens` button in left side of the page.");
    println!("And paste the id_token here: ");
    let mut id_token = String::new();
    std::io::stdin()
        .read_line(&mut id_token)
        .expect("Failed to read id_token");
    let id_token = id_token.trim();
    let [jwt_header_b64, jwt_payload_b64, _]: [&str; 3] = id_token
        .split('.')
        .collect::<Vec<&str>>()
        .try_into()
        .expect("Invalid id_token");

    let jwt_payload: IdTokenPayload = serde_json::from_slice(
        &base64_url::decode(jwt_payload_b64).expect("Failed to decode jwt_payload_b64"),
    )
    .expect("Failed to parse jwt_payload");

    // Check nonce
    if jwt_payload.nonce != nonce {
        panic!(
            "Invalid nonce, expected: {nonce} but got: {}",
            jwt_payload.nonce
        );
    }

    let pepper = Pepper::new([0u8; 31]);
    let idc = IdCommitment::new_from_preimage(&pepper, &jwt_payload.aud, "sub", &jwt_payload.sub)
        .expect("Failed to create idc");
    let oidb_pub = OidbPublicKey {
        iss_val: jwt_payload.iss,
        idc,
    };

    // Get address
    let address =
        AuthenticationKey::any_key(AnyPublicKey::oidb(oidb_pub.clone())).account_address();
    println!("address: {}", address.to_bs58_string());
    let seq = match client.get_account(address).await {
        Ok(account) => account.inner().sequence_number,
        Err(RestError::Api(err)) => {
            if err.status_code == 404 {
                0
            } else {
                panic!("Failed to get account: {err}");
            }
        },
        Err(e) => panic!("Failed to get account: {e}"),
    };
    println!("sequence number: {seq}");

    // Get zk proof
    let req = ProofRequest {
        jwt_b64: id_token.to_string(),
        epk: bcs::to_bytes(&epk).unwrap(),
        epk_blinder: blinder.to_vec(),
        exp_date_secs: expiry,
        exp_horizon_secs: 10_000_000,
        pepper: pepper.to_bytes().to_vec(),
        uid_key: "sub".to_string(),
        extra_field: None,
        aud_override: None,
    };
    println!("Fetching zk proof...");
    let resp: ProofResponse = reqwest::Client::default()
        .post("https://webwallet.endless.link/prove")
        .json(&req)
        .send()
        .await
        .expect("fetch failed")
        .json()
        .await
        .expect("parse proof failed");

    println!("zk proof: {resp:?}");

    let training_wheels_signature: EphemeralSignature =
        bcs::from_bytes(&hex::decode(resp.training_wheels_signature).unwrap()).unwrap();
    let proof_a = G1Bytes::new_from_vec(hex::decode(resp.proof.a).unwrap()).unwrap();
    let proof_b = G2Bytes::new_from_vec(hex::decode(resp.proof.b).unwrap()).unwrap();
    let proof_c = G1Bytes::new_from_vec(hex::decode(resp.proof.c).unwrap()).unwrap();
    let proof = Groth16Zkp::new(proof_a, proof_b, proof_c);
    let proof_sig = esk.sign(&proof).expect("Failed to sign proof");

    // Construct transaction
    let chain_id = client
        .get_ledger_information()
        .await
        .unwrap()
        .inner()
        .chain_id;
    let payload = if seq == 0 {
        println!("Constructing faucet transaction...");
        faucet_payload(address)
    } else {
        println!("Constructing transfer transaction...");
        transfer_payload()
    };
    let raw_txn = TransactionBuilder::new(payload, now() + 30, ChainId::new(chain_id))
        .sender(address)
        .sequence_number(seq)
        .max_gas_amount(3000)
        .gas_unit_price(100)
        .build();

    // Sign transaction use esk
    let signature_raw = esk.sign(&raw_txn).expect("Failed to sign transaction");
    let signed_zkp = SignedGroth16Zkp {
        proof,
        non_malleability_signature: EphemeralSignature::ed25519(proof_sig),
        exp_horizon_secs: 10_000_000,
        extra_field: None,
        override_aud_val: None,
        training_wheels_signature: Some(training_wheels_signature),
    };
    let google_sign = |signature: Ed25519Signature| {
        let sig = oidb::ZkpOrOpenIdSig::Groth16Zkp(signed_zkp.clone());
        // The whole txn signature
        OidbSignature {
            sig,
            jwt_header_b64: jwt_header_b64.to_string(),
            exp_timestamp_secs: expiry,
            ephemeral_pubkey: epk.clone(),
            ephemeral_signature: EphemeralSignature::ed25519(signature),
        }
    };

    let signature = google_sign(signature_raw);
    let txn = SignedTransaction::new_oidb(raw_txn, oidb_pub.clone(), signature);

    // Submit txn
    let tx = client
        .submit_and_wait_bcs(&txn)
        .await
        .expect("Submit txn failed")
        .into_inner();
    println!("Success, txn version: {}", tx.version);

    // Example for multi auth key with google account
    // Alice is a normal ed25519 account
    // Alice set auth key to [alice, google], and num_signatures_required to 2
    let alice = LocalAccount::generate(&mut rng);
    // Funding alice
    let txn_builder = TransactionBuilder::new(
        faucet_payload(alice.address()),
        now() + 30,
        ChainId::new(chain_id),
    );
    let signed = alice.sign_with_transaction_builder(txn_builder);
    println!("Funding for alice");
    client.submit_and_wait_bcs(&signed).await.unwrap();
    // Add auth key
    let raw_txn = RawTransaction::new_entry_function(
        alice.address(),
        alice.increment_sequence_number(),
        EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("account").unwrap()),
            Identifier::new("add_authentication_key").unwrap(),
            vec![],
            vec![],
        ),
        10000,
        100,
        now() + 30,
        ChainId::new(chain_id),
    );
    let message = RawTransactionWithData::new_multi_agent(raw_txn.clone(), vec![address]);
    let alice_singnature = alice.private_key().sign(&message).unwrap();
    let alice_authenticator =
        AccountAuthenticator::ed25519(alice.public_key().clone(), alice_singnature);
    let google_signature = google_sign(esk.sign(&message).unwrap());
    let google_authenticator = AccountAuthenticator::single_key(SingleKeyAuthenticator::new(
        AnyPublicKey::oidb(oidb_pub.clone()),
        AnySignature::oidb(google_signature),
    ));
    let signed =
        SignedTransaction::new_multi_agent(raw_txn, alice_authenticator, vec![address], vec![
            google_authenticator,
        ]);
    println!("Adding auth key");
    client.submit_and_wait_bcs(&signed).await.unwrap();
    // Update num signatures required
    let txn_builder = TransactionBuilder::new(
        TransactionPayload::EntryFunction(EntryFunction::new(
            ModuleId::new(AccountAddress::ONE, Identifier::new("account").unwrap()),
            Identifier::new("set_num_signatures_required").unwrap(),
            vec![],
            vec![bcs::to_bytes(&2u64).unwrap()],
        )),
        now() + 30,
        ChainId::new(chain_id),
    );
    let signed = alice.sign_with_transaction_builder(txn_builder);
    println!("Updating num signatures required");
    client.submit_and_wait_bcs(&signed).await.unwrap();
    // Submit txn use multi auth key
    let raw_txn = RawTransaction::new(
        alice.address(),
        alice.increment_sequence_number(),
        transfer_payload(),
        3000,
        100,
        now() + 30,
        ChainId::new(chain_id),
    );
    let alice_signature = alice.private_key().sign(&raw_txn).unwrap();
    let google_signature = google_sign(esk.sign(&raw_txn).unwrap());
    let authenticator =
        TransactionAuthenticator::single_sender(AccountAuthenticator::multi_auth_key(
            vec![
                AnyPublicKey::ed25519(alice.public_key().clone()),
                AnyPublicKey::oidb(oidb_pub),
            ],
            vec![
                AnySignature::ed25519(alice_signature),
                AnySignature::oidb(google_signature),
            ],
        ));
    let signed = SignedTransaction::new_with_authenticator(raw_txn, authenticator);
    println!("Submitting txn");
    let tx = client
        .submit_and_wait_bcs(&signed)
        .await
        .expect("Submit txn failed")
        .into_inner();
    println!("Success, txn version: {}", tx.version);
}

#[inline]
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug, Serialize, Deserialize)]
struct IdTokenPayload {
    aud: String,
    iss: String,
    sub: String,
    nonce: String,
}

#[derive(Debug, Serialize)]
pub struct ProofRequest {
    pub jwt_b64: String,
    #[serde(serialize_with = "hex_encode")]
    pub epk: Vec<u8>,
    #[serde(serialize_with = "hex_encode")]
    pub epk_blinder: Vec<u8>,
    pub exp_date_secs: u64,
    pub exp_horizon_secs: u64,
    #[serde(serialize_with = "hex_encode")]
    pub pepper: Vec<u8>,
    pub uid_key: String,
    pub extra_field: Option<String>,
    pub aud_override: Option<String>,
}

fn hex_encode<S>(value: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&hex::encode(value))
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ProofResponse {
    proof: Proof,
    public_inputs_hash: String,
    training_wheels_signature: String,
}

#[derive(Debug, Deserialize)]
struct Proof {
    a: String,
    b: String,
    c: String,
}

#[allow(dead_code)]
fn print_verification_key(path: &str) {
    let content = std::fs::read(path).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&content).unwrap();
    let a1 = G1Bytes::new_unchecked(
        value["vk_alpha_1"][0].as_str().unwrap(),
        value["vk_alpha_1"][1].as_str().unwrap(),
    )
    .unwrap();
    println!("a1: {}", hex::encode(bcs::to_bytes(&a1).unwrap()));
    let b2 = G2Bytes::new_unchecked(
        [
            value["vk_beta_2"][0][0].as_str().unwrap(),
            value["vk_beta_2"][0][1].as_str().unwrap(),
        ],
        [
            value["vk_beta_2"][1][0].as_str().unwrap(),
            value["vk_beta_2"][1][1].as_str().unwrap(),
        ],
    )
    .unwrap();
    println!("b2: {}", hex::encode(bcs::to_bytes(&b2).unwrap()));
    let g2 = G2Bytes::new_unchecked(
        [
            value["vk_gamma_2"][0][0].as_str().unwrap(),
            value["vk_gamma_2"][0][1].as_str().unwrap(),
        ],
        [
            value["vk_gamma_2"][1][0].as_str().unwrap(),
            value["vk_gamma_2"][1][1].as_str().unwrap(),
        ],
    )
    .unwrap();
    println!("g2: {}", hex::encode(bcs::to_bytes(&g2).unwrap()));
    let d2 = G2Bytes::new_unchecked(
        [
            value["vk_delta_2"][0][0].as_str().unwrap(),
            value["vk_delta_2"][0][1].as_str().unwrap(),
        ],
        [
            value["vk_delta_2"][1][0].as_str().unwrap(),
            value["vk_delta_2"][1][1].as_str().unwrap(),
        ],
    )
    .unwrap();
    println!("d2: {}", hex::encode(bcs::to_bytes(&d2).unwrap()));
    let ic1 = G1Bytes::new_unchecked(
        value["IC"][0][0].as_str().unwrap(),
        value["IC"][0][1].as_str().unwrap(),
    )
    .unwrap();
    let ic2 = G1Bytes::new_unchecked(
        value["IC"][1][0].as_str().unwrap(),
        value["IC"][1][1].as_str().unwrap(),
    )
    .unwrap();
    println!(
        "gamma_abc_g1: [\n{},\n{},\n]",
        hex::encode(bcs::to_bytes(&ic1).unwrap()),
        hex::encode(bcs::to_bytes(&ic2).unwrap())
    );
}

fn faucet_payload(addr: AccountAddress) -> TransactionPayload {
    TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(AccountAddress::ONE, Identifier::new("faucet").unwrap()),
        Identifier::new("fund").unwrap(),
        vec![],
        vec![addr.to_vec()],
    ))
}

fn transfer_payload() -> TransactionPayload {
    TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            AccountAddress::ONE,
            Identifier::new("endless_coin").unwrap(),
        ),
        Identifier::new("transfer").unwrap(),
        vec![],
        vec![AccountAddress::ONE.to_vec(), bcs::to_bytes(&1u128).unwrap()],
    ))
}
