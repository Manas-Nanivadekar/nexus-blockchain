#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct Quote {
	source_lp: u64,
	destination_lp: u64,
	rate: u64,
	public: bool,
	timestamp: u64,
	source_bank_id: u64,
}

decl_storage! {
	trait Store for Module<T: Config> as NexusApiQuote {
		ProvideRates get(fn update_api): map hasher(blake2_128_concat) (u32, T::AccountId) => Quote;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// FXP has submitted the quote for the given currencies.
		RatesProvided(u32, AccountId, u64),

		/// Source Bank is retriving the quote for the given currencies.
		RatesRequested(u32, AccountId, u64),

		/// FXP has deleted the quote for the given currencies.
		RatesDeleted(u32, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// The requested user has not stored a value yet
		NoValueStored,

		/// The value cannot be incremented further because it has reached the maximum allowed value
		MaxValueReached,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		#[weight= 10_000_000]
		fn provide_rate(origin,uuid: u32 ,source_lp: u64, destination_lp: u64, rate: u64, public: bool, timestamp: u64, source_bank_id: u64) -> DispatchResult {
				let user = ensure_signed(origin)?;
				let rate_clone = rate.clone();
				let quote = Quote {
					source_lp,
					destination_lp,
					rate,
					public,
					timestamp,
					source_bank_id,
			};

			<ProvideRates<T>>::insert((uuid, &user), quote);
			Self::deposit_event(RawEvent::RatesProvided(uuid, user, rate_clone));
			Ok(())
		}

		#[weight= 10_000_000]
		fn get_rate(origin, uuid:u32) -> DispatchResult {
				let user = ensure_signed(origin)?;
				let origin_account = (uuid, user.clone());
				ensure!(<ProvideRates<T>>::contains_key(&origin_account), "");
				let quote = <ProvideRates<T>>::get(&origin_account);

				Self::deposit_event(RawEvent::RatesRequested(uuid, user, quote.rate));
				Ok(())
			}

		#[weight= 10_000_000]
		fn delete_rate(origin, uuid: u32) -> DispatchResult {
				let user = ensure_signed(origin)?;
				let origin_account = (uuid, user.clone());
				ensure!(<ProvideRates<T>>::contains_key(&origin_account), "");
				<ProvideRates<T>>::take(&origin_account);
			Self::deposit_event(RawEvent::RatesDeleted(uuid, user));
			Ok(())
	}
}
}
