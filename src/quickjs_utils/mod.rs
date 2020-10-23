use crate::quickjsruntime::QuickJsRuntime;

pub mod arrays;
pub mod atoms;
pub mod bigints;
pub mod dates;
pub mod errors;
pub mod functions;
pub mod json;
pub mod modules;
pub mod objects;
pub mod primitives;
pub mod promises;
pub mod reflection;
pub mod typedarrays;

use crate::eserror::EsError;
use crate::quickjs_utils::objects::get_property;
use crate::valueref::{JSValueRef, TAG_NULL, TAG_UNDEFINED};
use libquickjs_sys as q;

// todo
// runtime and context in thread_local here
// all function (where applicable) get an Option<QuickJSRuntime> which if None will be gotten from the thread_local
// every function which returns a q::JSValue will return a OwnedValueRef to ensure values are freed on drop

pub fn gc(q_js_rt: &QuickJsRuntime) {
    log::trace!("GC called");
    unsafe { q::JS_RunGC(q_js_rt.runtime) }
    log::trace!("GC done");
}

pub fn new_undefined_ref() -> JSValueRef {
    JSValueRef::new(
        q::JSValue {
            u: q::JSValueUnion { int32: 0 },
            tag: TAG_UNDEFINED,
        },
        false,
        false,
        "new_undefined_ref",
    )
}

pub fn new_null() -> q::JSValue {
    q::JSValue {
        u: q::JSValueUnion { int32: 0 },
        tag: TAG_NULL,
    }
}

pub fn new_null_ref() -> JSValueRef {
    JSValueRef::new(new_null(), false, false, "new_null_ref")
}

pub fn get_global(q_js_rt: &QuickJsRuntime) -> JSValueRef {
    let global = unsafe { q::JS_GetGlobalObject(q_js_rt.context) };
    JSValueRef::new(global, false, true, "global")
}

pub fn get_constructor(
    q_js_rt: &QuickJsRuntime,
    constructor_name: &str,
) -> Result<JSValueRef, EsError> {
    let global_ref = get_global(q_js_rt);

    let constructor_ref = get_property(q_js_rt, &global_ref, constructor_name)?;

    Ok(constructor_ref)
}

/// # Safety
/// be safe
pub unsafe fn parse_args(argc: ::std::os::raw::c_int, argv: *mut q::JSValue) -> Vec<JSValueRef> {
    let arg_slice = std::slice::from_raw_parts(argv, argc as usize);
    arg_slice
        .iter()
        .map(|raw| JSValueRef::new(*raw, false, false, "quickjs_utils::parse_args"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
pub mod tests {
    use crate::esruntime::EsRuntime;
    use crate::quickjs_utils::get_global;
    use std::sync::Arc;

    #[test]
    fn test_global() {
        let rt: Arc<EsRuntime> = crate::esruntime::tests::TEST_ESRT.clone();
        let _io = rt.add_to_event_queue_sync(|q_js_rt| {
            let ct = get_global(q_js_rt).get_ref_count();
            for _ in 0..5 {
                let global = get_global(q_js_rt);
                assert_eq!(global.get_ref_count(), ct);
            }
        });
    }
}
