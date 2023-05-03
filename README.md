# Assess criticality of deploying contract with mismatching Environment 

This repo demonstrates the behaviour upon the contract instantiation and execution 
when `Environment` types mismatch the ones defined in the `pallet-contract`

[`substrate-contracts-node` v0.24.0](https://github.com/paritytech/substrate-contracts-node/releases/tag/v0.24.0)
was used for testing.


# Findings
 I am going to structure this mini-report with the effects of mismatching each individual type of `Environment` trait.

## `MAX_EVENT_TOPICS`
Mistamatching (below or great) this value has no effect on the instantiation of the contract if it doesn't emit any events in the constructor. What matters is the actual number of topics being emitted in the event. If it exceeds the limit, `TooManyTopics` dispatch error is thrown. 

## `AccoundId`
This one was harder to reproduce because it requires implementing or deriving requires traits bounds imposed by `Environment`.  For the sake of the experiment, I introduced my own definition `pub struct MyAccountId([u8; 16]);` which has a shorter byte slice than the original type. It was also easy to auto-derive `StorageLayout` with it.
If the `AccountId` is not used for any operations outside the contract (like a balance transfer) then it is fine. I managed to store it inside the contract storage inside `Mapping` which makes sense since it is up to the contract developer to decide who they identify accounts. Otherwise `Self::env().transfer(...)` will fail with the incompatible `AccountId` type with `TransferFailed` message. 
A similar behaviour is for `MyAccountId(Vec<u8>)` (with `A contract being executed must have a valid account id` message) so I assume having the internal slice larger than the original will also be incompatible.

## `Balance`
Simple, having a data type of smaller size than the original one fails with `OutputBufferTooSmall` error upon instantiation.

 ## `Hash`
Similarly to `AccountId`, I have introduced my own type `pub struct MyHash([u8; 64])`, the contract gets instantiated but gets immediately bricked with `DecodingFailed` error. 

## `Timestamp`
I can only specify types that are larger then `u32` due to `AtLeast32BitUnsigned` trait bound. It has no effect on the contract execution.

## `BlockNumber`
Same as `Timestamp`

## `ChainExtension`
Obviously fails if you try to call a function that is not exposed.

## Other notes
If you introduce copies of default types exposed by `ink_env` for `AccountId` and `Hash` then, as expected, they are compatible as long as the core functionality is the same.