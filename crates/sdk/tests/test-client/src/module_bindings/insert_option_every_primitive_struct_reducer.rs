// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

use super::every_primitive_struct_type::EveryPrimitiveStruct;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertOptionEveryPrimitiveStructArgs {
    pub s: Option<EveryPrimitiveStruct>,
}

impl From<InsertOptionEveryPrimitiveStructArgs> for super::Reducer {
    fn from(args: InsertOptionEveryPrimitiveStructArgs) -> Self {
        Self::InsertOptionEveryPrimitiveStruct { s: args.s }
    }
}

impl __sdk::InModule for InsertOptionEveryPrimitiveStructArgs {
    type Module = super::RemoteModule;
}

pub struct InsertOptionEveryPrimitiveStructCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_option_every_primitive_struct`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_option_every_primitive_struct {
    /// Request that the remote module invoke the reducer `insert_option_every_primitive_struct` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_option_every_primitive_struct`] callbacks.
    fn insert_option_every_primitive_struct(&self, s: Option<EveryPrimitiveStruct>) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_option_every_primitive_struct`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertOptionEveryPrimitiveStructCallbackId`] can be passed to [`Self::remove_on_insert_option_every_primitive_struct`]
    /// to cancel the callback.
    fn on_insert_option_every_primitive_struct(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &Option<EveryPrimitiveStruct>) + Send + 'static,
    ) -> InsertOptionEveryPrimitiveStructCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_option_every_primitive_struct`],
    /// causing it not to run in the future.
    fn remove_on_insert_option_every_primitive_struct(&self, callback: InsertOptionEveryPrimitiveStructCallbackId);
}

impl insert_option_every_primitive_struct for super::RemoteReducers {
    fn insert_option_every_primitive_struct(&self, s: Option<EveryPrimitiveStruct>) -> __sdk::Result<()> {
        self.imp.call_reducer(
            "insert_option_every_primitive_struct",
            InsertOptionEveryPrimitiveStructArgs { s },
        )
    }
    fn on_insert_option_every_primitive_struct(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &Option<EveryPrimitiveStruct>) + Send + 'static,
    ) -> InsertOptionEveryPrimitiveStructCallbackId {
        InsertOptionEveryPrimitiveStructCallbackId(self.imp.on_reducer(
            "insert_option_every_primitive_struct",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::InsertOptionEveryPrimitiveStruct { s },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, s)
            }),
        ))
    }
    fn remove_on_insert_option_every_primitive_struct(&self, callback: InsertOptionEveryPrimitiveStructCallbackId) {
        self.imp
            .remove_on_reducer("insert_option_every_primitive_struct", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_option_every_primitive_struct`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_option_every_primitive_struct {
    /// Set the call-reducer flags for the reducer `insert_option_every_primitive_struct` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_option_every_primitive_struct(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_option_every_primitive_struct for super::SetReducerFlags {
    fn insert_option_every_primitive_struct(&self, flags: __ws::CallReducerFlags) {
        self.imp
            .set_call_reducer_flags("insert_option_every_primitive_struct", flags);
    }
}
