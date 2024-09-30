// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct DeleteUniqueU32 {
    pub n: u32,
}

impl __sdk::spacetime_module::InModule for DeleteUniqueU32 {
    type Module = super::RemoteModule;
}

pub struct DeleteUniqueU32CallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `delete_unique_u32`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait delete_unique_u_32 {
    /// Request that the remote module invoke the reducer `delete_unique_u32` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_delete_unique_u_32`] callbacks.
    fn delete_unique_u_32(&self, n: u32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `delete_unique_u32`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`DeleteUniqueU32CallbackId`] can be passed to [`Self::remove_on_delete_unique_u_32`]
    /// to cancel the callback.
    fn on_delete_unique_u_32(
        &self,
        callback: impl FnMut(&super::EventContext, &u32) + Send + 'static,
    ) -> DeleteUniqueU32CallbackId;
    /// Cancel a callback previously registered by [`Self::on_delete_unique_u_32`],
    /// causing it not to run in the future.
    fn remove_on_delete_unique_u_32(&self, callback: DeleteUniqueU32CallbackId);
}

impl delete_unique_u_32 for super::RemoteReducers {
    fn delete_unique_u_32(&self, n: u32) -> __anyhow::Result<()> {
        self.imp.call_reducer("delete_unique_u32", DeleteUniqueU32 { n })
    }
    fn on_delete_unique_u_32(
        &self,
        mut callback: impl FnMut(&super::EventContext, &u32) + Send + 'static,
    ) -> DeleteUniqueU32CallbackId {
        DeleteUniqueU32CallbackId(self.imp.on_reducer::<DeleteUniqueU32>(
            "delete_unique_u32",
            Box::new(move |ctx: &super::EventContext, args: &DeleteUniqueU32| callback(ctx, &args.n)),
        ))
    }
    fn remove_on_delete_unique_u_32(&self, callback: DeleteUniqueU32CallbackId) {
        self.imp
            .remove_on_reducer::<DeleteUniqueU32>("delete_unique_u32", callback.0)
    }
}
