
<a id="0x4_token"></a>

# Module `0x4::token`

This defines an object-based Token. The key differentiating features from the Endless standard
token are:
* Decoupled token ownership from token data.
* Explicit data model for token metadata via adjacent resources
* Extensible framework for tokens


-  [Resource `Token`](#0x4_token_Token)
-  [Resource `TokenIdentifiers`](#0x4_token_TokenIdentifiers)
-  [Resource `ConcurrentTokenIdentifiers`](#0x4_token_ConcurrentTokenIdentifiers)
-  [Struct `BurnRef`](#0x4_token_BurnRef)
-  [Struct `MutatorRef`](#0x4_token_MutatorRef)
-  [Struct `MutationEvent`](#0x4_token_MutationEvent)
-  [Constants](#@Constants_0)
-  [Function `create_common`](#0x4_token_create_common)
-  [Function `create`](#0x4_token_create)
-  [Function `create_numbered_token`](#0x4_token_create_numbered_token)
-  [Function `create_named_token`](#0x4_token_create_named_token)
-  [Function `create_from_account`](#0x4_token_create_from_account)
-  [Function `create_token_address`](#0x4_token_create_token_address)
-  [Function `create_token_seed`](#0x4_token_create_token_seed)
-  [Function `generate_mutator_ref`](#0x4_token_generate_mutator_ref)
-  [Function `generate_burn_ref`](#0x4_token_generate_burn_ref)
-  [Function `address_from_burn_ref`](#0x4_token_address_from_burn_ref)
-  [Function `borrow`](#0x4_token_borrow)
-  [Function `creator`](#0x4_token_creator)
-  [Function `collection_name`](#0x4_token_collection_name)
-  [Function `collection_object`](#0x4_token_collection_object)
-  [Function `description`](#0x4_token_description)
-  [Function `name`](#0x4_token_name)
-  [Function `uri`](#0x4_token_uri)
-  [Function `royalty`](#0x4_token_royalty)
-  [Function `index`](#0x4_token_index)
-  [Function `borrow_mut`](#0x4_token_borrow_mut)
-  [Function `burn`](#0x4_token_burn)
-  [Function `set_description`](#0x4_token_set_description)
-  [Function `set_name`](#0x4_token_set_name)
-  [Function `set_uri`](#0x4_token_set_uri)


<pre><code><b>use</b> <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2">0x1::aggregator_v2</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="../../endless-framework/doc/event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/features.md#0x1_features">0x1::features</a>;
<b>use</b> <a href="../../endless-framework/doc/object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/doc/string_utils.md#0x1_string_utils">0x1::string_utils</a>;
<b>use</b> <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="collection.md#0x4_collection">0x4::collection</a>;
<b>use</b> <a href="royalty.md#0x4_royalty">0x4::royalty</a>;
</code></pre>



<a id="0x4_token_Token"></a>

## Resource `Token`

Represents the common fields to all tokens.


<pre><code>#[resource_group_member(#[group = <a href="../../endless-framework/doc/object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
<b>struct</b> <a href="token.md#0x4_token_Token">Token</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="collection.md#0x4_collection">collection</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x4_collection_Collection">collection::Collection</a>&gt;</code>
</dt>
<dd>
 The collection from which this token resides.
</dd>
<dt>
<code>index: u64</code>
</dt>
<dd>
 Deprecated in favor of <code>index</code> inside TokenIdentifiers.
 Will be populated until concurrent_token_v2_enabled feature flag is enabled.

 Unique identifier within the collection, optional, 0 means unassigned
</dd>
<dt>
<code>description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 A brief description of the token.
</dd>
<dt>
<code>name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Deprecated in favor of <code>name</code> inside TokenIdentifiers.
 Will be populated until concurrent_token_v2_enabled feature flag is enabled.

 The name of the token, which should be unique within the collection; the length of name
 should be smaller than 128, characters, eg: "Endless Animal #1234"
</dd>
<dt>
<code>uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off-chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
</dl>


</details>

<a id="0x4_token_TokenIdentifiers"></a>

## Resource `TokenIdentifiers`

Represents first addition to the common fields for all tokens
Starts being populated once aggregator_v2_api_enabled is enabled.


<pre><code>#[resource_group_member(#[group = <a href="../../endless-framework/doc/object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
<b>struct</b> <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot">aggregator_v2::AggregatorSnapshot</a>&lt;u64&gt;</code>
</dt>
<dd>
 Unique identifier within the collection, optional, 0 means unassigned
</dd>
<dt>
<code>name: <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_DerivedStringSnapshot">aggregator_v2::DerivedStringSnapshot</a></code>
</dt>
<dd>
 The name of the token, which should be unique within the collection; the length of name
 should be smaller than 128, characters, eg: "Endless Animal #1234"
</dd>
</dl>


</details>

<a id="0x4_token_ConcurrentTokenIdentifiers"></a>

## Resource `ConcurrentTokenIdentifiers`



<pre><code>#[resource_group_member(#[group = <a href="../../endless-framework/doc/object.md#0x1_object_ObjectGroup">0x1::object::ObjectGroup</a>])]
#[deprecated]
<b>struct</b> <a href="token.md#0x4_token_ConcurrentTokenIdentifiers">ConcurrentTokenIdentifiers</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>index: <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot">aggregator_v2::AggregatorSnapshot</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>name: <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_AggregatorSnapshot">aggregator_v2::AggregatorSnapshot</a>&lt;<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_token_BurnRef"></a>

## Struct `BurnRef`

This enables burning an NFT, if possible, it will also delete the object. Note, the data
in inner and self occupies 32-bytes each, rather than have both, this data structure makes
a small optimization to support either and take a fixed amount of 34-bytes.


<pre><code><b>struct</b> <a href="token.md#0x4_token_BurnRef">BurnRef</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../endless-framework/doc/object.md#0x1_object_DeleteRef">object::DeleteRef</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>self: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_token_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating description and URI by higher level services.


<pre><code><b>struct</b> <a href="token.md#0x4_token_MutatorRef">MutatorRef</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x4_token_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


<pre><code>#[<a href="../../endless-framework/doc/event.md#0x1_event">event</a>]
<b>struct</b> <a href="token.md#0x4_token_MutationEvent">MutationEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>mutated_field_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>old_value: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>new_value: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x4_token_EURI_TOO_LONG"></a>

The URI is over the maximum length


<pre><code><b>const</b> <a href="token.md#0x4_token_EURI_TOO_LONG">EURI_TOO_LONG</a>: u64 = 5;
</code></pre>



<a id="0x4_token_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> <a href="token.md#0x4_token_MAX_URI_LENGTH">MAX_URI_LENGTH</a>: u64 = 512;
</code></pre>



<a id="0x4_token_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


<pre><code><b>const</b> <a href="token.md#0x4_token_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>: u64 = 6;
</code></pre>



<a id="0x4_token_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="token.md#0x4_token_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>: u64 = 2048;
</code></pre>



<a id="0x4_token_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


<pre><code><b>const</b> <a href="token.md#0x4_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>: u64 = 3;
</code></pre>



<a id="0x4_token_ENOT_CREATOR"></a>

The provided signer is not the creator


<pre><code><b>const</b> <a href="token.md#0x4_token_ENOT_CREATOR">ENOT_CREATOR</a>: u64 = 2;
</code></pre>



<a id="0x4_token_ETOKEN_DOES_NOT_EXIST"></a>

The token does not exist


<pre><code><b>const</b> <a href="token.md#0x4_token_ETOKEN_DOES_NOT_EXIST">ETOKEN_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x4_token_ETOKEN_NAME_TOO_LONG"></a>

The token name is over the maximum length


<pre><code><b>const</b> <a href="token.md#0x4_token_ETOKEN_NAME_TOO_LONG">ETOKEN_NAME_TOO_LONG</a>: u64 = 4;
</code></pre>



<a id="0x4_token_MAX_TOKEN_NAME_LENGTH"></a>



<pre><code><b>const</b> <a href="token.md#0x4_token_MAX_TOKEN_NAME_LENGTH">MAX_TOKEN_NAME_LENGTH</a>: u64 = 128;
</code></pre>



<a id="0x4_token_create_common"></a>

## Function `create_common`



<pre><code><b>fun</b> <a href="token.md#0x4_token_create_common">create_common</a>(constructor_ref: &<a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, creator_address: <b>address</b>, collection_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name_prefix: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name_with_index_suffix: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="royalty.md#0x4_royalty">royalty</a>: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x4_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="token.md#0x4_token_create_common">create_common</a>(
    constructor_ref: &ConstructorRef,
    creator_address: <b>address</b>,
    collection_name: String,
    description: String,
    name_prefix: String,
    // If <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>, numbered <a href="token.md#0x4_token">token</a> is created - i.e. index is appended <b>to</b> the name.
    // If <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>, name_prefix is the full name of the <a href="token.md#0x4_token">token</a>.
    name_with_index_suffix: Option&lt;String&gt;,
    <a href="royalty.md#0x4_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
) {
    <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&name_with_index_suffix)) {
        // Be conservative, <b>as</b> we don't know what length the index will be, and <b>assume</b> worst case (20 chars in MAX_U64)
        <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&name_prefix) + 20 + <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&name_with_index_suffix)) &lt;= <a href="token.md#0x4_token_MAX_TOKEN_NAME_LENGTH">MAX_TOKEN_NAME_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_ETOKEN_NAME_TOO_LONG">ETOKEN_NAME_TOO_LONG</a>));
    } <b>else</b> {
        <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&name_prefix) &lt;= <a href="token.md#0x4_token_MAX_TOKEN_NAME_LENGTH">MAX_TOKEN_NAME_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_ETOKEN_NAME_TOO_LONG">ETOKEN_NAME_TOO_LONG</a>));
    };
    <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&description) &lt;= <a href="token.md#0x4_token_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>));
    <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&uri) &lt;= <a href="token.md#0x4_token_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_EURI_TOO_LONG">EURI_TOO_LONG</a>));

    <b>let</b> object_signer = <a href="../../endless-framework/doc/object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);

    <b>let</b> collection_addr = <a href="collection.md#0x4_collection_create_collection_address">collection::create_collection_address</a>(&creator_address, &collection_name);
    <b>let</b> <a href="collection.md#0x4_collection">collection</a> = <a href="../../endless-framework/doc/object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;Collection&gt;(collection_addr);

    // TODO[agg_v2](cleanup) once this flag is enabled, cleanup <a href="../../endless-framework/doc/code.md#0x1_code">code</a> for aggregator_api_enabled = <b>false</b>.
    // Flag which controls whether <a href="../../endless-framework/../endless-stdlib/doc/any.md#0x1_any">any</a> functions from <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2">aggregator_v2</a> <b>module</b> can be called.
    <b>let</b> aggregator_api_enabled = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/features.md#0x1_features_aggregator_v2_api_enabled">features::aggregator_v2_api_enabled</a>();
    // Flag which controls whether we are going <b>to</b> still <b>continue</b> writing <b>to</b> deprecated fields.
    <b>let</b> concurrent_token_v2_enabled = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/features.md#0x1_features_concurrent_token_v2_enabled">features::concurrent_token_v2_enabled</a>();

    <b>let</b> (deprecated_index, deprecated_name) = <b>if</b> (aggregator_api_enabled) {
        <b>let</b> index = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_destroy_with_default">option::destroy_with_default</a>(
            <a href="collection.md#0x4_collection_increment_concurrent_supply">collection::increment_concurrent_supply</a>(&<a href="collection.md#0x4_collection">collection</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&object_signer)),
            <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_snapshot">aggregator_v2::create_snapshot</a>&lt;u64&gt;(0)
        );

        // If create_numbered_token called us, add index <b>to</b> the name.
        <b>let</b> name = <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&name_with_index_suffix)) {
            <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_derive_string_concat">aggregator_v2::derive_string_concat</a>(name_prefix, &index, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> name_with_index_suffix))
        } <b>else</b> {
            <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_derived_string">aggregator_v2::create_derived_string</a>(name_prefix)
        };

        // Until concurrent_token_v2_enabled is enabled, we still need <b>to</b> write <b>to</b> deprecated fields.
        // Otherwise we put empty values there.
        // (we need <b>to</b> do these calls before creating token_concurrent, <b>to</b> avoid copying objects)
        <b>let</b> deprecated_index = <b>if</b> (concurrent_token_v2_enabled) {
            0
        } <b>else</b> {
            <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_snapshot">aggregator_v2::read_snapshot</a>(&index)
        };
        <b>let</b> deprecated_name = <b>if</b> (concurrent_token_v2_enabled) {
            <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"")
        } <b>else</b> {
            <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_derived_string">aggregator_v2::read_derived_string</a>(&name)
        };

        // If aggregator_api_enabled, we always populate newly added fields
        <b>let</b> token_concurrent = <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> {
            index,
            name,
        };
        <b>move_to</b>(&object_signer, token_concurrent);

        (deprecated_index, deprecated_name)
    } <b>else</b> {
        // If aggregator_api_enabled is disabled, we cannot <b>use</b> increment_concurrent_supply or
        // create <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>, so we fallback <b>to</b> the <b>old</b> behavior.
        <b>let</b> id = <a href="collection.md#0x4_collection_increment_supply">collection::increment_supply</a>(&<a href="collection.md#0x4_collection">collection</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&object_signer));
        <b>let</b> index = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_get_with_default">option::get_with_default</a>(&<b>mut</b> id, 0);

        // If create_numbered_token called us, add index <b>to</b> the name.
        <b>let</b> name = <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&name_with_index_suffix)) {
            <b>let</b> name = name_prefix;
            <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_append">string::append</a>(&<b>mut</b> name, to_string&lt;u64&gt;(&index));
            <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_append">string::append</a>(&<b>mut</b> name, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> name_with_index_suffix));
            name
        } <b>else</b> {
            name_prefix
        };

        (index, name)
    };

    <b>let</b> <a href="token.md#0x4_token">token</a> = <a href="token.md#0x4_token_Token">Token</a> {
        <a href="collection.md#0x4_collection">collection</a>,
        index: deprecated_index,
        description,
        name: deprecated_name,
        uri,
    };
    <b>move_to</b>(&object_signer, <a href="token.md#0x4_token">token</a>);

    <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&<a href="royalty.md#0x4_royalty">royalty</a>)) {
        <a href="royalty.md#0x4_royalty_init">royalty::init</a>(constructor_ref, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="royalty.md#0x4_royalty">royalty</a>))
    };
}
</code></pre>



</details>

<a id="0x4_token_create"></a>

## Function `create`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create">create</a>(creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, collection_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x4_royalty">royalty</a>: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x4_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create">create</a>(
    creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    collection_name: String,
    description: String,
    name: String,
    <a href="royalty.md#0x4_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator);
    <b>let</b> constructor_ref = <a href="../../endless-framework/doc/object.md#0x1_object_create_object">object::create_object</a>(creator_address);
    <a href="token.md#0x4_token_create_common">create_common</a>(&constructor_ref, creator_address, collection_name, description, name, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="royalty.md#0x4_royalty">royalty</a>, uri);
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_numbered_token"></a>

## Function `create_numbered_token`

Creates a new token object with a unique address and returns the ConstructorRef
for additional specialization.
The name is created by concatenating the (name_prefix, index, name_suffix).
After flag concurrent_token_v2_enabled is enabled, this function will allow
creating tokens in parallel, from the same collection, while providing sequential names.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_numbered_token">create_numbered_token</a>(creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, collection_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name_with_index_prefix: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name_with_index_suffix: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x4_royalty">royalty</a>: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x4_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_numbered_token">create_numbered_token</a>(
    creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    collection_name: String,
    description: String,
    name_with_index_prefix: String,
    name_with_index_suffix: String,
    <a href="royalty.md#0x4_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator);
    <b>let</b> constructor_ref = <a href="../../endless-framework/doc/object.md#0x1_object_create_object">object::create_object</a>(creator_address);
    <a href="token.md#0x4_token_create_common">create_common</a>(&constructor_ref, creator_address, collection_name, description, name_with_index_prefix, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(name_with_index_suffix), <a href="royalty.md#0x4_royalty">royalty</a>, uri);
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_named_token"></a>

## Function `create_named_token`

Creates a new token object from a token name and returns the ConstructorRef for
additional specialization.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_named_token">create_named_token</a>(creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, collection_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x4_royalty">royalty</a>: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x4_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_named_token">create_named_token</a>(
    creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    collection_name: String,
    description: String,
    name: String,
    <a href="royalty.md#0x4_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator);
    <b>let</b> seed = <a href="token.md#0x4_token_create_token_seed">create_token_seed</a>(&collection_name, &name);

    <b>let</b> constructor_ref = <a href="../../endless-framework/doc/object.md#0x1_object_create_named_object">object::create_named_object</a>(creator, seed);
    <a href="token.md#0x4_token_create_common">create_common</a>(&constructor_ref, creator_address, collection_name, description, name, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="royalty.md#0x4_royalty">royalty</a>, uri);
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_from_account"></a>

## Function `create_from_account`

DEPRECATED: Use <code>create</code> instead for identical behavior.

Creates a new token object from an account GUID and returns the ConstructorRef for
additional specialization.


<pre><code>#[deprecated]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_from_account">create_from_account</a>(creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>, collection_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x4_royalty">royalty</a>: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x4_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_from_account">create_from_account</a>(
    creator: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer">signer</a>,
    collection_name: String,
    description: String,
    name: String,
    <a href="royalty.md#0x4_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator);
    <b>let</b> constructor_ref = <a href="../../endless-framework/doc/object.md#0x1_object_create_object_from_account">object::create_object_from_account</a>(creator);
    <a href="token.md#0x4_token_create_common">create_common</a>(&constructor_ref, creator_address, collection_name, description, name, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="royalty.md#0x4_royalty">royalty</a>, uri);
    constructor_ref
}
</code></pre>



</details>

<a id="0x4_token_create_token_address"></a>

## Function `create_token_address`

Generates the token's address based upon the creator's address, the collection's name and the token's name.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_token_address">create_token_address</a>(creator: &<b>address</b>, <a href="collection.md#0x4_collection">collection</a>: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_token_address">create_token_address</a>(creator: &<b>address</b>, <a href="collection.md#0x4_collection">collection</a>: &String, name: &String): <b>address</b> {
    <a href="../../endless-framework/doc/object.md#0x1_object_create_object_address">object::create_object_address</a>(creator, <a href="token.md#0x4_token_create_token_seed">create_token_seed</a>(<a href="collection.md#0x4_collection">collection</a>, name))
}
</code></pre>



</details>

<a id="0x4_token_create_token_seed"></a>

## Function `create_token_seed`

Named objects are derived from a seed, the token's seed is its name appended to the collection's name.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_token_seed">create_token_seed</a>(<a href="collection.md#0x4_collection">collection</a>: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>, name: &<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_create_token_seed">create_token_seed</a>(<a href="collection.md#0x4_collection">collection</a>: &String, name: &String): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(name) &lt;= <a href="token.md#0x4_token_MAX_TOKEN_NAME_LENGTH">MAX_TOKEN_NAME_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_ETOKEN_NAME_TOO_LONG">ETOKEN_NAME_TOO_LONG</a>));
    <b>let</b> seed = *<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(<a href="collection.md#0x4_collection">collection</a>);
    <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, b"::");
    <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, *<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(name));
    seed
}
</code></pre>



</details>

<a id="0x4_token_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_generate_mutator_ref">generate_mutator_ref</a>(ref: &<a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="token.md#0x4_token_MutatorRef">token::MutatorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_generate_mutator_ref">generate_mutator_ref</a>(ref: &ConstructorRef): <a href="token.md#0x4_token_MutatorRef">MutatorRef</a> {
    <b>let</b> <a href="../../endless-framework/doc/object.md#0x1_object">object</a> = <a href="../../endless-framework/doc/object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(ref);
    <a href="token.md#0x4_token_MutatorRef">MutatorRef</a> { self: <a href="../../endless-framework/doc/object.md#0x1_object_object_address">object::object_address</a>(&<a href="../../endless-framework/doc/object.md#0x1_object">object</a>) }
}
</code></pre>



</details>

<a id="0x4_token_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a BurnRef, which gates the ability to burn the given token.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_generate_burn_ref">generate_burn_ref</a>(ref: &<a href="../../endless-framework/doc/object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="token.md#0x4_token_BurnRef">token::BurnRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_generate_burn_ref">generate_burn_ref</a>(ref: &ConstructorRef): <a href="token.md#0x4_token_BurnRef">BurnRef</a> {
    <b>let</b> (inner, self) = <b>if</b> (<a href="../../endless-framework/doc/object.md#0x1_object_can_generate_delete_ref">object::can_generate_delete_ref</a>(ref)) {
        <b>let</b> delete_ref = <a href="../../endless-framework/doc/object.md#0x1_object_generate_delete_ref">object::generate_delete_ref</a>(ref);
        (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(delete_ref), <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>())
    } <b>else</b> {
        <b>let</b> addr = <a href="../../endless-framework/doc/object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(ref);
        (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(addr))
    };
    <a href="token.md#0x4_token_BurnRef">BurnRef</a> { self, inner }
}
</code></pre>



</details>

<a id="0x4_token_address_from_burn_ref"></a>

## Function `address_from_burn_ref`

Extracts the tokens address from a BurnRef.


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_address_from_burn_ref">address_from_burn_ref</a>(ref: &<a href="token.md#0x4_token_BurnRef">token::BurnRef</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_address_from_burn_ref">address_from_burn_ref</a>(ref: &<a href="token.md#0x4_token_BurnRef">BurnRef</a>): <b>address</b> {
    <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&ref.inner)) {
        <a href="../../endless-framework/doc/object.md#0x1_object_address_from_delete_ref">object::address_from_delete_ref</a>(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&ref.inner))
    } <b>else</b> {
        *<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&ref.self)
    }
}
</code></pre>



</details>

<a id="0x4_token_borrow"></a>

## Function `borrow`



<pre><code><b>fun</b> <a href="token.md#0x4_token_borrow">borrow</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: &<a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): &<a href="token.md#0x4_token_Token">token::Token</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="token.md#0x4_token_borrow">borrow</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: &Object&lt;T&gt;): &<a href="token.md#0x4_token_Token">Token</a> <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <b>let</b> token_address = <a href="../../endless-framework/doc/object.md#0x1_object_object_address">object::object_address</a>(<a href="token.md#0x4_token">token</a>);
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(token_address),
        <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="token.md#0x4_token_ETOKEN_DOES_NOT_EXIST">ETOKEN_DOES_NOT_EXIST</a>),
    );
    <b>borrow_global</b>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(token_address)
}
</code></pre>



</details>

<a id="0x4_token_creator"></a>

## Function `creator`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_creator">creator</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_creator">creator</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): <b>address</b> <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <a href="collection.md#0x4_collection_creator">collection::creator</a>(<a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).<a href="collection.md#0x4_collection">collection</a>)
}
</code></pre>



</details>

<a id="0x4_token_collection_name"></a>

## Function `collection_name`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_collection_name">collection_name</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_collection_name">collection_name</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <a href="collection.md#0x4_collection_name">collection::name</a>(<a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).<a href="collection.md#0x4_collection">collection</a>)
}
</code></pre>



</details>

<a id="0x4_token_collection_object"></a>

## Function `collection_object`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_collection_object">collection_object</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x4_collection_Collection">collection::Collection</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_collection_object">collection_object</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): Object&lt;Collection&gt; <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).<a href="collection.md#0x4_collection">collection</a>
}
</code></pre>



</details>

<a id="0x4_token_description"></a>

## Function `description`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_description">description</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_description">description</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).description
}
</code></pre>



</details>

<a id="0x4_token_name"></a>

## Function `name`

Avoid this method in the same transaction as the token is minted
as that would prohibit transactions to be executed in parallel.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_name">name</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_name">name</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a>, <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> {
    <b>let</b> token_address = <a href="../../endless-framework/doc/object.md#0x1_object_object_address">object::object_address</a>(&<a href="token.md#0x4_token">token</a>);
    <b>if</b> (<b>exists</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(token_address)) {
        <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_derived_string">aggregator_v2::read_derived_string</a>(&<b>borrow_global</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(token_address).name)
    } <b>else</b> {
        <a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).name
    }
}
</code></pre>



</details>

<a id="0x4_token_uri"></a>

## Function `uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_uri">uri</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_uri">uri</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).uri
}
</code></pre>



</details>

<a id="0x4_token_royalty"></a>

## Function `royalty`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="royalty.md#0x4_royalty">royalty</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x4_royalty_Royalty">royalty::Royalty</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x4_royalty">royalty</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): Option&lt;Royalty&gt; <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>);
    <b>let</b> <a href="royalty.md#0x4_royalty">royalty</a> = <a href="royalty.md#0x4_royalty_get">royalty::get</a>(<a href="token.md#0x4_token">token</a>);
    <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&<a href="royalty.md#0x4_royalty">royalty</a>)) {
        <a href="royalty.md#0x4_royalty">royalty</a>
    } <b>else</b> {
        <b>let</b> creator = <a href="token.md#0x4_token_creator">creator</a>(<a href="token.md#0x4_token">token</a>);
        <b>let</b> collection_name = <a href="token.md#0x4_token_collection_name">collection_name</a>(<a href="token.md#0x4_token">token</a>);
        <b>let</b> collection_address = <a href="collection.md#0x4_collection_create_collection_address">collection::create_collection_address</a>(&creator, &collection_name);
        <b>let</b> <a href="collection.md#0x4_collection">collection</a> = <a href="../../endless-framework/doc/object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="collection.md#0x4_collection_Collection">collection::Collection</a>&gt;(collection_address);
        <a href="royalty.md#0x4_royalty_get">royalty::get</a>(<a href="collection.md#0x4_collection">collection</a>)
    }
}
</code></pre>



</details>

<a id="0x4_token_index"></a>

## Function `index`

Avoid this method in the same transaction as the token is minted
as that would prohibit transactions to be executed in parallel.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="token.md#0x4_token_index">index</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: <a href="../../endless-framework/doc/object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_index">index</a>&lt;T: key&gt;(<a href="token.md#0x4_token">token</a>: Object&lt;T&gt;): u64 <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a>, <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> {
    <b>let</b> token_address = <a href="../../endless-framework/doc/object.md#0x1_object_object_address">object::object_address</a>(&<a href="token.md#0x4_token">token</a>);
    <b>if</b> (<b>exists</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(token_address)) {
        <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_snapshot">aggregator_v2::read_snapshot</a>(&<b>borrow_global</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(token_address).index)
    } <b>else</b> {
        <a href="token.md#0x4_token_borrow">borrow</a>(&<a href="token.md#0x4_token">token</a>).index
    }
}
</code></pre>



</details>

<a id="0x4_token_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>fun</b> <a href="token.md#0x4_token_borrow_mut">borrow_mut</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">token::MutatorRef</a>): &<b>mut</b> <a href="token.md#0x4_token_Token">token::Token</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="token.md#0x4_token_borrow_mut">borrow_mut</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">MutatorRef</a>): &<b>mut</b> <a href="token.md#0x4_token_Token">Token</a> <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(mutator_ref.self),
        <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="token.md#0x4_token_ETOKEN_DOES_NOT_EXIST">ETOKEN_DOES_NOT_EXIST</a>),
    );
    <b>borrow_global_mut</b>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(mutator_ref.self)
}
</code></pre>



</details>

<a id="0x4_token_burn"></a>

## Function `burn`



<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_burn">burn</a>(burn_ref: <a href="token.md#0x4_token_BurnRef">token::BurnRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_burn">burn</a>(burn_ref: <a href="token.md#0x4_token_BurnRef">BurnRef</a>) <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a>, <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> {
    <b>let</b> (addr, previous_owner) = <b>if</b> (<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&burn_ref.inner)) {
        <b>let</b> delete_ref = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> burn_ref.inner);
        <b>let</b> addr = <a href="../../endless-framework/doc/object.md#0x1_object_address_from_delete_ref">object::address_from_delete_ref</a>(&delete_ref);
        <b>let</b> previous_owner = <a href="../../endless-framework/doc/object.md#0x1_object_owner">object::owner</a>(<a href="../../endless-framework/doc/object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(addr));
        <a href="../../endless-framework/doc/object.md#0x1_object_delete">object::delete</a>(delete_ref);
        (addr, previous_owner)
    } <b>else</b> {
        <b>let</b> addr = <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> burn_ref.self);
        <b>let</b> previous_owner = <a href="../../endless-framework/doc/object.md#0x1_object_owner">object::owner</a>(<a href="../../endless-framework/doc/object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(addr));
        (addr, previous_owner)
    };

    <b>if</b> (<a href="royalty.md#0x4_royalty_exists_at">royalty::exists_at</a>(addr)) {
        <a href="royalty.md#0x4_royalty_delete">royalty::delete</a>(addr)
    };

    <b>let</b> <a href="token.md#0x4_token_Token">Token</a> {
        <a href="collection.md#0x4_collection">collection</a>,
        index: deprecated_index,
        description: _,
        name: _,
        uri: _,
    } = <b>move_from</b>&lt;<a href="token.md#0x4_token_Token">Token</a>&gt;(addr);

    <b>let</b> index = <b>if</b> (<b>exists</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(addr)) {
        <b>let</b> <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> {
            index,
            name: _,
        } = <b>move_from</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(addr);
        <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_snapshot">aggregator_v2::read_snapshot</a>(&index)
    } <b>else</b> {
        deprecated_index
    };

    <a href="collection.md#0x4_collection_decrement_supply">collection::decrement_supply</a>(&<a href="collection.md#0x4_collection">collection</a>, addr, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/option.md#0x1_option_some">option::some</a>(index), previous_owner);
}
</code></pre>



</details>

<a id="0x4_token_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_set_description">set_description</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">token::MutatorRef</a>, description: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_set_description">set_description</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">MutatorRef</a>, description: String) <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&description) &lt;= <a href="token.md#0x4_token_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>));
    <b>let</b> <a href="token.md#0x4_token">token</a> = <a href="token.md#0x4_token_borrow_mut">borrow_mut</a>(mutator_ref);
    <a href="../../endless-framework/doc/event.md#0x1_event_emit">event::emit</a>(
        <a href="token.md#0x4_token_MutationEvent">MutationEvent</a> {
            mutated_field_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"description"),
            old_value: <a href="token.md#0x4_token">token</a>.description,
            new_value: description
        },
    );
    <a href="token.md#0x4_token">token</a>.description = description;
}
</code></pre>



</details>

<a id="0x4_token_set_name"></a>

## Function `set_name`



<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_set_name">set_name</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">token::MutatorRef</a>, name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_set_name">set_name</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">MutatorRef</a>, name: String) <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a>, <a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a> {
    <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&name) &lt;= <a href="token.md#0x4_token_MAX_TOKEN_NAME_LENGTH">MAX_TOKEN_NAME_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_ETOKEN_NAME_TOO_LONG">ETOKEN_NAME_TOO_LONG</a>));

    <b>let</b> <a href="token.md#0x4_token">token</a> = <a href="token.md#0x4_token_borrow_mut">borrow_mut</a>(mutator_ref);

    <b>let</b> old_name = <b>if</b> (<b>exists</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(mutator_ref.self)) {
        <b>let</b> token_concurrent = <b>borrow_global_mut</b>&lt;<a href="token.md#0x4_token_TokenIdentifiers">TokenIdentifiers</a>&gt;(mutator_ref.self);
        <b>let</b> old_name = <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_read_derived_string">aggregator_v2::read_derived_string</a>(&token_concurrent.name);
        token_concurrent.name = <a href="../../endless-framework/doc/aggregator_v2.md#0x1_aggregator_v2_create_derived_string">aggregator_v2::create_derived_string</a>(name);
        old_name
    } <b>else</b> {
        <b>let</b> old_name = <a href="token.md#0x4_token">token</a>.name;
        <a href="token.md#0x4_token">token</a>.name = name;
        old_name
    };

    <a href="../../endless-framework/doc/event.md#0x1_event_emit">event::emit</a>(
        <a href="token.md#0x4_token_MutationEvent">MutationEvent</a> {
            mutated_field_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"name"),
            old_value: old_name,
            new_value: name
        },
    );
}
</code></pre>



</details>

<a id="0x4_token_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_set_uri">set_uri</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">token::MutatorRef</a>, uri: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="token.md#0x4_token_set_uri">set_uri</a>(mutator_ref: &<a href="token.md#0x4_token_MutatorRef">MutatorRef</a>, uri: String) <b>acquires</b> <a href="token.md#0x4_token_Token">Token</a> {
    <b>assert</b>!(<a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_length">string::length</a>(&uri) &lt;= <a href="token.md#0x4_token_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="token.md#0x4_token_EURI_TOO_LONG">EURI_TOO_LONG</a>));
    <b>let</b> <a href="token.md#0x4_token">token</a> = <a href="token.md#0x4_token_borrow_mut">borrow_mut</a>(mutator_ref);
    <a href="../../endless-framework/doc/event.md#0x1_event_emit">event::emit</a>(
        <a href="token.md#0x4_token_MutationEvent">MutationEvent</a> {
            mutated_field_name: <a href="../../endless-framework/../endless-stdlib/../move-stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"uri"),
            old_value: <a href="token.md#0x4_token">token</a>.uri,
            new_value: uri,
        },
    );
    <a href="token.md#0x4_token">token</a>.uri = uri;
}
</code></pre>



</details>


[move-book]: https://endless.dev/move/book/SUMMARY
