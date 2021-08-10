#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;

use sp_std::prelude::*;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct Quote {
	source_lp: Vec<u16>,
	destination_lp: Vec<u16>,
	rate: Vec<u16>,
	public: bool,
	timestamp: Vec<u16>,
	source_bank_id: Vec<u16>,
	quote_uuid: Vec<u16>,
	fxp_uuid: Vec<u16>,
}

decl_storage! {
	trait Store for Module<T: Config> as NexusApiQuote {
		ProvideRates get(fn update_api): map hasher(blake2_128_concat) (Vec<u8>, Vec<u8>, Vec<u16> ,T::AccountId) => Quote;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// FXP has submitted the quote for the given currencies.
		RatesProvided(Vec<u8>, Vec<u8>, Vec<u16>, Vec<u16>),

		/// Source Bank is retriving the quote for the given currencies.
		RatesRequested(Vec<u8>, Vec<u8>, Vec<u16>, Vec<u16>, Vec<u16>),

		/// FXP has deleted the quote for the given currencies.
		RatesDeleted(Vec<u8>, Vec<u8>, AccountId, Vec<u16>),
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
		fn provide_rate(origin, source_currency:Vec<u8>, destination_currency: Vec<u8>, quote_uuid: Vec<u16> , fxp_uuid: Vec<u16>, source_lp: Vec<u16>, destination_lp: Vec<u16>, rate: Vec<u16>, public: bool, timestamp: Vec<u16>, source_bank_id: Vec<u16>) -> DispatchResult {
				let user = ensure_signed(origin)?;
				let rate_clone = rate.clone();
				let quote_uuid_clone = quote_uuid.clone();
				let quote = Quote {
					source_lp,
					destination_lp,
					rate,
					public,
					timestamp,
					source_bank_id,
					quote_uuid,
					fxp_uuid,
			};

			<ProvideRates<T>>::insert((&source_currency, &destination_currency,&quote_uuid_clone ,&user), quote);
			Self::deposit_event(RawEvent::RatesProvided(source_currency, destination_currency, quote_uuid_clone, rate_clone));
			Ok(())
		}

		#[weight= 10_000_000]
		fn get_rate(origin, source_currency: Vec<u8>, destination_currency: Vec<u8>, quote_uuid:Vec<u16>) -> DispatchResult {
				let user = ensure_signed(origin)?;

				let (source_currency_clone, destination_currency_clone) = (source_currency.clone(), destination_currency.clone());

				let origin_account = (source_currency, destination_currency, quote_uuid, user.clone());

				ensure!(<ProvideRates<T>>::contains_key(&origin_account), "");
				let quote = <ProvideRates<T>>::get(&origin_account);

				Self::deposit_event(RawEvent::RatesRequested(source_currency_clone,destination_currency_clone, quote.quote_uuid, quote.fxp_uuid, quote.rate));
				Ok(())
			}

		#[weight= 10_000_000]
		fn delete_rate(origin, source_currency: Vec<u8>, destination_currency: Vec<u8>, quote_uuid: Vec<u16>) -> DispatchResult {
				let user = ensure_signed(origin)?;

				let quote_uuid_clone = quote_uuid.clone();

				let (source_currency_clone, destination_currency_clone) = (source_currency.clone(), destination_currency.clone());

				let origin_account = (source_currency, destination_currency, quote_uuid,user.clone());


				ensure!(<ProvideRates<T>>::contains_key(&origin_account), "");
				<ProvideRates<T>>::take(&origin_account);
			Self::deposit_event(RawEvent::RatesDeleted(source_currency_clone, destination_currency_clone, user, quote_uuid_clone));
			Ok(())
	}
}
}
