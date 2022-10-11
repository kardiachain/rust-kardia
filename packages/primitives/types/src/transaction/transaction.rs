use kp_core::{hash::H256, keccak_256, uint::U256};
use rlp::{DecoderError, Rlp, RlpStream};

type Bytes = Vec<u8>;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub from: H256,
    /// Nonce.
    pub nonce: U256,
    /// Gas price for non 1559 transactions. MaxFeePerGas for 1559 transactions.
    pub gas_price: U256,
    /// Gas paid up front for transaction execution.
    pub gas: U256,
    /// Transfered value.s
    pub value: U256,
    /// Transaction data.
    pub data: Bytes,
    /// Action, can be either call or contract create.
    pub action: Action,
    // Signature
    pub v: u8,
    pub r: U256,
    pub s: U256,
    pub hash: H256,
}

/// Transaction action type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Create creates new contract.
    Create,
    /// Calls contract at given address.
    /// In the case of a transfer, this is the receiver's address.'
    Call(H256),
}

impl rlp::Encodable for Action {
    fn rlp_append(&self, s: &mut RlpStream) {
        match *self {
            Action::Create => s.append_internal(&""),
            Action::Call(ref addr) => s.append_internal(addr),
        };
    }
}


impl Default for Action {
    fn default() -> Action {
        Action::Create
    }
}

impl rlp::Decodable for Action {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        if rlp.is_empty() {
            if rlp.is_data() {
                Ok(Action::Create)
            } else {
                Err(DecoderError::RlpExpectedToBeData)
            }
        } else {
            Ok(Action::Call(rlp.as_val()?))
        }
    }
}

impl rlp::Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas);
        s.append(&self.action);
        s.append(&self.value);
        s.append(&self.data);
        s.append(&self.v);
        s.append(&self.r);
        s.append(&self.s);
    }
}

impl rlp::Decodable for Transaction {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        Ok(Transaction {
            from: H256::default(),
            nonce: rlp.val_at(0)?,
            gas_price: rlp.val_at(1)?,
            gas: rlp.val_at(2)?,
            action: rlp.val_at(3)?,
            value: rlp.val_at(4)?,
            data: rlp.val_at(5)?,
            v: rlp.val_at(6)?,
            r: rlp.val_at(7)?,
            s: rlp.val_at(8)?,
            hash: keccak_256(rlp.as_raw()).into(),
        })
    }
}

impl Transaction {
    pub fn hash(&self) -> H256 {
        return self.hash;
    }
}
