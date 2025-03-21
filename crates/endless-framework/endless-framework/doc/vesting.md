
<a id="0x1_vesting"></a>

# Module `0x1::vesting`


Simple vesting contract that allows specifying how much EDS coins should be vesting in each fixed-size period. The
vesting contract also comes with staking and allows shareholders to withdraw rewards anytime.

Vesting schedule is represented as a vector of distributions. For example, a vesting schedule of
[3/48, 3/48, 1/48] means that after the vesting starts:
1. The first and second periods will vest 3/48 of the total original grant.
2. The third period will vest 1/48.
3. All subsequent periods will also vest 1/48 (last distribution in the schedule) until the original grant runs out.

Shareholder flow:
1. Admin calls create_vesting_contract with a schedule of [3/48, 3/48, 1/48] with a vesting cliff of 1 year and
vesting period of 1 month.
2. After a month, a shareholder calls unlock_rewards to request rewards. They can also call vest() which would also
unlocks rewards but since the 1 year cliff has not passed (vesting has not started), vest() would not release any of
the original grant.
3. After the unlocked rewards become fully withdrawable (as it's subject to staking lockup), shareholders can call
distribute() to send all withdrawable funds to all shareholders based on the original grant's shares structure.
4. After 1 year and 1 month, the vesting schedule now starts. Shareholders call vest() to unlock vested coins. vest()
checks the schedule and unlocks 3/48 of the original grant in addition to any accumulated rewards since last
unlock_rewards(). Once the unlocked coins become withdrawable, shareholders can call distribute().
5. Assuming the shareholders forgot to call vest() for 2 months, when they call vest() again, they will unlock vested
tokens for the next period since last vest. This would be for the first month they missed. They can call vest() a
second time to unlock for the second month they missed.

Admin flow:
1. After creating the vesting contract, admin cannot change the vesting schedule.
2. Admin can call update_voter, update_operator, or reset_lockup at any time to update the underlying staking
contract.
3. Admin can also call update_beneficiary for any shareholder. This would send all distributions (rewards, vested
coins) of that shareholder to the beneficiary account. By defalt, if a beneficiary is not set, the distributions are
send directly to the shareholder account.
4. Admin can call terminate_vesting_contract to terminate the vesting. This would first finish any distribution but
will prevent any further rewards or vesting distributions from being created. Once the locked up stake becomes
withdrawable, admin can call admin_withdraw to withdraw all funds to the vesting contract's withdrawal address.


-  [Struct `VestingSchedule`](#0x1_vesting_VestingSchedule)
-  [Struct `StakingInfo`](#0x1_vesting_StakingInfo)
-  [Resource `VestingContract`](#0x1_vesting_VestingContract)
-  [Resource `VestingAccountManagement`](#0x1_vesting_VestingAccountManagement)
-  [Resource `AdminStore`](#0x1_vesting_AdminStore)
-  [Struct `CreateVestingContractEvent`](#0x1_vesting_CreateVestingContractEvent)
-  [Struct `UpdateOperatorEvent`](#0x1_vesting_UpdateOperatorEvent)
-  [Struct `UpdateVoterEvent`](#0x1_vesting_UpdateVoterEvent)
-  [Struct `ResetLockupEvent`](#0x1_vesting_ResetLockupEvent)
-  [Struct `SetBeneficiaryEvent`](#0x1_vesting_SetBeneficiaryEvent)
-  [Struct `UnlockRewardsEvent`](#0x1_vesting_UnlockRewardsEvent)
-  [Struct `VestEvent`](#0x1_vesting_VestEvent)
-  [Struct `DistributeEvent`](#0x1_vesting_DistributeEvent)
-  [Struct `TerminateEvent`](#0x1_vesting_TerminateEvent)
-  [Struct `AdminWithdrawEvent`](#0x1_vesting_AdminWithdrawEvent)
-  [Constants](#@Constants_0)
-  [Function `stake_pool_address`](#0x1_vesting_stake_pool_address)
-  [Function `vesting_start_secs`](#0x1_vesting_vesting_start_secs)
-  [Function `period_duration_secs`](#0x1_vesting_period_duration_secs)
-  [Function `remaining_grant`](#0x1_vesting_remaining_grant)
-  [Function `beneficiary`](#0x1_vesting_beneficiary)
-  [Function `operator_commission_percentage`](#0x1_vesting_operator_commission_percentage)
-  [Function `vesting_contracts`](#0x1_vesting_vesting_contracts)
-  [Function `operator`](#0x1_vesting_operator)
-  [Function `voter`](#0x1_vesting_voter)
-  [Function `vesting_schedule`](#0x1_vesting_vesting_schedule)
-  [Function `total_accumulated_rewards`](#0x1_vesting_total_accumulated_rewards)
-  [Function `accumulated_rewards`](#0x1_vesting_accumulated_rewards)
-  [Function `shareholders`](#0x1_vesting_shareholders)
-  [Function `shareholder`](#0x1_vesting_shareholder)
-  [Function `create_vesting_schedule`](#0x1_vesting_create_vesting_schedule)
-  [Function `create_vesting_contract`](#0x1_vesting_create_vesting_contract)
-  [Function `unlock_rewards`](#0x1_vesting_unlock_rewards)
-  [Function `unlock_rewards_many`](#0x1_vesting_unlock_rewards_many)
-  [Function `vest`](#0x1_vesting_vest)
-  [Function `vest_many`](#0x1_vesting_vest_many)
-  [Function `distribute`](#0x1_vesting_distribute)
-  [Function `distribute_many`](#0x1_vesting_distribute_many)
-  [Function `terminate_vesting_contract`](#0x1_vesting_terminate_vesting_contract)
-  [Function `admin_withdraw`](#0x1_vesting_admin_withdraw)
-  [Function `update_operator`](#0x1_vesting_update_operator)
-  [Function `update_operator_with_same_commission`](#0x1_vesting_update_operator_with_same_commission)
-  [Function `update_commission_percentage`](#0x1_vesting_update_commission_percentage)
-  [Function `update_voter`](#0x1_vesting_update_voter)
-  [Function `reset_lockup`](#0x1_vesting_reset_lockup)
-  [Function `set_beneficiary`](#0x1_vesting_set_beneficiary)
-  [Function `reset_beneficiary`](#0x1_vesting_reset_beneficiary)
-  [Function `set_management_role`](#0x1_vesting_set_management_role)
-  [Function `set_beneficiary_resetter`](#0x1_vesting_set_beneficiary_resetter)
-  [Function `set_beneficiary_for_operator`](#0x1_vesting_set_beneficiary_for_operator)
-  [Function `get_role_holder`](#0x1_vesting_get_role_holder)
-  [Function `get_vesting_account_signer`](#0x1_vesting_get_vesting_account_signer)
-  [Function `get_vesting_account_signer_internal`](#0x1_vesting_get_vesting_account_signer_internal)
-  [Function `create_vesting_contract_account`](#0x1_vesting_create_vesting_contract_account)
-  [Function `verify_admin`](#0x1_vesting_verify_admin)
-  [Function `assert_vesting_contract_exists`](#0x1_vesting_assert_vesting_contract_exists)
-  [Function `assert_active_vesting_contract`](#0x1_vesting_assert_active_vesting_contract)
-  [Function `unlock_stake`](#0x1_vesting_unlock_stake)
-  [Function `withdraw_stake`](#0x1_vesting_withdraw_stake)
-  [Function `get_beneficiary`](#0x1_vesting_get_beneficiary)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="endless_account.md#0x1_endless_account">0x1::endless_account</a>;
<b>use</b> <a href="endless_coin.md#0x1_endless_coin">0x1::endless_coin</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="../../endless-stdlib/doc/fixed_point64.md#0x1_fixed_point64">0x1::fixed_point64</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="../../endless-stdlib/doc/math128.md#0x1_math128">0x1::math128</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128">0x1::pool_u128</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="stake.md#0x1_stake">0x1::stake</a>;
<b>use</b> <a href="staking_contract.md#0x1_staking_contract">0x1::staking_contract</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="system_addresses.md#0x1_system_addresses">0x1::system_addresses</a>;
<b>use</b> <a href="timestamp.md#0x1_timestamp">0x1::timestamp</a>;
<b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_vesting_VestingSchedule"></a>

## Struct `VestingSchedule`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vesting_VestingSchedule">VestingSchedule</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>schedule: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../endless-stdlib/doc/fixed_point64.md#0x1_fixed_point64_FixedPoint64">fixed_point64::FixedPoint64</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>start_timestamp_secs: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>period_duration: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>last_vested_period: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_StakingInfo"></a>

## Struct `StakingInfo`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vesting_StakingInfo">StakingInfo</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>operator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_percentage: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_VestingContract"></a>

## Resource `VestingContract`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>state: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>grant_pool: <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_Pool">pool_u128::Pool</a></code>
</dt>
<dd>

</dd>
<dt>
<code>beneficiaries: <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_schedule: <a href="vesting.md#0x1_vesting_VestingSchedule">vesting::VestingSchedule</a></code>
</dt>
<dd>

</dd>
<dt>
<code>withdrawal_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking: <a href="vesting.md#0x1_vesting_StakingInfo">vesting::StakingInfo</a></code>
</dt>
<dd>

</dd>
<dt>
<code>remaining_grant: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>signer_cap: <a href="account.md#0x1_account_SignerCapability">account::SignerCapability</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_VestingAccountManagement"></a>

## Resource `VestingAccountManagement`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>roles: <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, <b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_AdminStore"></a>

## Resource `AdminStore`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vesting_contracts: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>nonce: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_CreateVestingContractEvent"></a>

## Struct `CreateVestingContractEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_CreateVestingContractEvent">CreateVestingContractEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>operator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>grant_amount: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>withdrawal_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking_pool_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_percentage: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_UpdateOperatorEvent"></a>

## Struct `UpdateOperatorEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_UpdateOperatorEvent">UpdateOperatorEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking_pool_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>old_operator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_operator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_percentage: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_UpdateVoterEvent"></a>

## Struct `UpdateVoterEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_UpdateVoterEvent">UpdateVoterEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking_pool_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>old_voter: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_voter: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_ResetLockupEvent"></a>

## Struct `ResetLockupEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_ResetLockupEvent">ResetLockupEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking_pool_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_lockup_expiration_secs: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_SetBeneficiaryEvent"></a>

## Struct `SetBeneficiaryEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_SetBeneficiaryEvent">SetBeneficiaryEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>shareholder: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>old_beneficiary: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>new_beneficiary: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_UnlockRewardsEvent"></a>

## Struct `UnlockRewardsEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_UnlockRewardsEvent">UnlockRewardsEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking_pool_address: <b>address</b></code>
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

<a id="0x1_vesting_VestEvent"></a>

## Struct `VestEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_VestEvent">VestEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>staking_pool_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>period_vested: u64</code>
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

<a id="0x1_vesting_DistributeEvent"></a>

## Struct `DistributeEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_DistributeEvent">DistributeEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
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

<a id="0x1_vesting_TerminateEvent"></a>

## Struct `TerminateEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_TerminateEvent">TerminateEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vesting_AdminWithdrawEvent"></a>

## Struct `AdminWithdrawEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vesting_AdminWithdrawEvent">AdminWithdrawEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_contract_address: <b>address</b></code>
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

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vesting_EEMPTY_VESTING_SCHEDULE"></a>

Vesting schedule cannot be empty.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EEMPTY_VESTING_SCHEDULE">EEMPTY_VESTING_SCHEDULE</a>: u64 = 2;
</code></pre>



<a id="0x1_vesting_EINVALID_WITHDRAWAL_ADDRESS"></a>

Withdrawal address is invalid.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EINVALID_WITHDRAWAL_ADDRESS">EINVALID_WITHDRAWAL_ADDRESS</a>: u64 = 1;
</code></pre>



<a id="0x1_vesting_ENOT_ADMIN"></a>

The signer is not the admin of the vesting contract.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_ENOT_ADMIN">ENOT_ADMIN</a>: u64 = 7;
</code></pre>



<a id="0x1_vesting_ENO_SHAREHOLDERS"></a>

Shareholders list cannot be empty.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_ENO_SHAREHOLDERS">ENO_SHAREHOLDERS</a>: u64 = 4;
</code></pre>



<a id="0x1_vesting_EPENDING_STAKE_FOUND"></a>

Cannot terminate the vesting contract with pending active stake. Need to wait until next epoch.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EPENDING_STAKE_FOUND">EPENDING_STAKE_FOUND</a>: u64 = 11;
</code></pre>



<a id="0x1_vesting_EPERMISSION_DENIED"></a>

Account is not admin or does not have the required role to take this action.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EPERMISSION_DENIED">EPERMISSION_DENIED</a>: u64 = 15;
</code></pre>



<a id="0x1_vesting_EROLE_NOT_FOUND"></a>

The vesting account has no such management role.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EROLE_NOT_FOUND">EROLE_NOT_FOUND</a>: u64 = 14;
</code></pre>



<a id="0x1_vesting_ESHARES_LENGTH_MISMATCH"></a>

The length of shareholders and shares lists don't match.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_ESHARES_LENGTH_MISMATCH">ESHARES_LENGTH_MISMATCH</a>: u64 = 5;
</code></pre>



<a id="0x1_vesting_EVEC_EMPTY_FOR_MANY_FUNCTION"></a>

Zero items were provided to a *_many function.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EVEC_EMPTY_FOR_MANY_FUNCTION">EVEC_EMPTY_FOR_MANY_FUNCTION</a>: u64 = 16;
</code></pre>



<a id="0x1_vesting_EVESTING_ACCOUNT_HAS_NO_ROLES"></a>

Vesting account has no other management roles beside admin.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EVESTING_ACCOUNT_HAS_NO_ROLES">EVESTING_ACCOUNT_HAS_NO_ROLES</a>: u64 = 13;
</code></pre>



<a id="0x1_vesting_EVESTING_CONTRACT_NOT_ACTIVE"></a>

Vesting contract needs to be in active state.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EVESTING_CONTRACT_NOT_ACTIVE">EVESTING_CONTRACT_NOT_ACTIVE</a>: u64 = 8;
</code></pre>



<a id="0x1_vesting_EVESTING_CONTRACT_NOT_FOUND"></a>

No vesting contract found at provided address.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EVESTING_CONTRACT_NOT_FOUND">EVESTING_CONTRACT_NOT_FOUND</a>: u64 = 10;
</code></pre>



<a id="0x1_vesting_EVESTING_CONTRACT_STILL_ACTIVE"></a>

Admin can only withdraw from an inactive (paused or terminated) vesting contract.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EVESTING_CONTRACT_STILL_ACTIVE">EVESTING_CONTRACT_STILL_ACTIVE</a>: u64 = 9;
</code></pre>



<a id="0x1_vesting_EVESTING_START_TOO_SOON"></a>

Vesting cannot start before or at the current block timestamp. Has to be in the future.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EVESTING_START_TOO_SOON">EVESTING_START_TOO_SOON</a>: u64 = 6;
</code></pre>



<a id="0x1_vesting_EZERO_GRANT"></a>

Grant amount cannot be 0.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EZERO_GRANT">EZERO_GRANT</a>: u64 = 12;
</code></pre>



<a id="0x1_vesting_EZERO_VESTING_SCHEDULE_PERIOD"></a>

Vesting period cannot be 0.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_EZERO_VESTING_SCHEDULE_PERIOD">EZERO_VESTING_SCHEDULE_PERIOD</a>: u64 = 3;
</code></pre>



<a id="0x1_vesting_MAXIMUM_SHAREHOLDERS"></a>

Maximum number of shareholders a vesting pool can support.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_MAXIMUM_SHAREHOLDERS">MAXIMUM_SHAREHOLDERS</a>: u64 = 30;
</code></pre>



<a id="0x1_vesting_ROLE_BENEFICIARY_RESETTER"></a>

Roles that can manage certain aspects of the vesting account beyond the main admin.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_ROLE_BENEFICIARY_RESETTER">ROLE_BENEFICIARY_RESETTER</a>: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [82, 79, 76, 69, 95, 66, 69, 78, 69, 70, 73, 67, 73, 65, 82, 89, 95, 82, 69, 83, 69, 84, 84, 69, 82];
</code></pre>



<a id="0x1_vesting_VESTING_POOL_ACTIVE"></a>

Vesting contract states.
Vesting contract is active and distributions can be made.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_VESTING_POOL_ACTIVE">VESTING_POOL_ACTIVE</a>: u64 = 1;
</code></pre>



<a id="0x1_vesting_VESTING_POOL_SALT"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_VESTING_POOL_SALT">VESTING_POOL_SALT</a>: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [101, 110, 100, 108, 101, 115, 115, 95, 102, 114, 97, 109, 101, 119, 111, 114, 107, 58, 58, 118, 101, 115, 116, 105, 110, 103];
</code></pre>



<a id="0x1_vesting_VESTING_POOL_TERMINATED"></a>

Vesting contract has been terminated and all funds have been released back to the withdrawal address.


<pre><code><b>const</b> <a href="vesting.md#0x1_vesting_VESTING_POOL_TERMINATED">VESTING_POOL_TERMINATED</a>: u64 = 2;
</code></pre>



<a id="0x1_vesting_stake_pool_address"></a>

## Function `stake_pool_address`

Return the address of the underlying stake pool (separate resource account) of the vesting contract.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_stake_pool_address">stake_pool_address</a>(vesting_contract_address: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_stake_pool_address">stake_pool_address</a>(vesting_contract_address: <b>address</b>): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).staking.pool_address
}
</code></pre>



</details>

<a id="0x1_vesting_vesting_start_secs"></a>

## Function `vesting_start_secs`

Return the vesting start timestamp (in seconds) of the vesting contract.
Vesting will start at this time, and once a full period has passed, the first vest will become unlocked.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_vesting_start_secs">vesting_start_secs</a>(vesting_contract_address: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_vesting_start_secs">vesting_start_secs</a>(vesting_contract_address: <b>address</b>): u64 <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).vesting_schedule.start_timestamp_secs
}
</code></pre>



</details>

<a id="0x1_vesting_period_duration_secs"></a>

## Function `period_duration_secs`

Return the duration of one vesting period (in seconds).
Each vest is released after one full period has started, starting from the specified start_timestamp_secs.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_period_duration_secs">period_duration_secs</a>(vesting_contract_address: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_period_duration_secs">period_duration_secs</a>(vesting_contract_address: <b>address</b>): u64 <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).vesting_schedule.period_duration
}
</code></pre>



</details>

<a id="0x1_vesting_remaining_grant"></a>

## Function `remaining_grant`

Return the remaining grant, consisting of unvested coins that have not been distributed to shareholders.
Prior to start_timestamp_secs, the remaining grant will always be equal to the original grant.
Once vesting has started, and vested tokens are distributed, the remaining grant will decrease over time,
according to the vesting schedule.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_remaining_grant">remaining_grant</a>(vesting_contract_address: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_remaining_grant">remaining_grant</a>(vesting_contract_address: <b>address</b>): u128 <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).remaining_grant
}
</code></pre>



</details>

<a id="0x1_vesting_beneficiary"></a>

## Function `beneficiary`

Return the beneficiary account of the specified shareholder in a vesting contract.
This is the same as the shareholder address by default and only different if it's been explicitly set.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_beneficiary">beneficiary</a>(vesting_contract_address: <b>address</b>, shareholder: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_beneficiary">beneficiary</a>(vesting_contract_address: <b>address</b>, shareholder: <b>address</b>): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <a href="vesting.md#0x1_vesting_get_beneficiary">get_beneficiary</a>(<b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address), shareholder)
}
</code></pre>



</details>

<a id="0x1_vesting_operator_commission_percentage"></a>

## Function `operator_commission_percentage`

Return the percentage of accumulated rewards that is paid to the operator as commission.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_operator_commission_percentage">operator_commission_percentage</a>(vesting_contract_address: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_operator_commission_percentage">operator_commission_percentage</a>(vesting_contract_address: <b>address</b>): u64 <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).staking.commission_percentage
}
</code></pre>



</details>

<a id="0x1_vesting_vesting_contracts"></a>

## Function `vesting_contracts`

Return all the vesting contracts a given address is an admin of.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_vesting_contracts">vesting_contracts</a>(admin: <b>address</b>): <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_vesting_contracts">vesting_contracts</a>(admin: <b>address</b>): <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; <b>acquires</b> <a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a> {
    <b>if</b> (!<b>exists</b>&lt;<a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a>&gt;(admin)) {
        <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<b>address</b>&gt;()
    } <b>else</b> {
        <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a>&gt;(admin).vesting_contracts
    }
}
</code></pre>



</details>

<a id="0x1_vesting_operator"></a>

## Function `operator`

Return the operator who runs the validator for the vesting contract.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_operator">operator</a>(vesting_contract_address: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_operator">operator</a>(vesting_contract_address: <b>address</b>): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).staking.operator
}
</code></pre>



</details>

<a id="0x1_vesting_voter"></a>

## Function `voter`

Return the voter who will be voting on on-chain governance proposals on behalf of the vesting contract's stake
pool.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_voter">voter</a>(vesting_contract_address: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_voter">voter</a>(vesting_contract_address: <b>address</b>): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).staking.voter
}
</code></pre>



</details>

<a id="0x1_vesting_vesting_schedule"></a>

## Function `vesting_schedule`

Return the vesting contract's vesting schedule. The core schedule is represented as a list of u64-based
fractions, where the rightmmost 32 bits can be divided by 2^32 to get the fraction, and anything else is the
whole number.

For example 3/48, or 0.0625, will be represented as 268435456. The fractional portion would be
268435456 / 2^32 = 0.0625. Since there are fewer than 32 bits, the whole number portion is effectively 0.
So 268435456 = 0.0625.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_vesting_schedule">vesting_schedule</a>(vesting_contract_address: <b>address</b>): <a href="vesting.md#0x1_vesting_VestingSchedule">vesting::VestingSchedule</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_vesting_schedule">vesting_schedule</a>(vesting_contract_address: <b>address</b>): <a href="vesting.md#0x1_vesting_VestingSchedule">VestingSchedule</a> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(vesting_contract_address);
    <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address).vesting_schedule
}
</code></pre>



</details>

<a id="0x1_vesting_total_accumulated_rewards"></a>

## Function `total_accumulated_rewards`

Return the total accumulated rewards that have not been distributed to shareholders of the vesting contract.
This excludes any unpaid commission that the operator has not collected.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_total_accumulated_rewards">total_accumulated_rewards</a>(vesting_contract_address: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_total_accumulated_rewards">total_accumulated_rewards</a>(vesting_contract_address: <b>address</b>): u128 <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(vesting_contract_address);

    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address);
    <b>let</b> (total_active_stake, _, commission_amount) =
        <a href="staking_contract.md#0x1_staking_contract_staking_contract_amounts">staking_contract::staking_contract_amounts</a>(vesting_contract_address, vesting_contract.staking.operator);
    total_active_stake - vesting_contract.remaining_grant - commission_amount
}
</code></pre>



</details>

<a id="0x1_vesting_accumulated_rewards"></a>

## Function `accumulated_rewards`

Return the accumulated rewards that have not been distributed to the provided shareholder. Caller can also pass
the beneficiary address instead of shareholder address.

This errors out if the vesting contract with the provided address doesn't exist.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_accumulated_rewards">accumulated_rewards</a>(vesting_contract_address: <b>address</b>, shareholder_or_beneficiary: <b>address</b>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_accumulated_rewards">accumulated_rewards</a>(
    vesting_contract_address: <b>address</b>, shareholder_or_beneficiary: <b>address</b>): u128 <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(vesting_contract_address);

    <b>let</b> total_accumulated_rewards = <a href="vesting.md#0x1_vesting_total_accumulated_rewards">total_accumulated_rewards</a>(vesting_contract_address);
    <b>let</b> shareholder = <a href="vesting.md#0x1_vesting_shareholder">shareholder</a>(vesting_contract_address, shareholder_or_beneficiary);
    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address);
    <b>let</b> shares = <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_shares">pool_u128::shares</a>(&vesting_contract.grant_pool, shareholder);
    <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_shares_to_amount_with_total_coins">pool_u128::shares_to_amount_with_total_coins</a>(&vesting_contract.grant_pool, shares, total_accumulated_rewards)
}
</code></pre>



</details>

<a id="0x1_vesting_shareholders"></a>

## Function `shareholders`

Return the list of all shareholders in the vesting contract.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_shareholders">shareholders</a>(vesting_contract_address: <b>address</b>): <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_shareholders">shareholders</a>(vesting_contract_address: <b>address</b>): <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(vesting_contract_address);

    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address);
    <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_shareholders">pool_u128::shareholders</a>(&vesting_contract.grant_pool)
}
</code></pre>



</details>

<a id="0x1_vesting_shareholder"></a>

## Function `shareholder`

Return the shareholder address given the beneficiary address in a given vesting contract. If there are multiple
shareholders with the same beneficiary address, only the first shareholder is returned. If the given beneficiary
address is actually a shareholder address, just return the address back.

This returns 0x0 if no shareholder is found for the given beneficiary / the address is not a shareholder itself.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_shareholder">shareholder</a>(vesting_contract_address: <b>address</b>, shareholder_or_beneficiary: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_shareholder">shareholder</a>(vesting_contract_address: <b>address</b>, shareholder_or_beneficiary: <b>address</b>): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(vesting_contract_address);

    <b>let</b> shareholders = &<a href="vesting.md#0x1_vesting_shareholders">shareholders</a>(vesting_contract_address);
    <b>if</b> (<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_contains">vector::contains</a>(shareholders, &shareholder_or_beneficiary)) {
        <b>return</b> shareholder_or_beneficiary
    };
    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(vesting_contract_address);
    <b>let</b> result = @0x0;
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_any">vector::any</a>(shareholders, |shareholder| {
        <b>if</b> (shareholder_or_beneficiary == <a href="vesting.md#0x1_vesting_get_beneficiary">get_beneficiary</a>(vesting_contract, *shareholder)) {
            result = *shareholder;
            <b>true</b>
        } <b>else</b> {
            <b>false</b>
        }
    });

    result
}
</code></pre>



</details>

<a id="0x1_vesting_create_vesting_schedule"></a>

## Function `create_vesting_schedule`

Create a vesting schedule with the given schedule of distributions, a vesting start time and period duration.


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_create_vesting_schedule">create_vesting_schedule</a>(schedule: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../endless-stdlib/doc/fixed_point64.md#0x1_fixed_point64_FixedPoint64">fixed_point64::FixedPoint64</a>&gt;, start_timestamp_secs: u64, period_duration: u64): <a href="vesting.md#0x1_vesting_VestingSchedule">vesting::VestingSchedule</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_create_vesting_schedule">create_vesting_schedule</a>(
    schedule: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FixedPoint64&gt;,
    start_timestamp_secs: u64,
    period_duration: u64,
): <a href="vesting.md#0x1_vesting_VestingSchedule">VestingSchedule</a> {
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&schedule) &gt; 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EEMPTY_VESTING_SCHEDULE">EEMPTY_VESTING_SCHEDULE</a>));
    <b>assert</b>!(period_duration &gt; 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EZERO_VESTING_SCHEDULE_PERIOD">EZERO_VESTING_SCHEDULE_PERIOD</a>));
    <b>assert</b>!(
        start_timestamp_secs &gt;= <a href="timestamp.md#0x1_timestamp_now_seconds">timestamp::now_seconds</a>(),
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EVESTING_START_TOO_SOON">EVESTING_START_TOO_SOON</a>),
    );

    <a href="vesting.md#0x1_vesting_VestingSchedule">VestingSchedule</a> {
        schedule,
        start_timestamp_secs,
        period_duration,
        last_vested_period: 0,
    }
}
</code></pre>



</details>

<a id="0x1_vesting_create_vesting_contract"></a>

## Function `create_vesting_contract`

Create a vesting contract with a given configurations.


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_create_vesting_contract">create_vesting_contract</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, shareholders: &<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, buy_ins: <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<b>address</b>, u128&gt;, all_buyin_coins: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, vesting_schedule: <a href="vesting.md#0x1_vesting_VestingSchedule">vesting::VestingSchedule</a>, withdrawal_address: <b>address</b>, operator: <b>address</b>, voter: <b>address</b>, commission_percentage: u64, contract_creation_seed: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_create_vesting_contract">create_vesting_contract</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    shareholders: &<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
    buy_ins: SimpleMap&lt;<b>address</b>, u128&gt;,
    all_buyin_coins: FungibleAsset,
    vesting_schedule: <a href="vesting.md#0x1_vesting_VestingSchedule">VestingSchedule</a>,
    withdrawal_address: <b>address</b>,
    operator: <b>address</b>,
    voter: <b>address</b>,
    commission_percentage: u64,
    // Optional seed used when creating the staking contract <a href="account.md#0x1_account">account</a>.
    contract_creation_seed: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a> {
    <b>assert</b>!(
        !<a href="system_addresses.md#0x1_system_addresses_is_reserved_address">system_addresses::is_reserved_address</a>(withdrawal_address),
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EINVALID_WITHDRAWAL_ADDRESS">EINVALID_WITHDRAWAL_ADDRESS</a>),
    );
    assert_account_is_registered_for_eds(withdrawal_address);
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(shareholders) &gt; 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_ENO_SHAREHOLDERS">ENO_SHAREHOLDERS</a>));
    <b>assert</b>!(
        <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_length">simple_map::length</a>(&buy_ins) == <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(shareholders),
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_ESHARES_LENGTH_MISMATCH">ESHARES_LENGTH_MISMATCH</a>),
    );

    // Create a coins pool <b>to</b> track shareholders and shares of the grant.
    <b>let</b> grant = <a href="endless_coin.md#0x1_endless_coin_zero">endless_coin::zero</a>();
    <b>let</b> grant_amount = 0;
    <b>let</b> grant_pool = <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_create">pool_u128::create</a>(<a href="vesting.md#0x1_vesting_MAXIMUM_SHAREHOLDERS">MAXIMUM_SHAREHOLDERS</a>);
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(shareholders, |shareholder| {
        <b>let</b> shareholder: <b>address</b> = *shareholder;
        <b>let</b> (_, buy_in_amount) = <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_remove">simple_map::remove</a>(&<b>mut</b> buy_ins, &shareholder);
        <a href="fungible_asset.md#0x1_fungible_asset_merge">fungible_asset::merge</a>(&<b>mut</b> grant, <a href="fungible_asset.md#0x1_fungible_asset_extract">fungible_asset::extract</a>(&<b>mut</b> all_buyin_coins, (buy_in_amount <b>as</b> u128)));
        <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_buy_in">pool_u128::buy_in</a>(
            &<b>mut</b> grant_pool,
            shareholder,
            buy_in_amount,
        );
        grant_amount = grant_amount + buy_in_amount;
    });
    <b>assert</b>!(grant_amount &gt; 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EZERO_GRANT">EZERO_GRANT</a>));
    <b>assert</b>!(<a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&all_buyin_coins) == 0, 0);
    <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">fungible_asset::destroy_zero</a>(all_buyin_coins);

    <b>assert</b>!(grant_amount &gt; 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EZERO_GRANT">EZERO_GRANT</a>));

    // If this is the first time this admin <a href="account.md#0x1_account">account</a> <b>has</b> created a <a href="vesting.md#0x1_vesting">vesting</a> contract, initialize the admin store.
    <b>let</b> admin_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(admin);
    <b>if</b> (!<b>exists</b>&lt;<a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a>&gt;(admin_address)) {
        <b>move_to</b>(admin, <a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a> {
            vesting_contracts: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<b>address</b>&gt;(),
            nonce: 0,
        });
    };

    // Initialize the <a href="vesting.md#0x1_vesting">vesting</a> contract in a new resource <a href="account.md#0x1_account">account</a>. This allows the same admin <b>to</b> create multiple
    // pools.
    <b>let</b> (contract_signer, contract_signer_cap) = <a href="vesting.md#0x1_vesting_create_vesting_contract_account">create_vesting_contract_account</a>(admin, contract_creation_seed);
    <b>let</b> pool_address = <a href="staking_contract.md#0x1_staking_contract_create_staking_contract_with_coins">staking_contract::create_staking_contract_with_coins</a>(
        &contract_signer, operator, voter, grant, commission_percentage, contract_creation_seed);

    // Add the newly created <a href="vesting.md#0x1_vesting">vesting</a> contract's <b>address</b> <b>to</b> the admin store.
    <b>let</b> contract_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&contract_signer);
    <b>let</b> admin_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a>&gt;(admin_address);
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> admin_store.vesting_contracts, contract_address);
    emit(
        <a href="vesting.md#0x1_vesting_CreateVestingContractEvent">CreateVestingContractEvent</a> {
            operator,
            voter,
            withdrawal_address,
            grant_amount,
            vesting_contract_address: contract_address,
            staking_pool_address: pool_address,
            commission_percentage,
        },
    );

    <b>move_to</b>(&contract_signer, <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
        state: <a href="vesting.md#0x1_vesting_VESTING_POOL_ACTIVE">VESTING_POOL_ACTIVE</a>,
        admin: admin_address,
        grant_pool,
        beneficiaries: <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;<b>address</b>, <b>address</b>&gt;(),
        vesting_schedule,
        withdrawal_address,
        staking: <a href="vesting.md#0x1_vesting_StakingInfo">StakingInfo</a> { pool_address, operator, voter, commission_percentage },
        remaining_grant: grant_amount,
        signer_cap: contract_signer_cap,
    });

    <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_destroy_empty">simple_map::destroy_empty</a>(buy_ins);
    contract_address
}
</code></pre>



</details>

<a id="0x1_vesting_unlock_rewards"></a>

## Function `unlock_rewards`

Unlock any accumulated rewards.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_unlock_rewards">unlock_rewards</a>(contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_unlock_rewards">unlock_rewards</a>(contract_address: <b>address</b>) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> accumulated_rewards = <a href="vesting.md#0x1_vesting_total_accumulated_rewards">total_accumulated_rewards</a>(contract_address);
    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_unlock_stake">unlock_stake</a>(vesting_contract, accumulated_rewards);
}
</code></pre>



</details>

<a id="0x1_vesting_unlock_rewards_many"></a>

## Function `unlock_rewards_many`

Call <code>unlock_rewards</code> for many vesting contracts.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_unlock_rewards_many">unlock_rewards_many</a>(contract_addresses: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_unlock_rewards_many">unlock_rewards_many</a>(contract_addresses: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> len = <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&contract_addresses);

    <b>assert</b>!(len != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EVEC_EMPTY_FOR_MANY_FUNCTION">EVEC_EMPTY_FOR_MANY_FUNCTION</a>));

    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(&contract_addresses, |contract_address| {
        <b>let</b> contract_address: <b>address</b> = *contract_address;
        <a href="vesting.md#0x1_vesting_unlock_rewards">unlock_rewards</a>(contract_address);
    });
}
</code></pre>



</details>

<a id="0x1_vesting_vest"></a>

## Function `vest`

Unlock any vested portion of the grant.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_vest">vest</a>(contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_vest">vest</a>(contract_address: <b>address</b>) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    // Unlock all rewards first, <b>if</b> <a href="../../endless-stdlib/doc/any.md#0x1_any">any</a>.
    <a href="vesting.md#0x1_vesting_unlock_rewards">unlock_rewards</a>(contract_address);

    // Unlock the vested amount. This amount will become withdrawable when the underlying <a href="stake.md#0x1_stake">stake</a> pool's lockup
    // expires.
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    // Short-circuit <b>if</b> <a href="vesting.md#0x1_vesting">vesting</a> hasn't started yet.
    <b>if</b> (vesting_contract.vesting_schedule.start_timestamp_secs &gt; <a href="timestamp.md#0x1_timestamp_now_seconds">timestamp::now_seconds</a>()) {
        <b>return</b>
    };

    // Check <b>if</b> the next vested period <b>has</b> already passed. If not, short-circuit since there's nothing <b>to</b> vest.
    <b>let</b> vesting_schedule = &<b>mut</b> vesting_contract.vesting_schedule;
    <b>let</b> last_vested_period = vesting_schedule.last_vested_period;
    <b>let</b> next_period_to_vest = last_vested_period + 1;
    <b>let</b> last_completed_period =
        (<a href="timestamp.md#0x1_timestamp_now_seconds">timestamp::now_seconds</a>() - vesting_schedule.start_timestamp_secs) / vesting_schedule.period_duration;
    <b>if</b> (last_completed_period &lt; next_period_to_vest) {
        <b>return</b>
    };

    // Calculate how much <b>has</b> vested, excluding rewards.
    // Index is 0-based <b>while</b> period is 1-based so we need <b>to</b> subtract 1.
    <b>let</b> schedule = &vesting_schedule.schedule;
    <b>let</b> schedule_index = next_period_to_vest - 1;
    <b>let</b> vesting_fraction = <b>if</b> (schedule_index &lt; <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(schedule)) {
        *<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(schedule, schedule_index)
    } <b>else</b> {
        // Last <a href="vesting.md#0x1_vesting">vesting</a> schedule fraction will repeat until the grant runs out.
        *<a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(schedule, <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(schedule) - 1)
    };
    <b>let</b> total_grant = <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_total_coins">pool_u128::total_coins</a>(&vesting_contract.grant_pool);
    <b>let</b> vested_amount = <a href="../../endless-stdlib/doc/fixed_point64.md#0x1_fixed_point64_multiply_u128">fixed_point64::multiply_u128</a>(total_grant, vesting_fraction);
    // Cap vested amount by the remaining grant amount so we don't try <b>to</b> distribute more than what's remaining.
    vested_amount = <b>min</b>(vested_amount, vesting_contract.remaining_grant);
    vesting_contract.remaining_grant = vesting_contract.remaining_grant - vested_amount;
    vesting_schedule.last_vested_period = next_period_to_vest;
    <a href="vesting.md#0x1_vesting_unlock_stake">unlock_stake</a>(vesting_contract, (vested_amount <b>as</b> u128));

    emit(
        <a href="vesting.md#0x1_vesting_VestEvent">VestEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            staking_pool_address: vesting_contract.staking.pool_address,
            period_vested: next_period_to_vest,
            amount: vested_amount,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_vest_many"></a>

## Function `vest_many`

Call <code>vest</code> for many vesting contracts.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_vest_many">vest_many</a>(contract_addresses: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_vest_many">vest_many</a>(contract_addresses: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> len = <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&contract_addresses);

    <b>assert</b>!(len != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EVEC_EMPTY_FOR_MANY_FUNCTION">EVEC_EMPTY_FOR_MANY_FUNCTION</a>));

    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(&contract_addresses, |contract_address| {
        <b>let</b> contract_address = *contract_address;
        <a href="vesting.md#0x1_vesting_vest">vest</a>(contract_address);
    });
}
</code></pre>



</details>

<a id="0x1_vesting_distribute"></a>

## Function `distribute`

Distribute any withdrawable stake from the stake pool.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_distribute">distribute</a>(contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_distribute">distribute</a>(contract_address: <b>address</b>) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(contract_address);

    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <b>let</b> coins = <a href="vesting.md#0x1_vesting_withdraw_stake">withdraw_stake</a>(vesting_contract, contract_address);
    <b>let</b> total_distribution_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&coins);
    <b>if</b> (total_distribution_amount == 0) {
        <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">fungible_asset::destroy_zero</a>(coins);
        <b>return</b>
    };

    // Distribute coins <b>to</b> all shareholders in the <a href="vesting.md#0x1_vesting">vesting</a> contract.
    <b>let</b> grant_pool = &vesting_contract.grant_pool;
    <b>let</b> shareholders = &<a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_shareholders">pool_u128::shareholders</a>(grant_pool);
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(shareholders, |shareholder| {
        <b>let</b> shareholder = *shareholder;
        <b>let</b> shares = <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_shares">pool_u128::shares</a>(grant_pool, shareholder);
        <b>let</b> amount = <a href="../../endless-stdlib/doc/pool_u128.md#0x1_pool_u128_shares_to_amount_with_total_coins">pool_u128::shares_to_amount_with_total_coins</a>(grant_pool, shares, total_distribution_amount);
        <b>let</b> share_of_coins = <a href="fungible_asset.md#0x1_fungible_asset_extract">fungible_asset::extract</a>(&<b>mut</b> coins, (amount <b>as</b> u128));
        <b>let</b> recipient_address = <a href="vesting.md#0x1_vesting_get_beneficiary">get_beneficiary</a>(vesting_contract, shareholder);
        <a href="endless_account.md#0x1_endless_account_deposit_coins">endless_account::deposit_coins</a>(recipient_address, share_of_coins);
    });

    // Send <a href="../../endless-stdlib/doc/any.md#0x1_any">any</a> remaining "dust" (leftover due <b>to</b> rounding <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error">error</a>) <b>to</b> the withdrawal <b>address</b>.
    <b>if</b> (<a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&coins) &gt; 0) {
        <a href="endless_account.md#0x1_endless_account_deposit_coins">endless_account::deposit_coins</a>(vesting_contract.withdrawal_address, coins);
    } <b>else</b> {
        <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">fungible_asset::destroy_zero</a>(coins);
    };

    emit(
        <a href="vesting.md#0x1_vesting_DistributeEvent">DistributeEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            amount: total_distribution_amount,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_distribute_many"></a>

## Function `distribute_many`

Call <code>distribute</code> for many vesting contracts.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_distribute_many">distribute_many</a>(contract_addresses: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_distribute_many">distribute_many</a>(contract_addresses: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> len = <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&contract_addresses);

    <b>assert</b>!(len != 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vesting_EVEC_EMPTY_FOR_MANY_FUNCTION">EVEC_EMPTY_FOR_MANY_FUNCTION</a>));

    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(&contract_addresses, |contract_address| {
        <b>let</b> contract_address = *contract_address;
        <a href="vesting.md#0x1_vesting_distribute">distribute</a>(contract_address);
    });
}
</code></pre>



</details>

<a id="0x1_vesting_terminate_vesting_contract"></a>

## Function `terminate_vesting_contract`

Terminate the vesting contract and send all funds back to the withdrawal address.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_terminate_vesting_contract">terminate_vesting_contract</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_terminate_vesting_contract">terminate_vesting_contract</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(contract_address);

    // Distribute all withdrawable coins, which should have been from previous rewards withdrawal or vest.
    <a href="vesting.md#0x1_vesting_distribute">distribute</a>(contract_address);

    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <b>let</b> (active_stake, _, pending_active_stake, _) = <a href="stake.md#0x1_stake_get_stake">stake::get_stake</a>(vesting_contract.staking.pool_address);
    <b>assert</b>!(pending_active_stake == 0, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vesting.md#0x1_vesting_EPENDING_STAKE_FOUND">EPENDING_STAKE_FOUND</a>));

    // Unlock all remaining active <a href="stake.md#0x1_stake">stake</a>.
    vesting_contract.state = <a href="vesting.md#0x1_vesting_VESTING_POOL_TERMINATED">VESTING_POOL_TERMINATED</a>;
    vesting_contract.remaining_grant = 0;
    <a href="vesting.md#0x1_vesting_unlock_stake">unlock_stake</a>(vesting_contract, active_stake);

    emit(
        <a href="vesting.md#0x1_vesting_TerminateEvent">TerminateEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_admin_withdraw"></a>

## Function `admin_withdraw`

Withdraw all funds to the preset vesting contract's withdrawal address. This can only be called if the contract
has already been terminated.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_admin_withdraw">admin_withdraw</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_admin_withdraw">admin_withdraw</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <b>assert</b>!(vesting_contract.state == <a href="vesting.md#0x1_vesting_VESTING_POOL_TERMINATED">VESTING_POOL_TERMINATED</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vesting.md#0x1_vesting_EVESTING_CONTRACT_STILL_ACTIVE">EVESTING_CONTRACT_STILL_ACTIVE</a>));

    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <b>let</b> coins = <a href="vesting.md#0x1_vesting_withdraw_stake">withdraw_stake</a>(vesting_contract, contract_address);
    <b>let</b> amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&coins);
    <b>if</b> (amount == 0) {
        <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">fungible_asset::destroy_zero</a>(coins);
        <b>return</b>
    };
    <a href="endless_account.md#0x1_endless_account_deposit_coins">endless_account::deposit_coins</a>(vesting_contract.withdrawal_address, coins);

    emit(
        <a href="vesting.md#0x1_vesting_AdminWithdrawEvent">AdminWithdrawEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            amount,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_update_operator"></a>

## Function `update_operator`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_operator">update_operator</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, new_operator: <b>address</b>, commission_percentage: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_operator">update_operator</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    new_operator: <b>address</b>,
    commission_percentage: u64,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
    <b>let</b> old_operator = vesting_contract.staking.operator;
    <a href="staking_contract.md#0x1_staking_contract_switch_operator">staking_contract::switch_operator</a>(contract_signer, old_operator, new_operator, commission_percentage);
    vesting_contract.staking.operator = new_operator;
    vesting_contract.staking.commission_percentage = commission_percentage;

    emit(
        <a href="vesting.md#0x1_vesting_UpdateOperatorEvent">UpdateOperatorEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            staking_pool_address: vesting_contract.staking.pool_address,
            old_operator,
            new_operator,
            commission_percentage,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_update_operator_with_same_commission"></a>

## Function `update_operator_with_same_commission`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_operator_with_same_commission">update_operator_with_same_commission</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, new_operator: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_operator_with_same_commission">update_operator_with_same_commission</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    new_operator: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> commission_percentage = <a href="vesting.md#0x1_vesting_operator_commission_percentage">operator_commission_percentage</a>(contract_address);
    <a href="vesting.md#0x1_vesting_update_operator">update_operator</a>(admin, contract_address, new_operator, commission_percentage);
}
</code></pre>



</details>

<a id="0x1_vesting_update_commission_percentage"></a>

## Function `update_commission_percentage`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_commission_percentage">update_commission_percentage</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, new_commission_percentage: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_commission_percentage">update_commission_percentage</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    new_commission_percentage: u64,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> operator = <a href="vesting.md#0x1_vesting_operator">operator</a>(contract_address);
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
    <a href="staking_contract.md#0x1_staking_contract_update_commision">staking_contract::update_commision</a>(contract_signer, operator, new_commission_percentage);
    vesting_contract.staking.commission_percentage = new_commission_percentage;
    // This function does not emit an <a href="event.md#0x1_event">event</a>. Instead, `staking_contract::update_commission_percentage`
    // <b>emits</b> the <a href="event.md#0x1_event">event</a> for this commission percentage <b>update</b>.
}
</code></pre>



</details>

<a id="0x1_vesting_update_voter"></a>

## Function `update_voter`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_voter">update_voter</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, new_voter: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_update_voter">update_voter</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    new_voter: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
    <b>let</b> old_voter = vesting_contract.staking.voter;
    <a href="staking_contract.md#0x1_staking_contract_update_voter">staking_contract::update_voter</a>(contract_signer, vesting_contract.staking.operator, new_voter);
    vesting_contract.staking.voter = new_voter;

    emit(
        <a href="vesting.md#0x1_vesting_UpdateVoterEvent">UpdateVoterEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            staking_pool_address: vesting_contract.staking.pool_address,
            old_voter,
            new_voter,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_reset_lockup"></a>

## Function `reset_lockup`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_reset_lockup">reset_lockup</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_reset_lockup">reset_lockup</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
    <a href="staking_contract.md#0x1_staking_contract_reset_lockup">staking_contract::reset_lockup</a>(contract_signer, vesting_contract.staking.operator);

    emit(
        <a href="vesting.md#0x1_vesting_ResetLockupEvent">ResetLockupEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            staking_pool_address: vesting_contract.staking.pool_address,
            new_lockup_expiration_secs: <a href="stake.md#0x1_stake_get_lockup_secs">stake::get_lockup_secs</a>(vesting_contract.staking.pool_address),
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_set_beneficiary"></a>

## Function `set_beneficiary`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_beneficiary">set_beneficiary</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, shareholder: <b>address</b>, new_beneficiary: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_beneficiary">set_beneficiary</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    shareholder: <b>address</b>,
    new_beneficiary: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    // Verify that the beneficiary <a href="account.md#0x1_account">account</a> is set up <b>to</b> receive EDS. This is a requirement so <a href="vesting.md#0x1_vesting_distribute">distribute</a>() wouldn't
    // fail and <a href="block.md#0x1_block">block</a> all other accounts from receiving EDS <b>if</b> one beneficiary is not registered.
    assert_account_is_registered_for_eds(new_beneficiary);

    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);

    <b>let</b> old_beneficiary = <a href="vesting.md#0x1_vesting_get_beneficiary">get_beneficiary</a>(vesting_contract, shareholder);
    <b>let</b> beneficiaries = &<b>mut</b> vesting_contract.beneficiaries;
    <b>if</b> (<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(beneficiaries, &shareholder)) {
        <b>let</b> beneficiary = <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_borrow_mut">simple_map::borrow_mut</a>(beneficiaries, &shareholder);
        *beneficiary = new_beneficiary;
    } <b>else</b> {
        <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_add">simple_map::add</a>(beneficiaries, shareholder, new_beneficiary);
    };

    emit(
        <a href="vesting.md#0x1_vesting_SetBeneficiaryEvent">SetBeneficiaryEvent</a> {
            admin: vesting_contract.admin,
            vesting_contract_address: contract_address,
            shareholder,
            old_beneficiary,
            new_beneficiary,
        },
    );
}
</code></pre>



</details>

<a id="0x1_vesting_reset_beneficiary"></a>

## Function `reset_beneficiary`

Remove the beneficiary for the given shareholder. All distributions will sent directly to the shareholder
account.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_reset_beneficiary">reset_beneficiary</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, shareholder: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_reset_beneficiary">reset_beneficiary</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    shareholder: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>, <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <b>let</b> addr = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>assert</b>!(
        addr == vesting_contract.admin ||
            addr == <a href="vesting.md#0x1_vesting_get_role_holder">get_role_holder</a>(contract_address, utf8(<a href="vesting.md#0x1_vesting_ROLE_BENEFICIARY_RESETTER">ROLE_BENEFICIARY_RESETTER</a>)),
        <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="vesting.md#0x1_vesting_EPERMISSION_DENIED">EPERMISSION_DENIED</a>),
    );

    <b>let</b> beneficiaries = &<b>mut</b> vesting_contract.beneficiaries;
    <b>if</b> (<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(beneficiaries, &shareholder)) {
        <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_remove">simple_map::remove</a>(beneficiaries, &shareholder);
    };
}
</code></pre>



</details>

<a id="0x1_vesting_set_management_role"></a>

## Function `set_management_role`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_management_role">set_management_role</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, role: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, role_holder: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_management_role">set_management_role</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    role: String,
    role_holder: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>, <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);

    <b>if</b> (!<b>exists</b>&lt;<a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>&gt;(contract_address)) {
        <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
        <b>move_to</b>(contract_signer, <a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a> {
            roles: <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;String, <b>address</b>&gt;(),
        })
    };
    <b>let</b> roles = &<b>mut</b> <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>&gt;(contract_address).roles;
    <b>if</b> (<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(roles, &role)) {
        *<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_borrow_mut">simple_map::borrow_mut</a>(roles, &role) = role_holder;
    } <b>else</b> {
        <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_add">simple_map::add</a>(roles, role, role_holder);
    };
}
</code></pre>



</details>

<a id="0x1_vesting_set_beneficiary_resetter"></a>

## Function `set_beneficiary_resetter`



<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_beneficiary_resetter">set_beneficiary_resetter</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>, beneficiary_resetter: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_beneficiary_resetter">set_beneficiary_resetter</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_address: <b>address</b>,
    beneficiary_resetter: <b>address</b>,
) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>, <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_set_management_role">set_management_role</a>(admin, contract_address, utf8(<a href="vesting.md#0x1_vesting_ROLE_BENEFICIARY_RESETTER">ROLE_BENEFICIARY_RESETTER</a>), beneficiary_resetter);
}
</code></pre>



</details>

<a id="0x1_vesting_set_beneficiary_for_operator"></a>

## Function `set_beneficiary_for_operator`

Set the beneficiary for the operator.


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_beneficiary_for_operator">set_beneficiary_for_operator</a>(operator: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_beneficiary: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vesting.md#0x1_vesting_set_beneficiary_for_operator">set_beneficiary_for_operator</a>(
    operator: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    new_beneficiary: <b>address</b>,
) {
    <a href="staking_contract.md#0x1_staking_contract_set_beneficiary_for_operator">staking_contract::set_beneficiary_for_operator</a>(operator, new_beneficiary);
}
</code></pre>



</details>

<a id="0x1_vesting_get_role_holder"></a>

## Function `get_role_holder`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_get_role_holder">get_role_holder</a>(contract_address: <b>address</b>, role: <a href="../../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_get_role_holder">get_role_holder</a>(contract_address: <b>address</b>, role: String): <b>address</b> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a> {
    <b>assert</b>!(<b>exists</b>&lt;<a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>&gt;(contract_address), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vesting_EVESTING_ACCOUNT_HAS_NO_ROLES">EVESTING_ACCOUNT_HAS_NO_ROLES</a>));
    <b>let</b> roles = &<b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingAccountManagement">VestingAccountManagement</a>&gt;(contract_address).roles;
    <b>assert</b>!(<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(roles, &role), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vesting_EROLE_NOT_FOUND">EROLE_NOT_FOUND</a>));
    *<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_borrow">simple_map::borrow</a>(roles, &role)
}
</code></pre>



</details>

<a id="0x1_vesting_get_vesting_account_signer"></a>

## Function `get_vesting_account_signer`

For emergency use in case the admin needs emergency control of vesting contract account.
This doesn't give the admin total power as the admin would still need to follow the rules set by
staking_contract and stake modules.


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_get_vesting_account_signer">get_vesting_account_signer</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>): <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vesting_get_vesting_account_signer">get_vesting_account_signer</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_address: <b>address</b>): <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a> <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <b>let</b> vesting_contract = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin, vesting_contract);
    <a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract)
}
</code></pre>



</details>

<a id="0x1_vesting_get_vesting_account_signer_internal"></a>

## Function `get_vesting_account_signer_internal`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">vesting::VestingContract</a>): <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>): <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a> {
    <a href="account.md#0x1_account_create_signer_with_capability">account::create_signer_with_capability</a>(&vesting_contract.signer_cap)
}
</code></pre>



</details>

<a id="0x1_vesting_create_vesting_contract_account"></a>

## Function `create_vesting_contract_account`

Create a salt for generating the resource accounts that will be holding the VestingContract.
This address should be deterministic for the same admin and vesting contract creation nonce.


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_create_vesting_contract_account">create_vesting_contract_account</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, contract_creation_seed: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="account.md#0x1_account_SignerCapability">account::SignerCapability</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_create_vesting_contract_account">create_vesting_contract_account</a>(
    admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    contract_creation_seed: <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
): (<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, SignerCapability) <b>acquires</b> <a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a> {
    <b>let</b> admin_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vesting_AdminStore">AdminStore</a>&gt;(<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(admin));
    <b>let</b> seed = <a href="../../endless-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(admin));
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, <a href="../../endless-stdlib/../move-stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&admin_store.nonce));
    admin_store.nonce = admin_store.nonce + 1;

    // Include a salt <b>to</b> avoid conflicts <b>with</b> <a href="../../endless-stdlib/doc/any.md#0x1_any">any</a> other modules out there that might also generate
    // deterministic resource accounts for the same admin <b>address</b> + nonce.
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, <a href="vesting.md#0x1_vesting_VESTING_POOL_SALT">VESTING_POOL_SALT</a>);
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, contract_creation_seed);

    <b>let</b> (account_signer, signer_cap) = <a href="account.md#0x1_account_create_resource_account">account::create_resource_account</a>(admin, seed);
    // Register the <a href="vesting.md#0x1_vesting">vesting</a> contract <a href="account.md#0x1_account">account</a> <b>to</b> receive EDS <b>as</b> it'll be sent <b>to</b> it when claiming unlocked <a href="stake.md#0x1_stake">stake</a> from
    // the underlying staking contract.
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">primary_fungible_store::ensure_primary_store_exists</a>(<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&account_signer), <a href="endless_coin.md#0x1_endless_coin_get_metadata">endless_coin::get_metadata</a>());

    (account_signer, signer_cap)
}
</code></pre>



</details>

<a id="0x1_vesting_verify_admin"></a>

## Function `verify_admin`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">vesting::VestingContract</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_verify_admin">verify_admin</a>(admin: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>) {
    <b>assert</b>!(<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(admin) == vesting_contract.admin, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_unauthenticated">error::unauthenticated</a>(<a href="vesting.md#0x1_vesting_ENOT_ADMIN">ENOT_ADMIN</a>));
}
</code></pre>



</details>

<a id="0x1_vesting_assert_vesting_contract_exists"></a>

## Function `assert_vesting_contract_exists`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(contract_address: <b>address</b>) {
    <b>assert</b>!(<b>exists</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address), <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vesting_EVESTING_CONTRACT_NOT_FOUND">EVESTING_CONTRACT_NOT_FOUND</a>));
}
</code></pre>



</details>

<a id="0x1_vesting_assert_active_vesting_contract"></a>

## Function `assert_active_vesting_contract`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(contract_address: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_assert_active_vesting_contract">assert_active_vesting_contract</a>(contract_address: <b>address</b>) <b>acquires</b> <a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a> {
    <a href="vesting.md#0x1_vesting_assert_vesting_contract_exists">assert_vesting_contract_exists</a>(contract_address);
    <b>let</b> vesting_contract = <b>borrow_global</b>&lt;<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>&gt;(contract_address);
    <b>assert</b>!(vesting_contract.state == <a href="vesting.md#0x1_vesting_VESTING_POOL_ACTIVE">VESTING_POOL_ACTIVE</a>, <a href="../../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vesting.md#0x1_vesting_EVESTING_CONTRACT_NOT_ACTIVE">EVESTING_CONTRACT_NOT_ACTIVE</a>));
}
</code></pre>



</details>

<a id="0x1_vesting_unlock_stake"></a>

## Function `unlock_stake`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_unlock_stake">unlock_stake</a>(vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">vesting::VestingContract</a>, amount: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_unlock_stake">unlock_stake</a>(vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>, amount: u128) {
    <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
    <a href="staking_contract.md#0x1_staking_contract_unlock_stake">staking_contract::unlock_stake</a>(contract_signer, vesting_contract.staking.operator, amount);
}
</code></pre>



</details>

<a id="0x1_vesting_withdraw_stake"></a>

## Function `withdraw_stake`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_withdraw_stake">withdraw_stake</a>(vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">vesting::VestingContract</a>, contract_address: <b>address</b>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_withdraw_stake">withdraw_stake</a>(vesting_contract: &<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>, contract_address: <b>address</b>): FungibleAsset {
    // Claim <a href="../../endless-stdlib/doc/any.md#0x1_any">any</a> withdrawable distribution from the staking contract. The withdrawn coins will be sent directly <b>to</b>
    // the <a href="vesting.md#0x1_vesting">vesting</a> contract's <a href="account.md#0x1_account">account</a>.
    <a href="staking_contract.md#0x1_staking_contract_distribute">staking_contract::distribute</a>(contract_address, vesting_contract.staking.operator);
    <b>let</b> withdraw_coins = <a href="endless_coin.md#0x1_endless_coin_balance">endless_coin::balance</a>(contract_address);
    <b>if</b> (withdraw_coins != 0) {
        <b>let</b> contract_signer = &<a href="vesting.md#0x1_vesting_get_vesting_account_signer_internal">get_vesting_account_signer_internal</a>(vesting_contract);
        <a href="endless_coin.md#0x1_endless_coin_withdraw">endless_coin::withdraw</a>(contract_signer, withdraw_coins)
    } <b>else</b> {
        <a href="endless_coin.md#0x1_endless_coin_zero">endless_coin::zero</a>()
    }
}
</code></pre>



</details>

<a id="0x1_vesting_get_beneficiary"></a>

## Function `get_beneficiary`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_get_beneficiary">get_beneficiary</a>(contract: &<a href="vesting.md#0x1_vesting_VestingContract">vesting::VestingContract</a>, shareholder: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vesting_get_beneficiary">get_beneficiary</a>(contract: &<a href="vesting.md#0x1_vesting_VestingContract">VestingContract</a>, shareholder: <b>address</b>): <b>address</b> {
    <b>if</b> (<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&contract.beneficiaries, &shareholder)) {
        *<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_borrow">simple_map::borrow</a>(&contract.beneficiaries, &shareholder)
    } <b>else</b> {
        shareholder
    }
}
</code></pre>



</details>


[move-book]: https://endless.dev/move/book/SUMMARY
