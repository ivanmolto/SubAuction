#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused)]
// Used for encoding/decoding into scale
use frame_support::{traits::{LockableCurrency, LockIdentifier, Currency, WithdrawReason, WithdrawReasons},
					Parameter, decl_error, decl_event, decl_module, decl_storage, ensure, dispatch::{DispatchResult, DispatchError}};
use frame_system::ensure_signed;
use sp_runtime::{
	traits::{AtLeast32Bit, AtLeast32BitUnsigned, Bounded, MaybeSerializeDeserialize, Member, One, MaybeDisplay},
	RuntimeDebug
};
use sp_std::{fmt::Debug, result, vec::Vec};
pub use traits::*;
pub mod traits;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/*
Parameter - can be used in Dispatchable function (so in the decl_module!)
Member - can be used in the runtime structures
Default - gives trait variables default values (bool = false, int = 0, etc)
Copy - value can be duplicated by simply copying the bits
MaybeSerializeDeserialize - type that implements Serialize, DeserializeOwned and Debug when in std environment.
Bounded - numbers which have upper and lower bounds (so, basically all primitive types???)
 */

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The balance type for bidding
	type Balance: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize;

	/// The auction ID type
	type AuctionId: Parameter + Member + AtLeast32BitUnsigned + Default + Copy + MaybeSerializeDeserialize + Bounded;

	// Currency
	type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

	// Account Id
	type AccountId: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + Default;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait AuctionStore for Module<T: Trait> as AuctionModule {
		/// Stores on-going and future auctions. Closed auction are removed.
		pub Auctions get(fn auctions): double_map hasher(twox_64_concat) T::AuctionId, hasher(twox_64_concat) AuctionType => Option<AuctionInfo<T::AccountId, T::Balance, T::BlockNumber>>;

		/// Track the next auction ID.
		pub AuctionsIndex get(fn auctions_index): T::AuctionId;

		/// Index auctions by end time.
		pub AuctionEndTime get(fn auction_end_time): double_map hasher(twox_64_concat) T::BlockNumber, hasher(twox_64_concat) T::AuctionId => Option<()>;
	}
}

decl_event!(
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
		<T as Trait>::Balance,
		<T as Trait>::AuctionId,
	{
		// Auction created
		AuctionCreated(AccountId, AuctionId),
		/// A bid is placed
		Bid(AuctionId, AccountId, Balance),
		//
		AuctionConcluded(AuctionId),
	}
);

decl_error! {
	/// Error for auction module.
	pub enum Error for Module<T: Trait> {
		AuctionNotExist,
		AuctionNotStarted,
		BidNotAccepted,
		InvalidBidPrice,
		NoAvailableAuctionId,
		AuctionStartAlreadyPassed,
		NonExistingAuctionType,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight=0]
		fn create_auction(origin, auction_info: PublicAuctionInfo<T::AccountId, T::Balance, T::BlockNumber>) {
			let sender = ensure_signed(origin)?;

			 match auction_info.auction_type {
				AuctionType::English => {
					let english_auction = AuctionInfo::<T::AccountId, T::Balance, T::BlockNumber> {};
					english_auction.new_auction(auction_info)?;
					Ok(())
				}
				_ => Error::<T>::NonExistingAuctionType,
			}
		}
	}
}

impl<T: Trait> Auction<T::AccountId, T::Balance, T::BlockNumber> for AuctionInfo<T::AccountId, T::Balance, T::BlockNumber> {
	type AuctionId = T::AuctionId;
	type Balance = T::Balance;
	type AccountId = T::AccountId;

	fn new_auction(&self, info: AuctionInfo<Self::AccountId, Self::Balance, T::BlockNumber>) -> result::Result<Self::AuctionId, DispatchError> {
		let current_block_number = frame_system::Module::<T>::block_number();
		ensure!(info.start <= current_block_number, Error::<T>::AuctionStartAlreadyPassed);
		let auction_id = <AuctionsIndex<T>>::try_mutate(|x| -> result::Result<Self::AuctionId, DispatchError> {
			let id = *x;
			ensure!(id != T::AuctionId::max_value(), Error::<T>::NoAvailableAuctionId);
			*x += One::one();
			Ok(id)
		})?;
		<Auctions<T>>::insert(auction_id, info.auction_type, info);
		Ok(auction_id)
	}

	fn auction_info(id: Self::AuctionId) -> Option<AuctionInfo<T::AccountId, Self::Balance, T::BlockNumber>> {
		unimplemented!()
	}

	fn update_auction(id: Self::AuctionId, info: AuctionInfo<T::AccountId, Self::Balance, T::BlockNumber>) -> DispatchResult {
		unimplemented!()
	}

	fn remove_auction(id: Self::AuctionId) {
		unimplemented!()
	}
}

// impl<T: Trait> CommonAuction<T::AccountId, T::BlockNumber> {
// 	type AuctionId = T::AuctionId;
// 	type Balance = T::Balance;
// 	type AccountId = T::AccountId;
//
// 	fn new_auction(&self, info: AuctionInfo<Self::AccountId, Self::Balance, T::BlockNumber>) -> result::Result<Self::AuctionId, DispatchError> {
// 		let current_block_number = frame_system::Module::<T>::block_number();
// 		ensure!(info.start <= current_block_number, Error::<T>::AuctionStartAlreadyPassed);
// 		let auction_id = <AuctionsIndex<T>>::try_mutate(|x| -> result::Result<Self::AuctionId, DispatchError> {
// 			let id = *x;
// 			ensure!(id != T::AuctionId::max_value(), Error::<T>::NoAvailableAuctionId);
// 			*x += One::one();
// 			Ok(id)
// 		})?;
// 		<Auctions<T>>::insert(auction_id, info.auction_type, info);
// 		Ok(auction_id)
// 	}
//
// 	fn auction_info(id: Self::AuctionId) -> Option<AuctionInfo<T::AccountId, Self::Balance, T::BlockNumber>> {
// 		unimplemented!()
// 	}
//
// 	fn update_auction(id: Self::AuctionId, info: AuctionInfo<T::AccountId, Self::Balance, T::BlockNumber>) -> DispatchResult {
// 		unimplemented!()
// 	}
//
// 	fn remove_auction(id: Self::AuctionId) {
// 		unimplemented!()
// 	}
// }

// impl<T: Trait> Auction<T::AccountId, T::BlockNumber> for EnglishAuction<T> {
// 	type AuctionId = T::AuctionId;
// 	type Balance = T::Balance;
// 	type AccountId = T::AccountId;
//
// 	fn new_auction(&self, info: AuctionInfo<Self::AccountId, Self::Balance, T::BlockNumber>) -> result::Result<Self::AuctionId, DispatchError> {
// 		self.default_auction.new_auction(info)
// 	}
//
// 	fn auction_info(id: Self::AuctionId) -> Option<AuctionInfo<T::AccountId, Self::Balance, T::BlockNumber>> {
// 		unimplemented!()
// 	}
//
// 	fn update_auction(id: Self::AuctionId, info: AuctionInfo<T::AccountId, Self::Balance, T::BlockNumber>) -> DispatchResult {
// 		unimplemented!()
// 	}
//
// 	fn remove_auction(id: Self::AuctionId) {
// 		unimplemented!()
// 	}
// }
