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
pub struct UpdatePkI128 {
    pub n: i128,
    pub data: i32,
}

impl __sdk::spacetime_module::InModule for UpdatePkI128 {
    type Module = super::RemoteModule;
}

pub struct UpdatePkI128CallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `update_pk_i128`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait update_pk_i_128 {
    /// Request that the remote module invoke the reducer `update_pk_i128` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_update_pk_i_128`] callbacks.
    fn update_pk_i_128(&self, n: i128, data: i32) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `update_pk_i128`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`UpdatePkI128CallbackId`] can be passed to [`Self::remove_on_update_pk_i_128`]
    /// to cancel the callback.
    fn on_update_pk_i_128(
        &self,
        callback: impl FnMut(&super::EventContext, &i128, &i32) + Send + 'static,
    ) -> UpdatePkI128CallbackId;
    /// Cancel a callback previously registered by [`Self::on_update_pk_i_128`],
    /// causing it not to run in the future.
    fn remove_on_update_pk_i_128(&self, callback: UpdatePkI128CallbackId);
}

impl update_pk_i_128 for super::RemoteReducers {
    fn update_pk_i_128(&self, n: i128, data: i32) -> __anyhow::Result<()> {
        self.imp.call_reducer("update_pk_i128", UpdatePkI128 { n, data })
    }
    fn on_update_pk_i_128(
        &self,
        mut callback: impl FnMut(&super::EventContext, &i128, &i32) + Send + 'static,
    ) -> UpdatePkI128CallbackId {
        UpdatePkI128CallbackId(self.imp.on_reducer::<UpdatePkI128>(
            "update_pk_i128",
            Box::new(move |ctx: &super::EventContext, args: &UpdatePkI128| callback(ctx, &args.n, &args.data)),
        ))
    }
    fn remove_on_update_pk_i_128(&self, callback: UpdatePkI128CallbackId) {
        self.imp.remove_on_reducer::<UpdatePkI128>("update_pk_i128", callback.0)
    }
}
