
/// Relevant client configuration items relevant for the client.
#[derive(Debug,Clone,Default)]
pub struct ClientExtraConfig {
	/// Enable the offchain worker db.
	pub offchain_worker_enabled: bool,
	/// If true, allows access from the runtime to write into offchain worker db.
	pub offchain_indexing_api: OffchainIndexingConfig,
}

/// Offchain indexing config.
#[derive(Debug,Clone,Default)]
pub struct OffchainIndexingConfig {
	/// Defines the desired state of the offchain API to be in.
	pub state: OffchainIndexingState,
	/// `true` if the current node is a validator.
	pub is_validator: bool,
}

impl OffchainIndexingConfig {
	/// Verify after calling `check_offchain_indexing` if the feature is enabled.
	pub fn is_enabled(&self) -> bool {
		// after configuration there is only `Enable` or `Disable`
		assert_ne!(OffchainIndexingState::ForceEnable, self.state);
		assert_ne!(OffchainIndexingState::ForceDisable, self.state);
		assert_ne!(OffchainIndexingState::Default, self.state);
		self.state == OffchainIndexingState::Enable
	}
}
/// State
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
pub enum OffchainIndexingState {
	/// Follow the default behaviour impl.
	Default,
	/// Attempt to enable, which only works initially.
	Enable,
	/// Attempt to disable, which only works initially.
	Disable,
	/// Ignore the consequences of incomplete state in the offchain worker database
	/// and force enabling of the indexing API.
	ForceEnable,
	/// Ignore the consequences of possibly incomplete state in the offchain worker database
	/// and force disabling of the indexing API.
	ForceDisable,
}

impl Default for OffchainIndexingState {
	fn default() -> Self {
		Self::Default
	}
}