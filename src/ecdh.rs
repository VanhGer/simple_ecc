use p3_field::AbstractField;
use p3_goldilocks::Goldilocks;
use rand::Rng;
use crate::curve::Elliptic;
use crate::errors::GeneralError;
use crate::P;
use crate::point::Point;

// EncryptedMsg the encrypted message with ciphertext, nonce and ciphertext public key. /
pub struct EncryptedMsg {
    cipher_text: String,
    nonce: Vec<u8>,
    cipher_text_pubkey: Point
}


// ECDH an implementation of a hybrid encryption scheme by using the ECDH (Elliptic Curve Diffie–Hellman) key exchange scheme
pub struct Ecdh {
    ec: Elliptic,
    private_key: Goldilocks,
}

impl Ecdh {
    pub fn new(ec: Elliptic, private_key: Goldilocks) -> Self{
        Self {
            ec,
            private_key
        }
    }

    pub fn pub_key(&self) -> Point {
        self.ec.generate(self.private_key)
    }

    // https://cryptobook.nakov.com/asymmetric-key-ciphers/ecc-encryption-decryption
    // generates and returns a shared ECC key and a ciphertext public key
    // using a received public key.
    pub fn calculate_encryption_key(&self, pub_key: &Point) -> (Point, Point) {
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(1..P);
        let ciphertext_priv_key = Goldilocks::from_canonical_u64(random_value);

        let shared_ecc_key = self.ec.calculate(pub_key, ciphertext_priv_key);

        let ciphertext_pub_key = self.ec.generate(ciphertext_priv_key);

        (shared_ecc_key, ciphertext_pub_key)
    }

    // returns the sharedECCKey which is used for the decryption.
    pub fn calculate_decryption_key(&self, ciphertext_pub_key: Point) -> Point {
        let shared_ecc_key = self.ec.calculate(&ciphertext_pub_key, self.private_key);
        shared_ecc_key
    }

    // pub fn encrypt(&self, msg: &String, pub_key: &Point) -> Result<EncryptedMsg, GeneralError> {
    //     let  (shared_ecc_key, ciphertext_pub_key) = self.calculate_encryption_key(pub_key);
    //
    //
    // }
}




