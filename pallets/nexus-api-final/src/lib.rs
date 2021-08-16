#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use frame_system::ensure_signed;

use codec::{Decode, Encode};

use sp_std::prelude::*;

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct FinalPaymentStruct {
	message_id: Vec<u8>,
	creation_time: Vec<u8>,
	settlement_amount: Vec<u8>,
	payment_uuid: Vec<u8>,
	clearing_system_ref: Vec<u8>,
	charge_bearer: Vec<u8>,
	quote_uuid: Vec<u8>,
	lp_source: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct DestinationBankStruct {
	dest_bank_id: Vec<u8>,
	dest_bank_acc_number: Vec<u8>,
	dest_bank_acc_name: Vec<u8>,
	dest_bank_acc_add: Vec<u8>,
	dest_bank_acc_dob: Vec<u8>,
	dest_bank_acc_dop: Vec<u8>,
	dest_bank_acc_national_id: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct SourceBankStruct {
	source_bank_id: Vec<u8>,
	source_bank_acc_number: Vec<u8>,
	source_bank_acc_name: Vec<u8>,
	source_bank_acc_add: Vec<u8>,
	source_bank_acc_dob: Vec<u8>,
	source_bank_acc_dop: Vec<u8>,
	source_bank_acc_national_id: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, Debug, Eq, PartialEq)]
pub struct DestBankStauts {
	status: Vec<u8>,
	reason_for_status: Vec<u8>,
}

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
	trait Store for Module<T: Config> as NexusApiFinal {
		FinalPayment get(fn final_payment): map hasher(blake2_128_concat) (T::AccountId , Vec<u8>)=> FinalPaymentStruct;
		DestBank get(fn dest_bank): map hasher(blake2_128_concat) (T::AccountId , Vec<u8>)=> DestinationBankStruct;
		SourceBank get(fn source_bank): map hasher(blake2_128_concat) (T::AccountId , Vec<u8>)=> SourceBankStruct;
		Status get(fn status): map hasher(blake2_128_concat) T::AccountId=> DestBankStauts;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
	{
		/// Status has Set
		StatusSet(AccountId, Vec<u8>, Vec<u8>),

		/// Dest Bank has been given
		DestBankSet(
			AccountId,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
		),

		/// Source Bank data has been given
		SourceBankSet(
			AccountId,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
		),

		/// Final Payment has been given
		FinalPaymentSet(
			AccountId,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
		),

		FinalData(
			AccountId,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
		),
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

		#[weight = 10_000]
		fn set_status(origin, current_status: Vec<u8>, reason_for_status: Vec<u8>) -> DispatchResult{
			let user = ensure_signed(origin)?;

			let current_status_clone = current_status.clone();

			let reason_for_status_clone = reason_for_status.clone();

			let status = DestBankStauts {
				status: current_status,
				reason_for_status: reason_for_status,
			};

			<Status<T>>::insert(&user, status);

			Self::deposit_event(RawEvent::StatusSet(user, current_status_clone, reason_for_status_clone));

			Ok(())
		}

		#[weight = 10_000_000]
		fn set_dest_bank_data(origin,payment_id: Vec<u8> ,dest_bank_id: Vec<u8>, dest_bank_acc_number: Vec<u8>, dest_bank_acc_name: Vec<u8>, dest_bank_acc_add: Vec<u8>, dest_bank_acc_dob: Vec<u8>, dest_bank_acc_dop: Vec<u8>, dest_bank_acc_national_id: Vec<u8>) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let dest_bank_id_clone = dest_bank_id.clone();
			let dest_bank_acc_number_clone = dest_bank_acc_number.clone();
			let dest_bank_acc_name_clone = dest_bank_acc_name.clone();
			let dest_bank_acc_add_clone = dest_bank_acc_add.clone();
			let dest_bank_acc_dob_clone = dest_bank_acc_dob.clone();
			let dest_bank_acc_dop_clone = dest_bank_acc_dop.clone();
			let dest_bank_acc_national_id_clone = dest_bank_acc_national_id.clone();
			let dest_bank = DestinationBankStruct {
				dest_bank_id: dest_bank_id,
				dest_bank_acc_number: dest_bank_acc_number,
				dest_bank_acc_name: dest_bank_acc_name,
				dest_bank_acc_add: dest_bank_acc_add,
				dest_bank_acc_dob: dest_bank_acc_dob,
				dest_bank_acc_dop: dest_bank_acc_dop,
				dest_bank_acc_national_id: dest_bank_acc_national_id,
			};
			<DestBank<T>>::insert((&user, payment_id), dest_bank);

			Self::deposit_event(RawEvent::DestBankSet(user, dest_bank_id_clone, dest_bank_acc_number_clone, dest_bank_acc_name_clone, dest_bank_acc_add_clone, dest_bank_acc_dob_clone, dest_bank_acc_dop_clone, dest_bank_acc_national_id_clone));

			Ok(())
		}

		#[weight = 10_000_000]
		fn set_source_bank_data(origin,payment_id: Vec<u8>  ,source_bank_id: Vec<u8>, source_bank_acc_number: Vec<u8>, source_bank_acc_name: Vec<u8>, source_bank_acc_add: Vec<u8>, source_bank_acc_dob: Vec<u8>, source_bank_acc_dop: Vec<u8>, source_bank_acc_national_id: Vec<u8>) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let source_bank_id_clone = source_bank_id.clone();
			let source_bank_acc_number_clone = source_bank_acc_number.clone();
			let source_bank_acc_name_clone = source_bank_acc_name.clone();
			let source_bank_acc_add_clone = source_bank_acc_add.clone();
			let source_bank_acc_dob_clone = source_bank_acc_dob.clone();
			let source_bank_acc_dop_clone = source_bank_acc_dop.clone();
			let source_bank_acc_national_id_clone = source_bank_acc_national_id.clone();
			let source_bank = SourceBankStruct {
				source_bank_id,
				 source_bank_acc_number,
				 source_bank_acc_name,
				 source_bank_acc_add,
				 source_bank_acc_dob,
				 source_bank_acc_dop,
				 source_bank_acc_national_id,
			};
			<SourceBank<T>>::insert((&user, payment_id), source_bank);

			Self::deposit_event(RawEvent::SourceBankSet(user, source_bank_id_clone, source_bank_acc_number_clone, source_bank_acc_name_clone, source_bank_acc_add_clone, source_bank_acc_dob_clone, source_bank_acc_dop_clone, source_bank_acc_national_id_clone));

			Ok(())
		}

		#[weight = 10_000_000]
		fn final_payment_func(origin, message_id: Vec<u8>, creation_time: Vec<u8>, settlement_amount: Vec<u8>, payment_uuid: Vec<u8>, clearing_system_ref: Vec<u8>, charge_bearer: Vec<u8>, quote_uuid: Vec<u8>, lp_source: Vec<u8>) -> DispatchResult {
			let user = ensure_signed(origin)?;

			let message_id_clone = message_id.clone();
			let creation_time_clone = creation_time.clone();
			let settlement_amount_clone = settlement_amount.clone();
			let payment_uuid_clone = payment_uuid.clone();
			let clearing_system_ref_clone = clearing_system_ref.clone();
			let charge_bearer_clone = charge_bearer.clone();
			let quote_uuid_clone = quote_uuid.clone();
			let lp_source_clone = lp_source.clone();

			let final_payment = FinalPaymentStruct {
				message_id,
				creation_time,
				settlement_amount,
				payment_uuid,
				clearing_system_ref,
				charge_bearer,
				quote_uuid,
				lp_source,
			};

			<FinalPayment<T>>::insert((&user, &payment_uuid_clone), final_payment);

			Self::deposit_event(RawEvent::FinalPaymentSet(user, message_id_clone, creation_time_clone, settlement_amount_clone, payment_uuid_clone, clearing_system_ref_clone, charge_bearer_clone, quote_uuid_clone, lp_source_clone));

			Ok(())

		}

		#[weight = 10_000_000]
		fn get_final_payment(origin, payment_id: Vec<u8>) -> DispatchResult {
			let user = ensure_signed(origin)?;
			let keys = (&user, payment_id);
			let keys_clone = keys.clone();
			let sec_clone = keys.clone();

			let dest_data = <DestBank<T>>::get(keys_clone);
			let source_data = <SourceBank<T>>::get(keys);
			let final_data = <FinalPayment<T>>::get(sec_clone);

			Self::deposit_event(RawEvent::FinalData(user, dest_data.dest_bank_id, dest_data.dest_bank_acc_number, dest_data.dest_bank_acc_name, dest_data.dest_bank_acc_add, dest_data.dest_bank_acc_dob, dest_data.dest_bank_acc_dop, dest_data.dest_bank_acc_national_id, source_data.source_bank_id, source_data.source_bank_acc_number, source_data.source_bank_acc_name, source_data.source_bank_acc_add, source_data.source_bank_acc_dob, source_data.source_bank_acc_dop, source_data.source_bank_acc_national_id, final_data.message_id, final_data.creation_time, final_data.settlement_amount, final_data.payment_uuid, final_data.clearing_system_ref, final_data.charge_bearer, final_data.quote_uuid, final_data.lp_source));
			Ok(())
		}
	}
}
