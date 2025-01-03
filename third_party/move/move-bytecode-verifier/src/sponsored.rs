use move_binary_format::{
    errors::{Location, PartialVMError, VMResult},
    CompiledModule,
};

pub fn verify_module(module: &CompiledModule) -> VMResult<()> {
    if module
        .function_defs
        .iter()
        .any(|f| f.sponsored && !f.is_entry)
    {
        return Err(PartialVMError::new(
            move_core_types::vm_status::StatusCode::INVALID_SPONSORED_DECL_WITHOUT_ENTRY,
        )
        .finish(Location::Module(module.self_id())));
    }
    Ok(())
}
