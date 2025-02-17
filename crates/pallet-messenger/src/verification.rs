use codec::{Decode, Encode};
use frame_support::PalletError;
use hash_db::Hasher;
use scale_info::TypeInfo;
use sp_core::storage::StorageKey;
use sp_trie::{read_trie_value, LayoutV1, StorageProof};
use std::marker::PhantomData;

/// Verification error.
#[derive(Debug, PartialEq, Eq, Encode, Decode, PalletError, TypeInfo)]
pub enum VerificationError {
    /// Emits when the expected state root doesn't exist
    InvalidStateRoot,
    /// Emits when the given storage proof is invalid.
    InvalidProof,
    /// Value doesn't exist in the Db for the given key.
    MissingValue,
    /// Failed to decode value.
    FailedToDecode,
}

pub(crate) struct StorageProofVerifier<H: Hasher>(PhantomData<H>);

impl<H: Hasher> StorageProofVerifier<H> {
    pub(crate) fn verify_and_get_value<V: Decode>(
        proof: Proof<H::Out>,
        key: StorageKey,
    ) -> Result<V, VerificationError> {
        let db = proof.message_proof.into_memory_db::<H>();
        let val = read_trie_value::<LayoutV1<H>, _>(&db, &proof.state_root, key.as_ref())
            .map_err(|_| VerificationError::InvalidProof)?
            .ok_or(VerificationError::MissingValue)?;

        let decoded = V::decode(&mut &val[..]).map_err(|_| VerificationError::FailedToDecode)?;

        Ok(decoded)
    }
}

/// Proof combines the storage proofs to validate messages.
#[derive(Debug, Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
pub struct Proof<StateRoot> {
    pub state_root: StateRoot,
    /// Storage proof that src_domain state_root is registered on System domain
    // TODO(ved): add system domain proof when store is available
    /// Storage proof that message is processed on src_domain.
    pub message_proof: StorageProof,
}
