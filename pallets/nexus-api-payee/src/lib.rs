#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
	traits::Vec,
};
use frame_system::ensure_signed;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct Payee {
	destination_country_id: Vec<u64>,
	destination_bank_identifier: Vec<u64>,
	destination_bank_account_number: Vec<u64>,
}

decl_storage! {
	trait Store for Module<T: Config> as SimpleMap {
		SimpleMap get(fn simple_map): map hasher(blake2_128_concat) T::AccountId => u32;
		ConfirmPayee get(fn confirm_payee): map hasher(blake2_128_concat) T::AccountId => Payee;
		SubProcess get(fn sub_process): map hasher(blake2_128_concat) T::AccountId => bool;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// A user has set their entry
		EntrySet(AccountId, u32),

		/// A user has read their entry, leaving it in storage
		EntryGot(AccountId, u32),

		/// A user has read their entry, removing it from storage
		EntryTaken(AccountId, u32),

		/// A user has read their entry, incremented it, and written the new entry to storage
		/// Parameters are (user, old_entry, new_entry)
		EntryIncreased(AccountId, u32, u32),

		/// SubProcess is done on behalf of the bank
		SubProcessDone(AccountId, bool),

		/// Confirmation Is Done
		PaymentConfirm(AccountId, Vec<u64>, Vec<u64>, Vec<u64>),
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

		/// Confirm the subprocess on behalf of the bank
		#[weight = 10_000]
		fn confirm_subprocess(origin, confirm: bool) -> DispatchResult {
			let user = ensure_signed(origin)?;
			<SubProcess<T>>::insert(&user, confirm);
			Self::deposit_event(RawEvent::SubProcessDone(user, confirm));
			Ok(())
		}

		/// Confirm the payment
		#[weight = 10_000]
		fn confirmation_of_payee(origin, destination_country_id: Vec<u64>, destination_bank_identifier: Vec<u64>, destination_bank_account_number: Vec<u64>) -> DispatchResult  {
			let user = ensure_signed(origin)?;

			// Check if the dest isp/bank has confirmed the payment
			let confirmation = <SubProcess<T>>::get(&user);

			ensure!(confirmation == true, "");

			let destination_country_id_clone = destination_country_id.clone();
			let destination_bank_identifier_clone = destination_bank_identifier.clone();
			let destination_bank_account_number_clone = destination_bank_account_number.clone();


			let payee = Payee {
				destination_country_id,
				destination_bank_identifier,
				destination_bank_account_number,
			};
			<ConfirmPayee<T>>::insert(&user, payee);
			Self::deposit_event(RawEvent::PaymentConfirm(user, destination_country_id_clone, destination_bank_identifier_clone, destination_bank_account_number_clone));
			Ok(())
		}
	}
}
