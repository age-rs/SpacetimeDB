// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use super::unique_u_32_type::UniqueU32;
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

/// Table handle for the table `unique_u32`.
///
/// Obtain a handle from the [`UniqueU32TableAccess::unique_u_32`] method on [`super::RemoteTables`],
/// like `ctx.db.unique_u_32()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_u_32().on_insert(...)`.
pub struct UniqueU32TableHandle<'ctx> {
    imp: __sdk::db_connection::TableHandle<UniqueU32>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `unique_u32`.
///
/// Implemented for [`super::RemoteTables`].
pub trait UniqueU32TableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`UniqueU32TableHandle`], which mediates access to the table `unique_u32`.
    fn unique_u_32(&self) -> UniqueU32TableHandle<'_>;
}

impl UniqueU32TableAccess for super::RemoteTables {
    fn unique_u_32(&self) -> UniqueU32TableHandle<'_> {
        UniqueU32TableHandle {
            imp: self.imp.get_table::<UniqueU32>("unique_u32"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct UniqueU32InsertCallbackId(__sdk::callbacks::CallbackId);
pub struct UniqueU32DeleteCallbackId(__sdk::callbacks::CallbackId);

impl<'ctx> __sdk::table::Table for UniqueU32TableHandle<'ctx> {
    type Row = UniqueU32;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = UniqueU32> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = UniqueU32InsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueU32InsertCallbackId {
        UniqueU32InsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: UniqueU32InsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = UniqueU32DeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> UniqueU32DeleteCallbackId {
        UniqueU32DeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: UniqueU32DeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn parse_table_update(
    deletes: Vec<__ws::EncodedValue>,
    inserts: Vec<__ws::EncodedValue>,
) -> __anyhow::Result<__sdk::spacetime_module::TableUpdate<UniqueU32>> {
    __sdk::spacetime_module::TableUpdate::parse_table_update_no_primary_key(deletes, inserts)
        .context("Failed to parse table update for table \"unique_u32\"")
}

/// Access to the `n` unique index on the table `unique_u32`,
/// which allows point queries on the field of the same name
/// via the [`UniqueU32NUnique::find`] method.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.unique_u_32().n().find(...)`.
pub struct UniqueU32NUnique<'ctx> {
    imp: __sdk::client_cache::UniqueConstraint<UniqueU32, u32>,
    phantom: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

impl<'ctx> UniqueU32TableHandle<'ctx> {
    /// Get a handle on the `n` unique index on the table `unique_u32`.
    pub fn n(&self) -> UniqueU32NUnique<'ctx> {
        UniqueU32NUnique {
            imp: self.imp.get_unique_constraint::<u32>("n", |row| &row.n),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'ctx> UniqueU32NUnique<'ctx> {
    /// Find the subscribed row whose `n` column value is equal to `col_val`,
    /// if such a row is present in the client cache.
    pub fn find(&self, col_val: &u32) -> Option<UniqueU32> {
        self.imp.find(col_val)
    }
}
