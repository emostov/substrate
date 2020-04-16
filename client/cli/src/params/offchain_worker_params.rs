// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.


//! Offchain worker related configuration parameters.
//!
//! A subset of configuration parameters which are relevant to
//! the inner working of offchain workers. The usage is solely
//! targeted at handling input parameter parsing providing
//! a reasonable abstraction.

use structopt::StructOpt;
use sc_service::config::OffchainWorkerConfig;
use sc_network::config::Role;

use crate::error;
use crate::{OffchainWorkerEnabled,OffchainIndexingEnabled};
use sc_client_db::offchain_indexing::{OffchainIndexingState, OffchainIndexingConfig};

/// Offchain worker related parameters.
#[derive(Debug, StructOpt, Clone)]
pub struct OffchainWorkerParams {

	/// Should execute offchain workers on every block.
	///
	/// By default it's only enabled for nodes that are authoring new blocks.
	#[structopt(
		long = "offchain-worker",
		value_name = "ENABLED",
		possible_values = &OffchainWorkerEnabled::variants(),
		case_insensitive = true,
		default_value = "WhenValidating"
    )]
    pub enabled: OffchainWorkerEnabled,

	/// Allow access to offchain workers indexing API
	///
	/// Enables runtime to write directly to the offchain worker's
	/// DB during block import.
    #[structopt(
        long = "enable-offchain-indexing",
		value_name = "ENABLE_OFFCHAIN_INDEXING",
		possible_values = &OffchainIndexingEnabled::variants(),
		case_insensitive = true,
		default_value = "Default"
    )]
	pub indexing_enabled: OffchainIndexingEnabled,
}

impl OffchainWorkerParams {
	/// Load spec to `Configuration` from `OffchainWorkerParams` and spec factory.
	pub fn offchain_worker(
		&self,
        role: &Role,
	) -> error::Result<OffchainWorkerConfig>
	{
        let enabled = match (&self.enabled, role) {
			(OffchainWorkerEnabled::WhenValidating, Role::Authority { .. }) => true,
			(OffchainWorkerEnabled::Always, _) => true,
			(OffchainWorkerEnabled::Never, _) => false,
			(OffchainWorkerEnabled::WhenValidating, _) => false,
		};


		let state = match self.indexing_enabled {
			OffchainIndexingEnabled::Default => OffchainIndexingState::Default,
			OffchainIndexingEnabled::Disabled => OffchainIndexingState::Disable,
			OffchainIndexingEnabled::Enabled => OffchainIndexingState::Enable,
			OffchainIndexingEnabled::ForceDisabled => OffchainIndexingState::ForceDisable,
			OffchainIndexingEnabled::ForceEnabled => OffchainIndexingState::ForceEnable,
		};

		let is_validator = match role {
			Role::Authority { .. } => true,
			Role::Full => false,
			Role::Light => false,
			Role::Sentry { .. } => false,
		};

		let indexing = OffchainIndexingConfig {
			is_validator,
			state,
		};
        Ok(OffchainWorkerConfig { enabled, indexing })
	}
}