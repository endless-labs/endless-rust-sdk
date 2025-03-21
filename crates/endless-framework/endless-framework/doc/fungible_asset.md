
<a id="0x1_fungible_asset"></a>

# Module `0x1::fungible_asset`

This defines the fungible asset module that can issue fungible asset of any <code><a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a></code> object. The
metadata object can be any object that equipped with <code><a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a></code> resource.


-  [Resource `Supply`](#0x1_fungible_asset_Supply)
-  [Resource `ConcurrentSupply`](#0x1_fungible_asset_ConcurrentSupply)
-  [Resource `Metadata`](#0x1_fungible_asset_Metadata)
-  [Resource `FungibleStore`](#0x1_fungible_asset_FungibleStore)
-  [Struct `FungibleAsset`](#0x1_fungible_asset_FungibleAsset)
-  [Struct `AggregatableCoin`](#0x1_fungible_asset_AggregatableCoin)
-  [Struct `MintRef`](#0x1_fungible_asset_MintRef)
-  [Struct `TransferRef`](#0x1_fungible_asset_TransferRef)
-  [Struct `BurnRef`](#0x1_fungible_asset_BurnRef)
-  [Struct `Deposit`](#0x1_fungible_asset_Deposit)
-  [Struct `Withdraw`](#0x1_fungible_asset_Withdraw)
-  [Struct `Frozen`](#0x1_fungible_asset_Frozen)
-  [Constants](#@Constants_0)
-  [Function `add_fungibility`](#0x1_fungible_asset_add_fungibility)
-  [Function `generate_mint_ref`](#0x1_fungible_asset_generate_mint_ref)
-  [Function `generate_burn_ref`](#0x1_fungible_asset_generate_burn_ref)
-  [Function `generate_transfer_ref`](#0x1_fungible_asset_generate_transfer_ref)
-  [Function `supply`](#0x1_fungible_asset_supply)
-  [Function `maximum`](#0x1_fungible_asset_maximum)
-  [Function `name`](#0x1_fungible_asset_name)
-  [Function `symbol`](#0x1_fungible_asset_symbol)
-  [Function `decimals`](#0x1_fungible_asset_decimals)
-  [Function `icon_uri`](#0x1_fungible_asset_icon_uri)
-  [Function `project_uri`](#0x1_fungible_asset_project_uri)
-  [Function `store_exists`](#0x1_fungible_asset_store_exists)
-  [Function `metadata_from_asset`](#0x1_fungible_asset_metadata_from_asset)
-  [Function `store_metadata`](#0x1_fungible_asset_store_metadata)
-  [Function `amount`](#0x1_fungible_asset_amount)
-  [Function `balance`](#0x1_fungible_asset_balance)
-  [Function `check_minimum_balance`](#0x1_fungible_asset_check_minimum_balance)
-  [Function `is_frozen`](#0x1_fungible_asset_is_frozen)
-  [Function `asset_metadata`](#0x1_fungible_asset_asset_metadata)
-  [Function `mint_ref_metadata`](#0x1_fungible_asset_mint_ref_metadata)
-  [Function `transfer_ref_metadata`](#0x1_fungible_asset_transfer_ref_metadata)
-  [Function `burn_ref_metadata`](#0x1_fungible_asset_burn_ref_metadata)
-  [Function `transfer`](#0x1_fungible_asset_transfer)
-  [Function `create_store`](#0x1_fungible_asset_create_store)
-  [Function `remove_store`](#0x1_fungible_asset_remove_store)
-  [Function `withdraw`](#0x1_fungible_asset_withdraw)
-  [Function `deposit`](#0x1_fungible_asset_deposit)
-  [Function `mint`](#0x1_fungible_asset_mint)
-  [Function `mint_to`](#0x1_fungible_asset_mint_to)
-  [Function `set_frozen_flag`](#0x1_fungible_asset_set_frozen_flag)
-  [Function `burn`](#0x1_fungible_asset_burn)
-  [Function `burn_from`](#0x1_fungible_asset_burn_from)
-  [Function `withdraw_with_ref`](#0x1_fungible_asset_withdraw_with_ref)
-  [Function `deposit_with_ref`](#0x1_fungible_asset_deposit_with_ref)
-  [Function `transfer_with_ref`](#0x1_fungible_asset_transfer_with_ref)
-  [Function `zero`](#0x1_fungible_asset_zero)
-  [Function `extract`](#0x1_fungible_asset_extract)
-  [Function `merge`](#0x1_fungible_asset_merge)
-  [Function `destroy_zero`](#0x1_fungible_asset_destroy_zero)
-  [Function `deposit_internal`](#0x1_fungible_asset_deposit_internal)
-  [Function `withdraw_internal`](#0x1_fungible_asset_withdraw_internal)
-  [Function `withdraw_internal_without_event`](#0x1_fungible_asset_withdraw_internal_without_event)
-  [Function `increase_supply`](#0x1_fungible_asset_increase_supply)
-  [Function `decrease_supply`](#0x1_fungible_asset_decrease_supply)
-  [Function `borrow_fungible_metadata`](#0x1_fungible_asset_borrow_fungible_metadata)
-  [Function `borrow_fungible_metadata_mut`](#0x1_fungible_asset_borrow_fungible_metadata_mut)
-  [Function `borrow_store_resource`](#0x1_fungible_asset_borrow_store_resource)
-  [Function `upgrade_to_concurrent`](#0x1_fungible_asset_upgrade_to_concurrent)
-  [Function `destroy_burn_cap`](#0x1_fungible_asset_destroy_burn_cap)
-  [Function `destroy_mint_cap`](#0x1_fungible_asset_destroy_mint_cap)
-  [Function `destroy_transfer_cap`](#0x1_fungible_asset_destroy_transfer_cap)
-  [Function `metadata_from_aggregatable_coin`](#0x1_fungible_asset_metadata_from_aggregatable_coin)
-  [Function `set_icon_uri`](#0x1_fungible_asset_set_icon_uri)
-  [Function `set_project_uri`](#0x1_fungible_asset_set_project_uri)
-  [Function `initialize_aggregatable_coin`](#0x1_fungible_asset_initialize_aggregatable_coin)
-  [Function `is_aggregatable_coin_zero`](#0x1_fungible_asset_is_aggregatable_coin_zero)
-  [Function `extract_aggregatable_coin`](#0x1_fungible_asset_extract_aggregatable_coin)
-  [Function `drain_aggregatable_coin`](#0x1_fungible_asset_drain_aggregatable_coin)
-  [Function `merge_aggregatable_coin`](#0x1_fungible_asset_merge_aggregatable_coin)
-  [Function `collect_into_aggregatable_coin`](#0x1_fungible_asset_collect_into_aggregatable_coin)


<pre><code><b>use</b> <a href="aggregator_v2.md#0x1_aggregator_v2">0x1::aggregator_v2</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/features.md#0x1_features">0x1::features</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_fungible_asset_Supply"></a>

## Resource `Supply`



<pre><code>#[resource_group_member(#[group = <a href="object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>maximum: <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_ConcurrentSupply"></a>

## Resource `ConcurrentSupply`



<pre><code>#[resource_group_member(#[group = <a href="object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>current: <a href="aggregator_v2.md#0x1_aggregator_v2_Aggregator">aggregator_v2::Aggregator</a>&lt;u128&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_Metadata"></a>

## Resource `Metadata`

Metadata of a Fungible asset


<pre><code>#[resource_group_member(#[group = <a href="object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Name of the fungible metadata, i.e., "USDT".
</dd>
<dt>
<code>symbol: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Symbol of the fungible metadata, usually a shorter version of the name.
 For example, Singapore Dollar is SGD.
</dd>
<dt>
<code>decimals: u8</code>
</dt>
<dd>
 Number of decimals used for display purposes.
 For example, if <code>decimals</code> equals <code>2</code>, a balance of <code>505</code> coins should
 be displayed to a user as <code>5.05</code> (<code>505 / 10 ** 2</code>).
</dd>
<dt>
<code>icon_uri: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to an image that can be used as the icon for this fungible
 asset.
</dd>
<dt>
<code>project_uri: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the website for the fungible asset.
</dd>
</dl>


</details>

<a id="0x1_fungible_asset_FungibleStore"></a>

## Resource `FungibleStore`

The store object that holds fungible assets of a specific type associated with an account.


<pre><code>#[resource_group_member(#[group = <a href="object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>
 The address of the base metadata object.
</dd>
<dt>
<code>balance: <a href="aggregator_v2.md#0x1_aggregator_v2_Aggregator">aggregator_v2::Aggregator</a>&lt;u128&gt;</code>
</dt>
<dd>
 The balance of the fungible metadata.
</dd>
<dt>
<code>frozen: bool</code>
</dt>
<dd>
 If true, owner transfer is disabled that only <code><a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a></code> can move in/out from this store.
</dd>
</dl>


</details>

<a id="0x1_fungible_asset_FungibleAsset"></a>

## Struct `FungibleAsset`

FungibleAsset can be passed into function for type safety and to guarantee a specific amount.
FungibleAsset is ephemeral and cannot be stored directly. It must be deposited back into a store.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_AggregatableCoin"></a>

## Struct `AggregatableCoin`

Represents a coin with aggregator as its value. This allows to update
the coin in every transaction avoiding read-modify-write conflicts. Only
used for gas fees distribution by Endless Framework (0x1).


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="aggregator_v2.md#0x1_aggregator_v2_Aggregator">aggregator_v2::Aggregator</a>&lt;u128&gt;</code>
</dt>
<dd>
 Amount of aggregatable coin this address has.
</dd>
</dl>


</details>

<a id="0x1_fungible_asset_MintRef"></a>

## Struct `MintRef`

MintRef can be used to mint the fungible asset into an account's store.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_TransferRef"></a>

## Struct `TransferRef`

TransferRef can be used to allow or disallow the owner of fungible assets from transferring the asset
and allow the holder of TransferRef to transfer fungible assets from any account.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_BurnRef"></a>

## Struct `BurnRef`

BurnRef can be used to burn fungible assets from a given holder account.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_Deposit"></a>

## Struct `Deposit`

Emitted when fungible assets are deposited into a store.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Deposit">Deposit</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>store: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_Withdraw"></a>

## Struct `Withdraw`

Emitted when fungible assets are withdrawn from a store.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Withdraw">Withdraw</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>store: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_fungible_asset_Frozen"></a>

## Struct `Frozen`

Emitted when a store's frozen status is updated.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Frozen">Frozen</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>store: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>frozen: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_fungible_asset_MAX_U64"></a>

Maximum possible aggregatable coin value.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a id="0x1_fungible_asset_MAX_U128"></a>

Maximum possible coin supply.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_U128">MAX_U128</a>: u128 = 340282366920938463463374607431768211455;
</code></pre>



<a id="0x1_fungible_asset_EAGGREGATABLE_COIN_METADATA_MISMATCH"></a>

Metadata of aggregatable coin and FungibleAsset do not match


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EAGGREGATABLE_COIN_METADATA_MISMATCH">EAGGREGATABLE_COIN_METADATA_MISMATCH</a>: u64 = 23;
</code></pre>



<a id="0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO"></a>

Amount cannot be zero.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO">EAMOUNT_CANNOT_BE_ZERO</a>: u64 = 1;
</code></pre>



<a id="0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO"></a>

Cannot destroy non-empty fungible assets.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO">EAMOUNT_IS_NOT_ZERO</a>: u64 = 12;
</code></pre>



<a id="0x1_fungible_asset_EBALANCE_IS_NOT_ZERO"></a>

Cannot destroy fungible stores with a non-zero balance.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EBALANCE_IS_NOT_ZERO">EBALANCE_IS_NOT_ZERO</a>: u64 = 14;
</code></pre>



<a id="0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH"></a>

Burn ref and fungible asset do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH">EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>: u64 = 13;
</code></pre>



<a id="0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH"></a>

Burn ref and store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH">EBURN_REF_AND_STORE_MISMATCH</a>: u64 = 10;
</code></pre>



<a id="0x1_fungible_asset_ECONCURRENT_SUPPLY_NOT_ENABLED"></a>

Flag for Concurrent Supply not enabled


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ECONCURRENT_SUPPLY_NOT_ENABLED">ECONCURRENT_SUPPLY_NOT_ENABLED</a>: u64 = 22;
</code></pre>



<a id="0x1_fungible_asset_EDECIMALS_TOO_LARGE"></a>

Decimals is over the maximum of 32


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EDECIMALS_TOO_LARGE">EDECIMALS_TOO_LARGE</a>: u64 = 17;
</code></pre>



<a id="0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH"></a>

Fungible asset and store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH">EFUNGIBLE_ASSET_AND_STORE_MISMATCH</a>: u64 = 11;
</code></pre>



<a id="0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH"></a>

Fungible asset do not match when merging.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH">EFUNGIBLE_ASSET_MISMATCH</a>: u64 = 6;
</code></pre>



<a id="0x1_fungible_asset_EINSUFFICIENT_BALANCE"></a>

Insufficient balance to withdraw or transfer.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EINSUFFICIENT_BALANCE">EINSUFFICIENT_BALANCE</a>: u64 = 4;
</code></pre>



<a id="0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED"></a>

The fungible asset's supply has exceeded maximum.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED">EMAX_SUPPLY_EXCEEDED</a>: u64 = 5;
</code></pre>



<a id="0x1_fungible_asset_EMINT_REF_AND_STORE_MISMATCH"></a>

The mint ref and the the store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EMINT_REF_AND_STORE_MISMATCH">EMINT_REF_AND_STORE_MISMATCH</a>: u64 = 7;
</code></pre>



<a id="0x1_fungible_asset_ENAME_TOO_LONG"></a>

Name of the fungible asset metadata is too long


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ENAME_TOO_LONG">ENAME_TOO_LONG</a>: u64 = 15;
</code></pre>



<a id="0x1_fungible_asset_ENOT_STORE_OWNER"></a>

Account is not the store's owner.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ENOT_STORE_OWNER">ENOT_STORE_OWNER</a>: u64 = 8;
</code></pre>



<a id="0x1_fungible_asset_EOBJECT_IS_DELETABLE"></a>

Fungibility is only available for non-deletable objects.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EOBJECT_IS_DELETABLE">EOBJECT_IS_DELETABLE</a>: u64 = 18;
</code></pre>



<a id="0x1_fungible_asset_ESTORE_IS_FROZEN"></a>

Store is disabled from sending and receiving this fungible asset.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>: u64 = 3;
</code></pre>



<a id="0x1_fungible_asset_ESUPPLY_NOT_FOUND"></a>

Supply resource is not found for a metadata object.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_NOT_FOUND">ESUPPLY_NOT_FOUND</a>: u64 = 21;
</code></pre>



<a id="0x1_fungible_asset_ESUPPLY_UNDERFLOW"></a>

The fungible asset's supply will be negative which should be impossible.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_UNDERFLOW">ESUPPLY_UNDERFLOW</a>: u64 = 20;
</code></pre>



<a id="0x1_fungible_asset_ESYMBOL_TOO_LONG"></a>

Symbol of the fungible asset metadata is too long


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESYMBOL_TOO_LONG">ESYMBOL_TOO_LONG</a>: u64 = 16;
</code></pre>



<a id="0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH"></a>

The transfer ref and the fungible asset do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH">ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>: u64 = 2;
</code></pre>



<a id="0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH"></a>

Transfer ref and store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH">ETRANSFER_REF_AND_STORE_MISMATCH</a>: u64 = 9;
</code></pre>



<a id="0x1_fungible_asset_EURI_TOO_LONG"></a>

URI for the icon of the fungible asset metadata is too long


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>: u64 = 19;
</code></pre>



<a id="0x1_fungible_asset_MAX_DECIMALS"></a>



<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_DECIMALS">MAX_DECIMALS</a>: u8 = 32;
</code></pre>



<a id="0x1_fungible_asset_MAX_NAME_LENGTH"></a>



<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_NAME_LENGTH">MAX_NAME_LENGTH</a>: u64 = 32;
</code></pre>



<a id="0x1_fungible_asset_MAX_SYMBOL_LENGTH"></a>



<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_SYMBOL_LENGTH">MAX_SYMBOL_LENGTH</a>: u64 = 10;
</code></pre>



<a id="0x1_fungible_asset_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>: u64 = 512;
</code></pre>



<a id="0x1_fungible_asset_add_fungibility"></a>

## Function `add_fungibility`

Make an existing object fungible by adding the Metadata resource.
This returns the capabilities to mint, burn, and transfer.
maximum_supply defines the behavior of maximum supply when monitoring:
- option::none(): Monitoring unlimited supply
(width of the field - MAX_U128 is the implicit maximum supply)
if option::some(MAX_U128) is used, it is treated as unlimited supply.
- option::some(max): Monitoring fixed supply with <code>max</code> as the maximum supply.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_add_fungibility">add_fungibility</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, maximum_supply: <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;, name: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, decimals: u8, icon_uri: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, project_uri: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_add_fungibility">add_fungibility</a>(
    constructor_ref: &ConstructorRef,
    maximum_supply: Option&lt;u128&gt;,
    name: String,
    symbol: String,
    decimals: u8,
    icon_uri: String,
    project_uri: String,
): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    <b>assert</b>!(!<a href="object.md#0x1_object_can_generate_delete_ref">object::can_generate_delete_ref</a>(constructor_ref), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EOBJECT_IS_DELETABLE">EOBJECT_IS_DELETABLE</a>));
    <b>let</b> metadata_object_signer = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&name) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_NAME_LENGTH">MAX_NAME_LENGTH</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_ENAME_TOO_LONG">ENAME_TOO_LONG</a>));
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&symbol) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_SYMBOL_LENGTH">MAX_SYMBOL_LENGTH</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESYMBOL_TOO_LONG">ESYMBOL_TOO_LONG</a>));
    <b>assert</b>!(<a href="fungible_asset.md#0x1_fungible_asset_decimals">decimals</a> &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_DECIMALS">MAX_DECIMALS</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EDECIMALS_TOO_LARGE">EDECIMALS_TOO_LARGE</a>));
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&icon_uri) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>));
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&project_uri) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>));
    <b>move_to</b>(metadata_object_signer,
        <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
            name,
            symbol,
            decimals,
            icon_uri,
            project_uri,
        }
    );

    <b>if</b> (<a href="../../endless-stdlib/../move-stdlib/doc/features.md#0x1_features_concurrent_fungible_assets_enabled">features::concurrent_fungible_assets_enabled</a>()) {
        <b>let</b> unlimited = <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&maximum_supply);
        <b>move_to</b>(metadata_object_signer, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
            current: <b>if</b> (unlimited) {
                <a href="aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator">aggregator_v2::create_unbounded_aggregator</a>()
            } <b>else</b> {
                <a href="aggregator_v2.md#0x1_aggregator_v2_create_aggregator">aggregator_v2::create_aggregator</a>(<a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> maximum_supply))
            },
        });
    } <b>else</b> {
        <b>move_to</b>(metadata_object_signer, <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
            current: 0,
            maximum: maximum_supply
        });
    };

    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_generate_mint_ref"></a>

## Function `generate_mint_ref`

Creates a mint ref that can be used to mint fungible assets from the given fungible object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_mint_ref">generate_mint_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_mint_ref">generate_mint_ref</a>(constructor_ref: &ConstructorRef): <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> { metadata }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a burn ref that can be used to burn fungible assets from the given fungible object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_burn_ref">generate_burn_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_burn_ref">generate_burn_ref</a>(constructor_ref: &ConstructorRef): <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> { metadata }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_generate_transfer_ref"></a>

## Function `generate_transfer_ref`

Creates a transfer ref that can be used to freeze/unfreeze/transfer fungible assets from the given fungible
object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_transfer_ref">generate_transfer_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_transfer_ref">generate_transfer_ref</a>(constructor_ref: &ConstructorRef): <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> { metadata }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_supply"></a>

## Function `supply`

Get the current supply from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">supply</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">supply</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): Option&lt;u128&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address);
        <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="aggregator_v2.md#0x1_aggregator_v2_read">aggregator_v2::read</a>(&supply.current))
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address);
        <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(supply.current)
    } <b>else</b> {
        <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_maximum"></a>

## Function `maximum`

Get the maximum supply from the <code>metadata</code> object.
If supply is unlimited (or set explicitly to MAX_U128), none is returned


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_maximum">maximum</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_maximum">maximum</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): Option&lt;u128&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address);
        <b>let</b> max_value = <a href="aggregator_v2.md#0x1_aggregator_v2_max_value">aggregator_v2::max_value</a>(&supply.current);
        <b>if</b> (max_value == <a href="fungible_asset.md#0x1_fungible_asset_MAX_U128">MAX_U128</a>) {
            <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>()
        } <b>else</b> {
            <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(max_value)
        }
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address);
        supply.maximum
    } <b>else</b> {
        <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_name"></a>

## Function `name`

Get the name of the fungible asset from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_name">name</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_name">name</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).name
}
</code></pre>



</details>

<a id="0x1_fungible_asset_symbol"></a>

## Function `symbol`

Get the symbol of the fungible asset from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_symbol">symbol</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_symbol">symbol</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).symbol
}
</code></pre>



</details>

<a id="0x1_fungible_asset_decimals"></a>

## Function `decimals`

Get the decimals from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_decimals">decimals</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_decimals">decimals</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): u8 <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).decimals
}
</code></pre>



</details>

<a id="0x1_fungible_asset_icon_uri"></a>

## Function `icon_uri`

Get the icon uri from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_icon_uri">icon_uri</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_icon_uri">icon_uri</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).icon_uri
}
</code></pre>



</details>

<a id="0x1_fungible_asset_project_uri"></a>

## Function `project_uri`

Get the project uri from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_project_uri">project_uri</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_project_uri">project_uri</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).project_uri
}
</code></pre>



</details>

<a id="0x1_fungible_asset_store_exists"></a>

## Function `store_exists`

Return whether the provided address has a store initialized.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(store: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(store: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_metadata_from_asset"></a>

## Function `metadata_from_asset`

Return the underlying metadata object


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">metadata_from_asset</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">metadata_from_asset</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    fa.metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_store_metadata"></a>

## Function `store_metadata`

Return the underlying metadata object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>&lt;T: key&gt;(store: Object&lt;T&gt;): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_amount"></a>

## Function `amount`

Return the <code>amount</code> of a given fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_amount">amount</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_amount">amount</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>): u128 {
    fa.amount
}
</code></pre>



</details>

<a id="0x1_fungible_asset_balance"></a>

## Function `balance`

Get the balance of a given store.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_balance">balance</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_balance">balance</a>&lt;T: key&gt;(store: Object&lt;T&gt;): u128 <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>if</b> (<a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store))) {
        <a href="aggregator_v2.md#0x1_aggregator_v2_read">aggregator_v2::read</a>(&<a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).balance)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_check_minimum_balance"></a>

## Function `check_minimum_balance`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_check_minimum_balance">check_minimum_balance</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, least: u128): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_check_minimum_balance">check_minimum_balance</a>&lt;T: key&gt;(store: Object&lt;T&gt;, least: u128): bool <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>if</b> (least == 0) {
        <b>return</b> <b>true</b>
    };
    <b>if</b> (<a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store))) {
        <a href="aggregator_v2.md#0x1_aggregator_v2_greater_than_or_equal">aggregator_v2::greater_than_or_equal</a>(&<a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).balance, least)
    } <b>else</b> {
        <b>false</b>
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_is_frozen"></a>

## Function `is_frozen`

Return whether a store is frozen.

If the store has not been created, we default to returning false so deposits can be sent to it.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>&lt;T: key&gt;(store: Object&lt;T&gt;): bool <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store)) && <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).frozen
}
</code></pre>



</details>

<a id="0x1_fungible_asset_asset_metadata"></a>

## Function `asset_metadata`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">asset_metadata</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">asset_metadata</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    fa.metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_mint_ref_metadata"></a>

## Function `mint_ref_metadata`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_ref_metadata">mint_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_ref_metadata">mint_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_transfer_ref_metadata"></a>

## Function `transfer_ref_metadata`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">transfer_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">transfer_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_burn_ref_metadata"></a>

## Function `burn_ref_metadata`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_ref_metadata">burn_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_ref_metadata">burn_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_transfer"></a>

## Function `transfer`

Transfer an <code>amount</code> of fungible asset from <code>from_store</code>, which should be owned by <code>sender</code>, to <code>receiver</code>.
Note: it does not move the underlying object.


<pre><code><b>public</b> entry <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer">transfer</a>&lt;T: key&gt;(sender: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, from: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <b>to</b>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer">transfer</a>&lt;T: key&gt;(
    sender: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    from: Object&lt;T&gt;,
    <b>to</b>: Object&lt;T&gt;,
    amount: u128,
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> fa = <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>(sender, from, amount);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>(<b>to</b>, fa);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_create_store"></a>

## Function `create_store`

Allow an object to hold a store for fungible assets.
Applications can use this to create multiple stores for isolating fungible assets for different purposes.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_create_store">create_store</a>&lt;T: key&gt;(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_create_store">create_store</a>&lt;T: key&gt;(
    constructor_ref: &ConstructorRef,
    metadata: Object&lt;T&gt;,
): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt; {
    <b>let</b> store_obj = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);
    <b>move_to</b>(store_obj, <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
        metadata: <a href="object.md#0x1_object_convert">object::convert</a>(metadata),
        balance: <a href="aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator">aggregator_v2::create_unbounded_aggregator</a>(),
        frozen: <b>false</b>,
    });
    <b>spec</b> {
        <b>assume</b> <b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(constructor_ref.self);
        <b>assume</b> <a href="object.md#0x1_object_spec_exists_at">object::spec_exists_at</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(constructor_ref.self);
        <b>assume</b> <b>global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(constructor_ref.self).metadata.inner == metadata.inner;
    };
    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(constructor_ref)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_remove_store"></a>

## Function `remove_store`

Used to delete a store.  Requires the store to be completely empty prior to removing it


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_remove_store">remove_store</a>(delete_ref: &<a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_remove_store">remove_store</a>(delete_ref: &DeleteRef) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> store = &<a href="object.md#0x1_object_object_from_delete_ref">object::object_from_delete_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(delete_ref);
    <b>let</b> addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(store);
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> { metadata: _, balance, frozen: _ }
        = <b>move_from</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(addr);
    <b>assert</b>!(<a href="aggregator_v2.md#0x1_aggregator_v2_read">aggregator_v2::read</a>(&balance) == 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_EBALANCE_IS_NOT_ZERO">EBALANCE_IS_NOT_ZERO</a>));
}
</code></pre>



</details>

<a id="0x1_fungible_asset_withdraw"></a>

## Function `withdraw`

Withdraw <code>amount</code> of the fungible asset from <code>store</code> by the owner.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>&lt;T: key&gt;(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>&lt;T: key&gt;(
    owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    store: Object&lt;T&gt;,
    amount: u128,
): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(<a href="object.md#0x1_object_owns">object::owns</a>(store, <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner)), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_ENOT_STORE_OWNER">ENOT_STORE_OWNER</a>));
    <b>assert</b>!(!<a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>(store), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>));
    <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), amount)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_deposit"></a>

## Function `deposit`

Deposit <code>amount</code> of the fungible asset to <code>store</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>&lt;T: key&gt;(store: Object&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(!<a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>(store), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>));
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(store, fa);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_mint"></a>

## Function `mint`

Mint the specified <code>amount</code> of the fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint">mint</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint">mint</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    // <b>assert</b>!(amount &gt; 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO">EAMOUNT_CANNOT_BE_ZERO</a>));
    <b>let</b> metadata = ref.metadata;
    <a href="fungible_asset.md#0x1_fungible_asset_increase_supply">increase_supply</a>(&metadata, amount);

    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
        metadata,
        amount
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_mint_to"></a>

## Function `mint_to`

Mint the specified <code>amount</code> of the fungible asset to a destination store.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_to">mint_to</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_to">mint_to</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>, store: Object&lt;T&gt;, amount: u128)
<b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>(store, <a href="fungible_asset.md#0x1_fungible_asset_mint">mint</a>(ref, amount));
}
</code></pre>



</details>

<a id="0x1_fungible_asset_set_frozen_flag"></a>

## Function `set_frozen_flag`

Enable/disable a store's ability to do direct transfers of the fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_frozen_flag">set_frozen_flag</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, frozen: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_frozen_flag">set_frozen_flag</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>,
    store: Object&lt;T&gt;,
    frozen: bool,
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        ref.metadata == <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store),
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH">ETRANSFER_REF_AND_STORE_MISMATCH</a>),
    );
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr).frozen = frozen;

    <b>let</b> coin = <a href="object.md#0x1_object_object_address">object::object_address</a>(&ref.metadata);
    <b>let</b> owner = <a href="object.md#0x1_object_owner">object::owner</a>(store);

    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="fungible_asset.md#0x1_fungible_asset_Frozen">Frozen</a> { store: store_addr, owner, coin, frozen });
}
</code></pre>



</details>

<a id="0x1_fungible_asset_burn"></a>

## Function `burn`

Burns a fungible asset


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn">burn</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn">burn</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
        metadata,
        amount,
    } = fa;
    <b>assert</b>!(ref.metadata == metadata, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH">EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>));
    <a href="fungible_asset.md#0x1_fungible_asset_decrease_supply">decrease_supply</a>(&metadata, amount);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_burn_from"></a>

## Function `burn_from`

Burn the <code>amount</code> of the fungible asset from the given store.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_from">burn_from</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_from">burn_from</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>,
    store: Object&lt;T&gt;,
    amount: u128
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    <b>let</b> metadata = ref.metadata;
    <b>assert</b>!(metadata == <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH">EBURN_REF_AND_STORE_MISMATCH</a>));
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <a href="fungible_asset.md#0x1_fungible_asset_burn">burn</a>(ref, <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(store_addr, amount));
}
</code></pre>



</details>

<a id="0x1_fungible_asset_withdraw_with_ref"></a>

## Function `withdraw_with_ref`

Withdraw <code>amount</code> of the fungible asset from the <code>store</code> ignoring <code>frozen</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">withdraw_with_ref</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">withdraw_with_ref</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>,
    store: Object&lt;T&gt;,
    amount: u128
): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        ref.metadata == <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store),
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH">ETRANSFER_REF_AND_STORE_MISMATCH</a>),
    );
    <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), amount)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_deposit_with_ref"></a>

## Function `deposit_with_ref`

Deposit the fungible asset into the <code>store</code> ignoring <code>frozen</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">deposit_with_ref</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">deposit_with_ref</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>,
    store: Object&lt;T&gt;,
    fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        ref.metadata == fa.metadata,
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH">ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>)
    );
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(store, fa);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer <code>amount</code> of the fungible asset with <code><a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a></code> even it is frozen.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_with_ref">transfer_with_ref</a>&lt;T: key&gt;(transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, from: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <b>to</b>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_with_ref">transfer_with_ref</a>&lt;T: key&gt;(
    transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>,
    from: Object&lt;T&gt;,
    <b>to</b>: Object&lt;T&gt;,
    amount: u128,
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> fa = <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">withdraw_with_ref</a>(transfer_ref, from, amount);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">deposit_with_ref</a>(transfer_ref, <b>to</b>, fa);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_zero"></a>

## Function `zero`

Create a fungible asset with zero amount.
This can be useful when starting a series of computations where the initial value is 0.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">zero</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">zero</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
        metadata: <a href="object.md#0x1_object_convert">object::convert</a>(metadata),
        amount: 0,
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_extract"></a>

## Function `extract`

Extract a given amount from the given fungible asset and return a new one.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_extract">extract</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_extract">extract</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
    <b>assert</b>!(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.amount &gt;= amount, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EINSUFFICIENT_BALANCE">EINSUFFICIENT_BALANCE</a>));
    <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.amount = <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.amount - amount;
    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
        metadata: <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.metadata,
        amount,
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_merge"></a>

## Function `merge`

"Merges" the two given fungible assets. The fungible asset passed in as <code>dst_fungible_asset</code> will have a value
equal to the sum of the two (<code>dst_fungible_asset</code> and <code>src_fungible_asset</code>).


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_merge">merge</a>(dst_fungible_asset: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, src_fungible_asset: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_merge">merge</a>(dst_fungible_asset: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>, src_fungible_asset: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount } = src_fungible_asset;
    <b>assert</b>!(metadata == dst_fungible_asset.metadata, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH">EFUNGIBLE_ASSET_MISMATCH</a>));
    dst_fungible_asset.amount = dst_fungible_asset.amount + amount;
}
</code></pre>



</details>

<a id="0x1_fungible_asset_destroy_zero"></a>

## Function `destroy_zero`

Destroy an empty fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">destroy_zero</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">destroy_zero</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { amount, metadata: _ } = <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>;
    <b>assert</b>!(amount == 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO">EAMOUNT_IS_NOT_ZERO</a>));
}
</code></pre>



</details>

<a id="0x1_fungible_asset_deposit_internal"></a>

## Function `deposit_internal`



<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>&lt;T: key&gt;(store: Object&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount } = fa;
    <b>if</b> (amount == 0) <b>return</b>;

    <b>let</b> store_metadata = <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store);
    <b>assert</b>!(metadata == store_metadata, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH">EFUNGIBLE_ASSET_AND_STORE_MISMATCH</a>));
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <b>let</b> owner = <a href="object.md#0x1_object_owner">object::owner</a>(store);
    <b>let</b> store = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr);
    // <b>spec</b> {
    //     <b>assume</b> store.balance + <a href="fungible_asset.md#0x1_fungible_asset_amount">amount</a> &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_U64">MAX_U64</a>;
    // };
    <a href="aggregator_v2.md#0x1_aggregator_v2_add">aggregator_v2::add</a>(&<b>mut</b> store.balance, amount);

    <b>let</b> coin = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Deposit">Deposit</a>&gt;(<a href="fungible_asset.md#0x1_fungible_asset_Deposit">Deposit</a> { store: store_addr, owner, coin, amount });
}
</code></pre>



</details>

<a id="0x1_fungible_asset_withdraw_internal"></a>

## Function `withdraw_internal`

Extract <code>amount</code> of the fungible asset from <code>store</code>.


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(store_addr: <b>address</b>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(
    store_addr: <b>address</b>,
    amount: u128,
): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(amount != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO">EAMOUNT_CANNOT_BE_ZERO</a>));
    <b>let</b> store = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr);
    <b>assert</b>!(<a href="aggregator_v2.md#0x1_aggregator_v2_try_sub">aggregator_v2::try_sub</a>(&<b>mut</b> store.balance, amount), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EINSUFFICIENT_BALANCE">EINSUFFICIENT_BALANCE</a>));

    <b>let</b> metadata = store.metadata;
    <b>let</b> owner = <a href="object.md#0x1_object_owner">object::owner</a>(<a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr));
    <b>let</b> coin = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Withdraw">Withdraw</a>&gt;(<a href="fungible_asset.md#0x1_fungible_asset_Withdraw">Withdraw</a> { store: store_addr, owner, coin, amount });

    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_withdraw_internal_without_event"></a>

## Function `withdraw_internal_without_event`

Extract <code>amount</code> of the fungible asset from <code>store</code> without a Withdraw event.


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal_without_event">withdraw_internal_without_event</a>(store_addr: <b>address</b>, amount: u128): (bool, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal_without_event">withdraw_internal_without_event</a>(
    store_addr: <b>address</b>,
    amount: u128,
): (bool, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(amount != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO">EAMOUNT_CANNOT_BE_ZERO</a>));
    <b>let</b> store = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr);
    <b>let</b> metadata = store.metadata;

    <b>if</b> (<a href="aggregator_v2.md#0x1_aggregator_v2_try_sub">aggregator_v2::try_sub</a>(&<b>mut</b> store.balance, amount)) {
        (<b>true</b>, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount })
    } <b>else</b> {
        (<b>false</b>, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount: 0 })
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_increase_supply"></a>

## Function `increase_supply`

Increase the supply of a fungible asset by minting.


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_increase_supply">increase_supply</a>&lt;T: key&gt;(metadata: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_increase_supply">increase_supply</a>&lt;T: key&gt;(metadata: &Object&lt;T&gt;, amount: u128) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    // <b>assert</b>!(amount != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO">EAMOUNT_CANNOT_BE_ZERO</a>));
    <b>if</b> (amount == 0) {
        <b>return</b>
    };

    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(metadata);

    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address);
        <b>assert</b>!(
            <a href="aggregator_v2.md#0x1_aggregator_v2_try_add">aggregator_v2::try_add</a>(&<b>mut</b> supply.current, (amount <b>as</b> u128)),
            <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED">EMAX_SUPPLY_EXCEEDED</a>)
        );
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address);
        <b>if</b> (<a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&supply.maximum)) {
            <b>let</b> max = *<a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow_mut">option::borrow_mut</a>(&<b>mut</b> supply.maximum);
            <b>assert</b>!(
                max - supply.current &gt;= (amount <b>as</b> u128),
                <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED">EMAX_SUPPLY_EXCEEDED</a>)
            )
        };
        supply.current = supply.current + (amount <b>as</b> u128);
    } <b>else</b> {
        <b>assert</b>!(<b>false</b>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_NOT_FOUND">ESUPPLY_NOT_FOUND</a>));
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_decrease_supply"></a>

## Function `decrease_supply`

Decrease the supply of a fungible asset by burning.


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_decrease_supply">decrease_supply</a>&lt;T: key&gt;(metadata: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_decrease_supply">decrease_supply</a>&lt;T: key&gt;(metadata: &Object&lt;T&gt;, amount: u128) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
    <b>assert</b>!(amount != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_CANNOT_BE_ZERO">EAMOUNT_CANNOT_BE_ZERO</a>));
    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(metadata);

    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a>&gt;(metadata_address);

        <b>assert</b>!(
            <a href="aggregator_v2.md#0x1_aggregator_v2_try_sub">aggregator_v2::try_sub</a>(&<b>mut</b> supply.current, (amount <b>as</b> u128)),
            <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_UNDERFLOW">ESUPPLY_UNDERFLOW</a>)
        );
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address)) {
        <b>assert</b>!(<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_NOT_FOUND">ESUPPLY_NOT_FOUND</a>));
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address);
        <b>assert</b>!(
            supply.current &gt;= (amount <b>as</b> u128),
            <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_UNDERFLOW">ESUPPLY_UNDERFLOW</a>)
        );
        supply.current = supply.current - (amount <b>as</b> u128);
    } <b>else</b> {
        <b>assert</b>!(<b>false</b>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_NOT_FOUND">ESUPPLY_NOT_FOUND</a>));
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_borrow_fungible_metadata"></a>

## Function `borrow_fungible_metadata`



<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>&lt;T: key&gt;(metadata: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): &<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>&lt;T: key&gt;(
    metadata: &Object&lt;T&gt;
): &<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <b>let</b> addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(metadata);
    <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(addr)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_borrow_fungible_metadata_mut"></a>

## Function `borrow_fungible_metadata_mut`



<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata_mut">borrow_fungible_metadata_mut</a>&lt;T: key&gt;(metadata: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata_mut">borrow_fungible_metadata_mut</a>&lt;T: key&gt;(
    metadata: &Object&lt;T&gt;
): &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <b>let</b> addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(metadata);
    <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(addr)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_borrow_store_resource"></a>

## Function `borrow_store_resource`



<pre><code><b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>&lt;T: key&gt;(store: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): &<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>&lt;T: key&gt;(store: &Object&lt;T&gt;): &<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(store))
}
</code></pre>



</details>

<a id="0x1_fungible_asset_upgrade_to_concurrent"></a>

## Function `upgrade_to_concurrent`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_upgrade_to_concurrent">upgrade_to_concurrent</a>(ref: &<a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_upgrade_to_concurrent">upgrade_to_concurrent</a>(
    ref: &ExtendRef,
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
    <b>let</b> metadata_object_address = <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(ref);
    <b>let</b> metadata_object_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(ref);
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/features.md#0x1_features_concurrent_fungible_assets_enabled">features::concurrent_fungible_assets_enabled</a>(), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ECONCURRENT_SUPPLY_NOT_ENABLED">ECONCURRENT_SUPPLY_NOT_ENABLED</a>));
    <b>assert</b>!(<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_object_address), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_NOT_FOUND">ESUPPLY_NOT_FOUND</a>));
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
        current,
        maximum,
    } = <b>move_from</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_object_address);

    <b>let</b> unlimited = <a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&maximum);
    <b>let</b> supply = <a href="fungible_asset.md#0x1_fungible_asset_ConcurrentSupply">ConcurrentSupply</a> {
        current: <b>if</b> (unlimited) {
            <a href="aggregator_v2.md#0x1_aggregator_v2_create_unbounded_aggregator">aggregator_v2::create_unbounded_aggregator</a>()
        }
        <b>else</b> {
            <a href="aggregator_v2.md#0x1_aggregator_v2_create_aggregator">aggregator_v2::create_aggregator</a>(<a href="../../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> maximum))
        },
    };
    // <b>update</b> current state:
    <a href="aggregator_v2.md#0x1_aggregator_v2_add">aggregator_v2::add</a>(&<b>mut</b> supply.current, current);
    <b>move_to</b>(&metadata_object_signer, supply);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_destroy_burn_cap"></a>

## Function `destroy_burn_cap`

Destroy a burn capability.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_burn_cap">destroy_burn_cap</a>(burn_cap: <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_burn_cap">destroy_burn_cap</a>(burn_cap: <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> {metadata:_} = burn_cap;
}
</code></pre>



</details>

<a id="0x1_fungible_asset_destroy_mint_cap"></a>

## Function `destroy_mint_cap`

Destroy a mint capability.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_mint_cap">destroy_mint_cap</a>(mint_cap: <a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_mint_cap">destroy_mint_cap</a>(mint_cap: <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> {metadata:_} = mint_cap;
}
</code></pre>



</details>

<a id="0x1_fungible_asset_destroy_transfer_cap"></a>

## Function `destroy_transfer_cap`

Destroy a transfer capability.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_transfer_cap">destroy_transfer_cap</a>(transfer_cap: <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_transfer_cap">destroy_transfer_cap</a>(transfer_cap: <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> {metadata:_} = transfer_cap;
}
</code></pre>



</details>

<a id="0x1_fungible_asset_metadata_from_aggregatable_coin"></a>

## Function `metadata_from_aggregatable_coin`

Return the underlying metadata object


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_aggregatable_coin">metadata_from_aggregatable_coin</a>(coin: &<a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_aggregatable_coin">metadata_from_aggregatable_coin</a>(coin: &<a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    coin.metadata
}
</code></pre>



</details>

<a id="0x1_fungible_asset_set_icon_uri"></a>

## Function `set_icon_uri`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_icon_uri">set_icon_uri</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, icon_uri: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_icon_uri">set_icon_uri</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;, icon_uri: String) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <b>assert</b>!(<a href="object.md#0x1_object_is_owner">object::is_owner</a>(metadata, <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner)), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_ENOT_STORE_OWNER">ENOT_STORE_OWNER</a>));
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&icon_uri) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>));
    <b>let</b> meta = <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata_mut">borrow_fungible_metadata_mut</a>(&metadata);
    meta.icon_uri = icon_uri;
}
</code></pre>



</details>

<a id="0x1_fungible_asset_set_project_uri"></a>

## Function `set_project_uri`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_project_uri">set_project_uri</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, project_uri: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_project_uri">set_project_uri</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;, project_uri: String) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <b>assert</b>!(<a href="object.md#0x1_object_is_owner">object::is_owner</a>(metadata, <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner)), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_ENOT_STORE_OWNER">ENOT_STORE_OWNER</a>));
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&project_uri) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>));
    <b>let</b> meta = <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata_mut">borrow_fungible_metadata_mut</a>(&metadata);
    meta.project_uri = project_uri;
}
</code></pre>



</details>

<a id="0x1_fungible_asset_initialize_aggregatable_coin"></a>

## Function `initialize_aggregatable_coin`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_initialize_aggregatable_coin">initialize_aggregatable_coin</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_initialize_aggregatable_coin">initialize_aggregatable_coin</a>(metadata: Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;): <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a> {
    <b>let</b> <a href="aggregator.md#0x1_aggregator">aggregator</a> = <a href="aggregator_v2.md#0x1_aggregator_v2_create_aggregator">aggregator_v2::create_aggregator</a>&lt;u128&gt;(<a href="fungible_asset.md#0x1_fungible_asset_MAX_U128">MAX_U128</a>);
    <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a> {
        metadata: metadata,
        value: <a href="aggregator.md#0x1_aggregator">aggregator</a>,
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_is_aggregatable_coin_zero"></a>

## Function `is_aggregatable_coin_zero`

Returns true if the value of aggregatable coin is zero.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_aggregatable_coin_zero">is_aggregatable_coin_zero</a>(coin: &<a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_aggregatable_coin_zero">is_aggregatable_coin_zero</a>(coin: &<a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a>): bool {
    <b>let</b> amount = <a href="aggregator_v2.md#0x1_aggregator_v2_read">aggregator_v2::read</a>(&coin.value);
    amount == 0
}
</code></pre>



</details>

<a id="0x1_fungible_asset_extract_aggregatable_coin"></a>

## Function `extract_aggregatable_coin`

Extract a given amount from the given aggregatable coin


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_extract_aggregatable_coin">extract_aggregatable_coin</a>(coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_extract_aggregatable_coin">extract_aggregatable_coin</a>(coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a>, amount: u128): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
    // underflow will not succeed in sub
    <a href="aggregator_v2.md#0x1_aggregator_v2_sub">aggregator_v2::sub</a>(&<b>mut</b> coin.value, amount);
    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
        metadata: coin.metadata,
        amount,
    }
}
</code></pre>



</details>

<a id="0x1_fungible_asset_drain_aggregatable_coin"></a>

## Function `drain_aggregatable_coin`

Drains the aggregatable coin, setting it to zero and returning a standard coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_drain_aggregatable_coin">drain_aggregatable_coin</a>(coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_drain_aggregatable_coin">drain_aggregatable_coin</a>(coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
    <b>let</b> amount = <a href="aggregator_v2.md#0x1_aggregator_v2_read">aggregator_v2::read</a>(&coin.value);
    <a href="fungible_asset.md#0x1_fungible_asset_extract_aggregatable_coin">extract_aggregatable_coin</a>(coin, amount)
}
</code></pre>



</details>

<a id="0x1_fungible_asset_merge_aggregatable_coin"></a>

## Function `merge_aggregatable_coin`

Merges <code>coin</code> into aggregatable coin (<code>dst_coin</code>).


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_merge_aggregatable_coin">merge_aggregatable_coin</a>(dst_coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>, coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_merge_aggregatable_coin">merge_aggregatable_coin</a>(
    dst_coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a>,
    coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount } = coin;
    <b>assert</b>!(dst_coin.metadata == metadata, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAGGREGATABLE_COIN_METADATA_MISMATCH">EAGGREGATABLE_COIN_METADATA_MISMATCH</a>));
    <a href="aggregator_v2.md#0x1_aggregator_v2_add">aggregator_v2::add</a>(&<b>mut</b> dst_coin.value, amount);
}
</code></pre>



</details>

<a id="0x1_fungible_asset_collect_into_aggregatable_coin"></a>

## Function `collect_into_aggregatable_coin`

Collects a specified amount of coin form an account into aggregatable coin.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_collect_into_aggregatable_coin">collect_into_aggregatable_coin</a>(store_addr: <b>address</b>, amount: u128, dst_coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">fungible_asset::AggregatableCoin</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_collect_into_aggregatable_coin">collect_into_aggregatable_coin</a>(
    store_addr: <b>address</b>,
    amount: u128,
    dst_coin: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_AggregatableCoin">AggregatableCoin</a>,
): bool <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    // Skip collecting <b>if</b> amount is zero.
    <b>if</b> (amount == 0) {
        <b>return</b> <b>true</b>
    };
    <b>let</b> (success, fa) = <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal_without_event">withdraw_internal_without_event</a>(store_addr, amount);
    <b>if</b> (success) {
        <a href="fungible_asset.md#0x1_fungible_asset_merge_aggregatable_coin">merge_aggregatable_coin</a>(dst_coin, fa);
        <b>true</b>
    } <b>else</b> {
        <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata: _, amount: _ } = fa;
        <b>false</b>
    }
}
</code></pre>



</details>


[move-book]: https://endless.dev/move/book/SUMMARY
