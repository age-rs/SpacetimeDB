// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct InsertVecStringArgs {
    pub s: Vec<String>,
}

impl From<InsertVecStringArgs> for super::Reducer {
    fn from(args: InsertVecStringArgs) -> Self {
        Self::InsertVecString { s: args.s }
    }
}

impl __sdk::InModule for InsertVecStringArgs {
    type Module = super::RemoteModule;
}

pub struct InsertVecStringCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `insert_vec_string`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait insert_vec_string {
    /// Request that the remote module invoke the reducer `insert_vec_string` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_insert_vec_string`] callbacks.
    fn insert_vec_string(&self, s: Vec<String>) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `insert_vec_string`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`InsertVecStringCallbackId`] can be passed to [`Self::remove_on_insert_vec_string`]
    /// to cancel the callback.
    fn on_insert_vec_string(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &Vec<String>) + Send + 'static,
    ) -> InsertVecStringCallbackId;
    /// Cancel a callback previously registered by [`Self::on_insert_vec_string`],
    /// causing it not to run in the future.
    fn remove_on_insert_vec_string(&self, callback: InsertVecStringCallbackId);
}

impl insert_vec_string for super::RemoteReducers {
    fn insert_vec_string(&self, s: Vec<String>) -> __sdk::Result<()> {
        self.imp.call_reducer("insert_vec_string", InsertVecStringArgs { s })
    }
    fn on_insert_vec_string(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &Vec<String>) + Send + 'static,
    ) -> InsertVecStringCallbackId {
        InsertVecStringCallbackId(self.imp.on_reducer(
            "insert_vec_string",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::InsertVecString { s },
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
    fn remove_on_insert_vec_string(&self, callback: InsertVecStringCallbackId) {
        self.imp.remove_on_reducer("insert_vec_string", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `insert_vec_string`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_insert_vec_string {
    /// Set the call-reducer flags for the reducer `insert_vec_string` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn insert_vec_string(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_insert_vec_string for super::SetReducerFlags {
    fn insert_vec_string(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("insert_vec_string", flags);
    }
}
