/// Spawn foo at given position.
/// `foo_type_name` corresponds to the `foo_types` defined in Bar config.
/// `foo_type_name` can be `nullptr`, in which case, random foo is assigned to the new agent.
///
/// Returns invalid `FooHandle` in case of error.
#[no_mangle]
pub unsafe extern "C" fn bar_add_foo_at_position(
    bar: *mut Bar,
    transform: DTransform,
    foo_type_name: *const String,
) -> FooHandle {
    if bar.is_null() {
        return FooHandle::new_invalid();
    }

    ffi_helpers::catch_panic(|| {
        let foo_type_name_opt =
            foo_type_name
                .as_ref()
                .cloned()
                .and_then(|s| if s.is_empty() { None } else { Some(s) });

        Ok((*bar)
            .add_foo_at_position(transform, foo_type_name_opt)
            .sync()?)
    })
    .unwrap_or_else(|_| FooHandle::new_invalid())
}

/// Get n-th bar information. Use this function (and `foo_bars_count`),
/// to iterate over all bars in "FOO".
///
/// `bar_index` must be below value returned from `foo_bars_count`.
///
/// Returns error code when invalid input provided.
#[no_mangle]
pub unsafe extern "C" fn foo_bar_by_num(
    foo: *const FOO,
    bar_index: size_t,
    out_bar: *mut Bar,
) -> ResultCode {
    null_pointer_check!(foo, -1);
    catch_panic_return_negative! {
        *out_bar = (*foo).bar_by_num(bar_index as usize).into();
        Ok(0)
    }
}

/// Get current `BarState` given `Id` of `Foo`.
/// You can get Ids of all bars by iterating over them in `FOO`
/// (using `foo_bars_count` and `foo_bar_by_num`).
///
/// Returns error code, when invalid input provided.
#[no_mangle]
pub unsafe extern "C" fn auxiliary_object_name(
    auxiliary_object: *const AuxiliaryObject,
    foo: *const FOO,
) -> *mut String {
    null_pointer_check!(auxiliary_object);
    null_pointer_check!(foo);
    catch_panic! {
        let name = &(*foo).auxiliary_object_by_id(Id::from_raw((*auxiliary_object).id)).name;
        Ok(Box::into_raw(Box::new(name.clone())))
    }
}
