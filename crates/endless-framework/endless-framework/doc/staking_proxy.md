
<a id="0x1_staking_proxy"></a>

# Module `0x1::staking_proxy`



-  [Function `set_operator`](#0x1_staking_proxy_set_operator)
-  [Function `set_voter`](#0x1_staking_proxy_set_voter)
-  [Function `set_vesting_contract_operator`](#0x1_staking_proxy_set_vesting_contract_operator)
-  [Function `set_staking_contract_operator`](#0x1_staking_proxy_set_staking_contract_operator)
-  [Function `set_stake_pool_operator`](#0x1_staking_proxy_set_stake_pool_operator)
-  [Function `set_vesting_contract_voter`](#0x1_staking_proxy_set_vesting_contract_voter)
-  [Function `set_staking_contract_voter`](#0x1_staking_proxy_set_staking_contract_voter)
-  [Function `set_stake_pool_voter`](#0x1_staking_proxy_set_stake_pool_voter)
-  [Specification](#@Specification_0)
    -  [High-level Requirements](#high-level-req)
    -  [Module-level Specification](#module-level-spec)
    -  [Function `set_operator`](#@Specification_0_set_operator)
    -  [Function `set_voter`](#@Specification_0_set_voter)
    -  [Function `set_vesting_contract_operator`](#@Specification_0_set_vesting_contract_operator)
    -  [Function `set_staking_contract_operator`](#@Specification_0_set_staking_contract_operator)
    -  [Function `set_stake_pool_operator`](#@Specification_0_set_stake_pool_operator)
    -  [Function `set_vesting_contract_voter`](#@Specification_0_set_vesting_contract_voter)
    -  [Function `set_staking_contract_voter`](#@Specification_0_set_staking_contract_voter)
    -  [Function `set_stake_pool_voter`](#@Specification_0_set_stake_pool_voter)


<pre><code><b>use</b> <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="stake.md#0x1_stake">0x1::stake</a>;
<b>use</b> <a href="staking_contract.md#0x1_staking_contract">0x1::staking_contract</a>;
<b>use</b> <a href="vesting.md#0x1_vesting">0x1::vesting</a>;
</code></pre>



<a id="0x1_staking_proxy_set_operator"></a>

## Function `set_operator`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_operator">set_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_operator">set_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>) {
    <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_operator">set_vesting_contract_operator</a>(owner, old_operator, new_operator);
    <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_operator">set_staking_contract_operator</a>(owner, old_operator, new_operator);
    <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_operator">set_stake_pool_operator</a>(owner, new_operator);
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_voter"></a>

## Function `set_voter`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_voter">set_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_voter">set_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>) {
    <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_voter">set_vesting_contract_voter</a>(owner, operator, new_voter);
    <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_voter">set_staking_contract_voter</a>(owner, operator, new_voter);
    <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_voter">set_stake_pool_voter</a>(owner, new_voter);
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_vesting_contract_operator"></a>

## Function `set_vesting_contract_operator`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_operator">set_vesting_contract_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_operator">set_vesting_contract_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>) {
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>let</b> vesting_contracts = &<a href="vesting.md#0x1_vesting_vesting_contracts">vesting::vesting_contracts</a>(owner_address);
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(vesting_contracts, |vesting_contract| {
        <b>let</b> vesting_contract = *vesting_contract;
        <b>if</b> (<a href="vesting.md#0x1_vesting_operator">vesting::operator</a>(vesting_contract) == old_operator) {
            <b>let</b> current_commission_percentage = <a href="vesting.md#0x1_vesting_operator_commission_percentage">vesting::operator_commission_percentage</a>(vesting_contract);
            <a href="vesting.md#0x1_vesting_update_operator">vesting::update_operator</a>(owner, vesting_contract, new_operator, current_commission_percentage);
        };
    });
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_staking_contract_operator"></a>

## Function `set_staking_contract_operator`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_operator">set_staking_contract_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_operator">set_staking_contract_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>) {
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>if</b> (<a href="staking_contract.md#0x1_staking_contract_staking_contract_exists">staking_contract::staking_contract_exists</a>(owner_address, old_operator)) {
        <b>let</b> current_commission_percentage = <a href="staking_contract.md#0x1_staking_contract_commission_percentage">staking_contract::commission_percentage</a>(owner_address, old_operator);
        <a href="staking_contract.md#0x1_staking_contract_switch_operator">staking_contract::switch_operator</a>(owner, old_operator, new_operator, current_commission_percentage);
    };
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_stake_pool_operator"></a>

## Function `set_stake_pool_operator`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_operator">set_stake_pool_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_operator: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_operator">set_stake_pool_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_operator: <b>address</b>) {
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>if</b> (<a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(owner_address)) {
        <a href="stake.md#0x1_stake_set_operator">stake::set_operator</a>(owner, new_operator);
    };
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_vesting_contract_voter"></a>

## Function `set_vesting_contract_voter`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_voter">set_vesting_contract_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_voter">set_vesting_contract_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>) {
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>let</b> vesting_contracts = &<a href="vesting.md#0x1_vesting_vesting_contracts">vesting::vesting_contracts</a>(owner_address);
    <a href="../../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(vesting_contracts, |vesting_contract| {
        <b>let</b> vesting_contract = *vesting_contract;
        <b>if</b> (<a href="vesting.md#0x1_vesting_operator">vesting::operator</a>(vesting_contract) == operator) {
            <a href="vesting.md#0x1_vesting_update_voter">vesting::update_voter</a>(owner, vesting_contract, new_voter);
        };
    });
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_staking_contract_voter"></a>

## Function `set_staking_contract_voter`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_voter">set_staking_contract_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_voter">set_staking_contract_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>) {
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>if</b> (<a href="staking_contract.md#0x1_staking_contract_staking_contract_exists">staking_contract::staking_contract_exists</a>(owner_address, operator)) {
        <a href="staking_contract.md#0x1_staking_contract_update_voter">staking_contract::update_voter</a>(owner, operator, new_voter);
    };
}
</code></pre>



</details>

<a id="0x1_staking_proxy_set_stake_pool_voter"></a>

## Function `set_stake_pool_voter`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_voter">set_stake_pool_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_voter: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_voter">set_stake_pool_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_voter: <b>address</b>) {
    <b>if</b> (<a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner))) {
        <a href="stake.md#0x1_stake_set_delegated_voter">stake::set_delegated_voter</a>(owner, new_voter);
    };
}
</code></pre>



</details>

<a id="@Specification_0"></a>

## Specification




<a id="high-level-req"></a>

### High-level Requirements

<table>
<tr>
<th>No.</th><th>Requirement</th><th>Criticality</th><th>Implementation</th><th>Enforcement</th>
</tr>

<tr>
<td>1</td>
<td>When updating the Vesting operator, it should be updated throughout all depending units.</td>
<td>Medium</td>
<td>The VestingContract contains a StakingInfo object that has an operator field, and this operator is mapped to a StakingContract object that in turn encompasses a StakePool object where the operator matches.</td>
<td>Audited that it ensures the two operator fields hold the new value after the update.</td>
</tr>

<tr>
<td>2</td>
<td>When updating the Vesting voter, it should be updated throughout all depending units.</td>
<td>Medium</td>
<td>The VestingContract contains a StakingInfo object that has an operator field, and this operator is mapped to a StakingContract object that in turn encompasses a StakePool object where the operator matches.</td>
<td>Audited that it ensures the two operator fields hold the new value after the update.</td>
</tr>

<tr>
<td>3</td>
<td>The operator and voter of a Vesting Contract should only be updated by the owner of the contract.</td>
<td>High</td>
<td>The owner-operator-voter model, as defined in the documentation, grants distinct abilities to each role. Therefore, it's crucial to ensure that only the owner has the authority to modify the operator or voter, to prevent the compromise of the StakePool.</td>
<td>Audited that it ensures the signer owns the AdminStore resource and that the operator or voter intended for the update actually exists.</td>
</tr>

<tr>
<td>4</td>
<td>The operator and voter of a Staking Contract should only be updated by the owner of the contract.</td>
<td>High</td>
<td>The owner-operator-voter model, as defined in the documentation, grants distinct abilities to each role. Therefore, it's crucial to ensure that only the owner has the authority to modify the operator or voter, to prevent the compromise of the StakePool.</td>
<td>Audited the patterns of updating operators and voters in the staking contract.</td>
</tr>

<tr>
<td>5</td>
<td>Staking Contract's operators should be unique inside a store.</td>
<td>Medium</td>
<td>Duplicates among operators could result in incorrectly updating the operator or voter associated with the incorrect StakingContract.</td>
<td>Enforced via <a href="https://github.com/Endless-labs/endless/blob/main/endless-move/framework/endless-framework/sources/staking_contract.move#L87">SimpleMap</a>.</td>
</tr>

</table>




<a id="module-level-spec"></a>

### Module-level Specification


<pre><code><b>pragma</b> verify = <b>true</b>;
<b>pragma</b> aborts_if_is_strict;
</code></pre>



<a id="@Specification_0_set_operator"></a>

### Function `set_operator`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_operator">set_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>)
</code></pre>


Aborts if conditions of SetStakePoolOperator are not met


<pre><code><b>pragma</b> verify = <b>false</b>;
<b>pragma</b> aborts_if_is_partial;
<b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakePoolOperator">SetStakePoolOperator</a>;
<b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakingContractOperator">SetStakingContractOperator</a>;
</code></pre>



<a id="@Specification_0_set_voter"></a>

### Function `set_voter`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_voter">set_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>)
</code></pre>


Aborts if conditions of SetStackingContractVoter and SetStackPoolVoterAbortsIf are not met


<pre><code><b>pragma</b> aborts_if_is_partial;
<b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakingContractVoter">SetStakingContractVoter</a>;
<b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakePoolVoterAbortsIf">SetStakePoolVoterAbortsIf</a>;
</code></pre>



<a id="@Specification_0_set_vesting_contract_operator"></a>

### Function `set_vesting_contract_operator`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_operator">set_vesting_contract_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_0_set_staking_contract_operator"></a>

### Function `set_staking_contract_operator`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_operator">set_staking_contract_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, old_operator: <b>address</b>, new_operator: <b>address</b>)
</code></pre>




<pre><code><b>pragma</b> aborts_if_is_partial;
<b>pragma</b> verify = <b>false</b>;
<b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakingContractOperator">SetStakingContractOperator</a>;
</code></pre>




<a id="0x1_staking_proxy_SetStakingContractOperator"></a>


<pre><code><b>schema</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakingContractOperator">SetStakingContractOperator</a> {
    owner: <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>;
    old_operator: <b>address</b>;
    new_operator: <b>address</b>;
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>let</b> store = <b>global</b>&lt;Store&gt;(owner_address);
    <b>let</b> staking_contract_exists = <b>exists</b>&lt;Store&gt;(owner_address) && <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key">simple_map::spec_contains_key</a>(store.staking_contracts, old_operator);
    <b>aborts_if</b> staking_contract_exists && <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key">simple_map::spec_contains_key</a>(store.staking_contracts, new_operator);
    <b>let</b> <b>post</b> post_store = <b>global</b>&lt;Store&gt;(owner_address);
    <b>ensures</b> staking_contract_exists ==&gt; !<a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key">simple_map::spec_contains_key</a>(post_store.staking_contracts, old_operator);
    <b>let</b> <a href="staking_contract.md#0x1_staking_contract">staking_contract</a> = <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_get">simple_map::spec_get</a>(store.staking_contracts, old_operator);
    <b>let</b> stake_pool = <b>global</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(<a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.pool_address);
    <b>let</b> active = stake_pool.active;
    <b>let</b> pending_active = stake_pool.pending_active;
    <b>let</b> total_active_stake = active + pending_active;
    <b>let</b> accumulated_rewards = total_active_stake - <a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.principal;
    <b>let</b> commission_amount = accumulated_rewards * <a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.commission_percentage / 100;
    <b>aborts_if</b> staking_contract_exists && !<b>exists</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(<a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.pool_address);
    <b>ensures</b> staking_contract_exists ==&gt;
        <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_get">simple_map::spec_get</a>(post_store.staking_contracts, new_operator).principal == total_active_stake - commission_amount;
    <b>let</b> pool_address = <a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.owner_cap.pool_address;
    <b>let</b> current_commission_percentage = <a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.commission_percentage;
    <b>aborts_if</b> staking_contract_exists && commission_amount != 0 && !<b>exists</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(pool_address);
    <b>ensures</b> staking_contract_exists && commission_amount != 0 ==&gt;
        <b>global</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(pool_address).operator_address == new_operator
        && <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_get">simple_map::spec_get</a>(post_store.staking_contracts, new_operator).commission_percentage == current_commission_percentage;
    <b>ensures</b> staking_contract_exists ==&gt; <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key">simple_map::spec_contains_key</a>(post_store.staking_contracts, new_operator);
}
</code></pre>



<a id="@Specification_0_set_stake_pool_operator"></a>

### Function `set_stake_pool_operator`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_operator">set_stake_pool_operator</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_operator: <b>address</b>)
</code></pre>


Aborts if stake_pool is exists and when OwnerCapability or stake_pool_exists
One of them are not exists


<pre><code><b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakePoolOperator">SetStakePoolOperator</a>;
</code></pre>




<a id="0x1_staking_proxy_SetStakePoolOperator"></a>


<pre><code><b>schema</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakePoolOperator">SetStakePoolOperator</a> {
    owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>;
    new_operator: <b>address</b>;
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>let</b> ownership_cap = <b>borrow_global</b>&lt;<a href="stake.md#0x1_stake_OwnerCapability">stake::OwnerCapability</a>&gt;(owner_address);
    <b>let</b> pool_address = ownership_cap.pool_address;
    <b>aborts_if</b> <a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(owner_address) && !(<b>exists</b>&lt;<a href="stake.md#0x1_stake_OwnerCapability">stake::OwnerCapability</a>&gt;(owner_address) && <a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(pool_address));
    <b>ensures</b> <a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(owner_address) ==&gt; <b>global</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(pool_address).operator_address == new_operator;
}
</code></pre>



<a id="@Specification_0_set_vesting_contract_voter"></a>

### Function `set_vesting_contract_voter`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_vesting_contract_voter">set_vesting_contract_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>)
</code></pre>




<pre><code><b>pragma</b> verify = <b>false</b>;
</code></pre>



<a id="@Specification_0_set_staking_contract_voter"></a>

### Function `set_staking_contract_voter`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_staking_contract_voter">set_staking_contract_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, new_voter: <b>address</b>)
</code></pre>




<pre><code><b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakingContractVoter">SetStakingContractVoter</a>;
</code></pre>


Make sure staking_contract_exists first
Then abort if the resource is not exist


<a id="0x1_staking_proxy_SetStakingContractVoter"></a>


<pre><code><b>schema</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakingContractVoter">SetStakingContractVoter</a> {
    owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>;
    operator: <b>address</b>;
    new_voter: <b>address</b>;
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>let</b> staker = owner_address;
    <b>let</b> store = <b>global</b>&lt;Store&gt;(staker);
    <b>let</b> staking_contract_exists = <b>exists</b>&lt;Store&gt;(staker) && <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_contains_key">simple_map::spec_contains_key</a>(store.staking_contracts, operator);
    <b>let</b> staker_address = owner_address;
    <b>let</b> <a href="staking_contract.md#0x1_staking_contract">staking_contract</a> = <a href="../../endless-stdlib/doc/simple_map.md#0x1_simple_map_spec_get">simple_map::spec_get</a>(store.staking_contracts, operator);
    <b>let</b> pool_address = <a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.pool_address;
    <b>let</b> pool_address1 = <a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.owner_cap.pool_address;
    <b>aborts_if</b> staking_contract_exists && !<b>exists</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(pool_address);
    <b>aborts_if</b> staking_contract_exists && !<b>exists</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(<a href="staking_contract.md#0x1_staking_contract">staking_contract</a>.owner_cap.pool_address);
    <b>ensures</b> staking_contract_exists ==&gt; <b>global</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(pool_address1).delegated_voter == new_voter;
}
</code></pre>



<a id="@Specification_0_set_stake_pool_voter"></a>

### Function `set_stake_pool_voter`


<pre><code><b>public</b> entry <b>fun</b> <a href="staking_proxy.md#0x1_staking_proxy_set_stake_pool_voter">set_stake_pool_voter</a>(owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, new_voter: <b>address</b>)
</code></pre>




<pre><code><b>include</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakePoolVoterAbortsIf">SetStakePoolVoterAbortsIf</a>;
</code></pre>




<a id="0x1_staking_proxy_SetStakePoolVoterAbortsIf"></a>


<pre><code><b>schema</b> <a href="staking_proxy.md#0x1_staking_proxy_SetStakePoolVoterAbortsIf">SetStakePoolVoterAbortsIf</a> {
    owner: &<a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>;
    new_voter: <b>address</b>;
    <b>let</b> owner_address = <a href="../../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <b>let</b> ownership_cap = <b>global</b>&lt;<a href="stake.md#0x1_stake_OwnerCapability">stake::OwnerCapability</a>&gt;(owner_address);
    <b>let</b> pool_address = ownership_cap.pool_address;
    <b>aborts_if</b> <a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(owner_address) && !(<b>exists</b>&lt;<a href="stake.md#0x1_stake_OwnerCapability">stake::OwnerCapability</a>&gt;(owner_address) && <a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(pool_address));
    <b>ensures</b> <a href="stake.md#0x1_stake_stake_pool_exists">stake::stake_pool_exists</a>(owner_address) ==&gt; <b>global</b>&lt;<a href="stake.md#0x1_stake_StakePool">stake::StakePool</a>&gt;(pool_address).delegated_voter == new_voter;
}
</code></pre>


[move-book]: https://endless.dev/move/book/SUMMARY
