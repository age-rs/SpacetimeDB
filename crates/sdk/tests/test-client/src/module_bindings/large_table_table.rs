// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use super::byte_struct_type::ByteStruct;
use super::enum_with_payload_type::EnumWithPayload;
use super::every_primitive_struct_type::EveryPrimitiveStruct;
use super::every_vec_struct_type::EveryVecStruct;
use super::large_table_type::LargeTable;
use super::simple_enum_type::SimpleEnum;
use super::unit_struct_type::UnitStruct;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

/// Table handle for the table `large_table`.
///
/// Obtain a handle from the [`LargeTableTableAccess::large_table`] method on [`super::RemoteTables`],
/// like `ctx.db.large_table()`.
///
/// Users are encouraged not to explicitly reference this type,
/// but to directly chain method calls,
/// like `ctx.db.large_table().on_insert(...)`.
pub struct LargeTableTableHandle<'ctx> {
    imp: __sdk::TableHandle<LargeTable>,
    ctx: std::marker::PhantomData<&'ctx super::RemoteTables>,
}

#[allow(non_camel_case_types)]
/// Extension trait for access to the table `large_table`.
///
/// Implemented for [`super::RemoteTables`].
pub trait LargeTableTableAccess {
    #[allow(non_snake_case)]
    /// Obtain a [`LargeTableTableHandle`], which mediates access to the table `large_table`.
    fn large_table(&self) -> LargeTableTableHandle<'_>;
}

impl LargeTableTableAccess for super::RemoteTables {
    fn large_table(&self) -> LargeTableTableHandle<'_> {
        LargeTableTableHandle {
            imp: self.imp.get_table::<LargeTable>("large_table"),
            ctx: std::marker::PhantomData,
        }
    }
}

pub struct LargeTableInsertCallbackId(__sdk::CallbackId);
pub struct LargeTableDeleteCallbackId(__sdk::CallbackId);

impl<'ctx> __sdk::Table for LargeTableTableHandle<'ctx> {
    type Row = LargeTable;
    type EventContext = super::EventContext;

    fn count(&self) -> u64 {
        self.imp.count()
    }
    fn iter(&self) -> impl Iterator<Item = LargeTable> + '_ {
        self.imp.iter()
    }

    type InsertCallbackId = LargeTableInsertCallbackId;

    fn on_insert(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> LargeTableInsertCallbackId {
        LargeTableInsertCallbackId(self.imp.on_insert(Box::new(callback)))
    }

    fn remove_on_insert(&self, callback: LargeTableInsertCallbackId) {
        self.imp.remove_on_insert(callback.0)
    }

    type DeleteCallbackId = LargeTableDeleteCallbackId;

    fn on_delete(
        &self,
        callback: impl FnMut(&Self::EventContext, &Self::Row) + Send + 'static,
    ) -> LargeTableDeleteCallbackId {
        LargeTableDeleteCallbackId(self.imp.on_delete(Box::new(callback)))
    }

    fn remove_on_delete(&self, callback: LargeTableDeleteCallbackId) {
        self.imp.remove_on_delete(callback.0)
    }
}

#[doc(hidden)]
pub(super) fn register_table(client_cache: &mut __sdk::ClientCache<super::RemoteModule>) {
    let _table = client_cache.get_or_make_table::<LargeTable>("large_table");
}
#[doc(hidden)]
pub(super) fn parse_table_update(
    raw_updates: __ws::TableUpdate<__ws::BsatnFormat>,
) -> __sdk::Result<__sdk::TableUpdate<LargeTable>> {
    __sdk::TableUpdate::parse_table_update_no_primary_key(raw_updates).map_err(|e| {
        __sdk::InternalError::failed_parse("TableUpdate<LargeTable>", "TableUpdate")
            .with_cause(e)
            .into()
    })
}
