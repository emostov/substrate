//! Offchain Indexing specific validation.
//!
//! Validates the configuration parameters from the cli
//! are compatible with previous execution flags
//! present in the key value store.

use crate::utils::{meta_keys, COLUMN_META, db_err};
use codec::{Decode, Encode};
use kvdb::{KeyValueDB, DBTransaction};

pub use sc_client::{OffchainIndexingState, OffchainIndexingConfig};

/// Validate status against in DB stored flag value for indexing and update the state in the meta DB.
///
/// Indexing shall not change in between executions.
/// Returns the rsolved `OffchainIndexingState` which can be
/// of variant `Enable` or `Disable`.
pub fn check_offchain_indexing(
    db: &dyn KeyValueDB,
    config: &OffchainIndexingConfig,
) -> Result<OffchainIndexingState, sp_blockchain::Error> {
    // currently this only applies to offchain config
	let db_value = read_db_offchain_indexing(db)?;

	let val = if let Some(db_value) = db_value {
		match (db_value, config.state) {
			(true, OffchainIndexingState::Disable) =>
				Err(sp_blockchain::Error::Backend(
					"The DB requires indexing to be enabled. Start a separate DB or use \
					 ForceDisable, but be aware that re-enabling will require re-sync".to_string())
				)?,
			(false, OffchainIndexingState::Enable) =>
				Err(sp_blockchain::Error::Backend("Re-sync required due to config change of offchain indexing".to_string()))?,
			// no change
			(true, OffchainIndexingState::Enable) => true,
			(true, OffchainIndexingState::ForceEnable) => true,
			// no change
			(false, OffchainIndexingState::Disable) => false,
			(false, OffchainIndexingState::ForceDisable) => false,
			// force overwrite
			(true, OffchainIndexingState::ForceDisable) => {
				// TODO warn on next start
				false
			},
			(false, OffchainIndexingState::ForceEnable) => {
				// TODO warn on next start
				true
			},
			(previous, OffchainIndexingState::Default) => {
				previous
			},
		}
	} else {
		// database does not contain this value
		// which implies this is the first start
		match config.state {
			OffchainIndexingState::Default => config.is_validator,
			OffchainIndexingState::Enable | OffchainIndexingState::ForceEnable => true,
			OffchainIndexingState::Disable | OffchainIndexingState::ForceDisable => false,
		}
	};

	write_db_offchain_indexing(db, val)?;
	let val = if val {
		OffchainIndexingState::Enable
	} else {
		OffchainIndexingState::Disable
	};
	Ok(val)
}

/// Update the offchain indexing DB.
fn write_db_offchain_indexing(
	db: &dyn KeyValueDB,
	enabled: bool,
) -> sp_blockchain::Result<()> {
	let mut transaction = DBTransaction::new();
	let val: Vec<u8> = enabled.encode();
	transaction.put(COLUMN_META, meta_keys::OFFCHAIN_INDEXING, val.as_slice());
	db.write(transaction).map_err(db_err)?;
	Ok(())
}

/// Update the offchain indexing DB.
fn read_db_offchain_indexing(
	db: &dyn KeyValueDB,
) -> sp_blockchain::Result<Option<bool>> {
	let previous = db.get(COLUMN_META, meta_keys::OFFCHAIN_INDEXING)
		.map_err(db_err)?;
	if let Some(raw) = previous {
		Ok(
			Some(
				<bool as Decode>::decode(&mut raw.as_slice())
					.map_err(|_e| sp_blockchain::Error::Backend(
							format!("Failed to decode offchain indexing state value")
						)
					)?
			)
		)
	} else {
		Ok(None)
	}
}