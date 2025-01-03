module 0x42::M {
    struct A has key {
        key: auth,
    }

    fun f1(key: auth) {
        let _key = copy key;
    }
    fun f2(caller: &signer, _key: &auth) {
        let a: auth = 1;
        let b: auth = @0xcafe;
        let c: auth = caller;
    }
    fun auth() {}
    #[view]
    fun view_fn(key: auth, a: u64) {}
}

script { fun auth() {} }