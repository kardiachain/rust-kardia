use kp_core::H256;

use crate::{empty_hash, get_split_point, inner_hash, leaf_hash};
/// Error for  proof.
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum Error {
    #[cfg_attr(feature = "std", error("Invalid root {0:x?}, expected {1:x?}"))]
    RootMismatch(H256, H256),
    #[cfg_attr(feature = "std", error("Invalid leaf {0:x?}, expected {1:x?}"))]
    LeafMismatch(H256, H256),
}

#[derive(Default, Debug, PartialEq)]
pub struct Proof {
    total: usize,
    index: usize,
    left_hash: H256,
    aunts: Vec<H256>,
}

impl Proof {
    fn new(total: usize, index: usize, left_hash: H256, aunts: Vec<H256>) -> Proof {
        Proof {
            total: total,
            index: index,
            left_hash: left_hash,
            aunts: aunts,
        }
    }

    fn compute_hash(&self) -> H256 {
        let res =
            compute_hash_from_aunt(self.index, self.total, self.left_hash, self.aunts.clone());
        match res {
            Some(hash) => hash,
            None => H256::default(),
        }
    }

    pub fn verify(&self, root_hash: H256, leaf: &[u8]) -> Result<bool, Error> {
        let leaf_hash = leaf_hash(leaf);
        if !leaf_hash.eq(&self.left_hash) {
            Err(Error::LeafMismatch(self.left_hash, leaf_hash))
        } else if !self.compute_hash().eq(&root_hash) {
            Err(Error::RootMismatch(self.left_hash, leaf_hash))
        } else {
            Ok(true)
        }
    }
}

fn compute_hash_from_aunt(
    idx: usize,
    total: usize,
    leaf_hash: H256,
    inner_hashs: Vec<H256>,
) -> Option<H256> {
    match total {
        0 => panic!("Cannot call compute_hash_from_aunt() with 0 total"),
        1 => {
            if inner_hashs.len() != 0 {
                return None;
            }
            return Some(leaf_hash);
        }
        _ => {
            if inner_hashs.len() == 0 {
                return None;
            }

            let num_left = get_split_point(total);
            let k = inner_hashs.len() + 1;
            if idx < num_left {
                let left_hash =
                    compute_hash_from_aunt(idx, num_left, leaf_hash, inner_hashs[..k].to_vec());
                match left_hash {
                    Some(left_hash) => Some(inner_hash(
                        left_hash.as_bytes(),
                        inner_hashs[inner_hashs.len()].as_bytes(),
                    )),
                    None => return None,
                }
            } else {
                let right_hash =
                    compute_hash_from_aunt(idx, num_left, leaf_hash, inner_hashs[k..].to_vec());
                match right_hash {
                    Some(right_hash) => Some(inner_hash(
                        right_hash.as_bytes(),
                        inner_hashs[inner_hashs.len()].as_bytes(),
                    )),
                    None => return None,
                }
            }
        }
    }
}

pub fn proof_from_byte_vectors(byte_vecs: Vec<Vec<u8>>) -> (H256, Vec<Proof>) {
    let total_items = byte_vecs.len();
    let (trails, root_spn) = trails_from_byte_vectors(byte_vecs);
    let root_hash = root_spn.hash;
    let mut proofs: Vec<Proof> = Vec::with_capacity(total_items);
    for (i, trail) in trails.iter().enumerate() {
        proofs.push(Proof::new(
            total_items,
            i,
            trail.hash,
            trail.clone().flatten_aunts(),
        ));
    }
    (root_hash, proofs)
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct ProofNode {
    pub hash: H256,
    pub parent: Option<Box<ProofNode>>,
    pub left: Option<Box<ProofNode>>,
    pub right: Option<Box<ProofNode>>,
}

impl ProofNode {
    pub fn new(hash: H256) -> ProofNode {
        ProofNode {
            hash: hash,
            parent: None,
            left: None,
            right: None,
        }
    }

    pub fn set_parent(&mut self, root: ProofNode) {
        self.parent = Some(Box::new(root));
    }

    pub fn flatten_aunts(self) -> Vec<H256> {
        let mut inner_hashes: Vec<H256> = Vec::new();
        let mut spn = Some(Box::new(self));
        loop {
            match spn {
                Some(_spn) => {
                    match _spn.left {
                        Some(left) => {
                            inner_hashes.push(left.hash);
                        }
                        None => (),
                    }
                    match _spn.right {
                        Some(right) => {
                            inner_hashes.push(right.hash);
                        }
                        None => (),
                    }
                    spn = _spn.parent;
                }
                None => break,
            }
        }
        inner_hashes
    }
}

fn trails_from_byte_vectors(byte_vecs: Vec<Vec<u8>>) -> (Vec<ProofNode>, ProofNode) {
    let length = byte_vecs.len();
    match length {
        0 => {
            let empty_proof_node = ProofNode::new(empty_hash());
            (vec![empty_proof_node.clone()], empty_proof_node.clone())
        }
        1 => {
            let proof_node = ProofNode::new(leaf_hash(byte_vecs[0].as_slice()));
            (vec![proof_node.clone()], proof_node)
        }
        _ => {
            let k = get_split_point(byte_vecs.len());
            let (mut lefts, mut left_root) = trails_from_byte_vectors(byte_vecs[..k].to_vec());
            let (mut rights, mut right_root) = trails_from_byte_vectors(byte_vecs[k..].to_vec());
            let root_hash = inner_hash(&left_root.hash.as_bytes(), &right_root.hash.as_bytes());
            let root = ProofNode::new(root_hash);
            left_root.set_parent(root.clone());
            right_root.set_parent(root.clone());
            left_root.right = Some(Box::new(right_root));
            rights.append(&mut lefts);
            (rights, root.clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use kp_core::hexdisplay::AsBytesRef;
    use subtle_encoding::hex;
    use super::*; // TODO: use non-subtle ?

    #[test]
    fn test_proof_from_byte_vectors() {
        let hash_string = "3639618e90314d0feb7ce289d97468263ca4af996ffc621399d092e1f12aa1dd";
        let hash_bytes = &hex::decode(hash_string).unwrap();
        let data = vec![vec![46]];

        let (root, proofs) = proof_from_byte_vectors(data);
        assert_eq!(root, H256::from_slice(hash_bytes));
        assert!(proofs.len() == 1);
    }

    #[test]
    fn test_verify() {
        let data = vec![vec![46]];
        let (root, proofs) = proof_from_byte_vectors(data.clone());
        let res = proofs[0].verify(root, data[0].as_bytes_ref());
        assert!(res.unwrap());
    }
}
