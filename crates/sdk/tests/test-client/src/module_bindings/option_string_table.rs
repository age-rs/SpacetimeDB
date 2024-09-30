// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::option_string_type::OptionString;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `option_string`.
///
/// Obtain a handle from the [`OptionStringTableAccess::option_string`] method on [`super::RemoteTables`],
/// like `ctx.db.option_string()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.option_string().on_insert(...)`.
pub struct OptionStringTableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<OptionString>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `option_string`.
///
/// Implemented for [`super::RemoteTables`].
pub trait OptionStringTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`OptionStringTableHandle`], which mediates access to the table `option_string`.
    fn option_string(&self) -> OptionStringTableHandle<'_>;
}

impl OptionStringTableAccess for super::RemoteTables {
    fn option_string(&self) -> OptionStringTableHandle<'_> {
        OptionStringTableHandle {
            imp: self.imp.get_table::<OptionString>("option_string"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct OptionStringInsertCallbackId(__sdk::callbacks::CallbackId);
pub struct OptionStringDeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for OptionStringTableHandle<'ctx> {
    type Row = OptionString;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = OptionString> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = OptionStringInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionStringInsertCallbackId {
        OptionStringInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: OptionStringInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = OptionStringDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> OptionStringDeleteCallbackId {
        OptionStringDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: OptionStringDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<OptionString>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"option_string\"")
}
