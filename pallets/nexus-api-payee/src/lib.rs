#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;

use sp_std::prelude::*;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct Payee {
	destination_country_id: Vec<u8>,
	destination_bank_identifier: Vec<u8>,
	destination_bank_account_number: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct DestinationPayee {
	destination_bank_acc_holder_name: Vec<u8>,
	destination_bank_acc_display_name: Vec<u8>,
}

decl_storage! {
	trait Store for Module<T: Config> as NexusApiPayee {
		ConfirmPayee get(fn confirm_payee): map hasher(blake2_128_concat) T::AccountId => Payee;
		SubProcess get(fn sub_process): map hasher(blake2_128_concat) T::AccountId => DestinationPayee;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// SubProcess is done on behalf of the bank
		SubProcessDone(AccountId, Vec<u8>, Vec<u8>),

		/// Confirmation Is Done
		PaymentConfirm(AccountId, Vec<u8>, Vec<u8>),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Invalid Payee
		Invalid
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {

		// Initialize errors
		type Error = Error<T>;

		// Initialize events
		fn deposit_event() = default;

		/// The request will be sent by the frontend
		#[weight = 10_000]
		fn confirm_subprocess(origin, 	destination_bank_acc_holder_name: Vec<u8>,
			destination_bank_acc_display_name: Vec<u8>,) -> DispatchResult {
			let user = ensure_signed(origin)?;

			let destination_payee = DestinationPayee {
				destination_bank_acc_holder_name,
				destination_bank_acc_display_name,
			};

			let destination_payee_clone = destination_payee.clone();

			<SubProcess<T>>::insert(&user, &destination_payee);
			Self::deposit_event(RawEvent::SubProcessDone(user, destination_payee_clone.destination_bank_acc_holder_name, destination_payee_clone.destination_bank_acc_display_name));
			Ok(())
		}

		/// Confirm the payment
		#[weight = 10_000]
		fn confirmation_of_payee(origin, destination_country_id: Vec<u8>, destination_bank_identifier: Vec<u8>, destination_bank_account_number: Vec<u8>) -> DispatchResult  {
			let user = ensure_signed(origin)?;

			// Check if the dest isp/bank has confirmed the payment
			let confirmation = <SubProcess<T>>::get(&user);

			let payee = Payee {
				destination_country_id,
				destination_bank_identifier,
				destination_bank_account_number,
			};
			<ConfirmPayee<T>>::insert(&user, payee);
			Self::deposit_event(RawEvent::PaymentConfirm(user, confirmation.destination_bank_acc_holder_name, confirmation.destination_bank_acc_display_name));
			Ok(())
		}
	}
}
