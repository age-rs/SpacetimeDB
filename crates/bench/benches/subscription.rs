use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spacetimedb::db::relational_db::{open_db, RelationalDB};
use spacetimedb::error::DBError;
use spacetimedb::execution_context::ExecutionContext;
use spacetimedb::host::module_host::{DatabaseTableUpdate, DatabaseUpdate, TableOp};
use spacetimedb::subscription::query::compile_read_only_query;
use spacetimedb::subscription::subscription::ExecutionSet;
use spacetimedb_lib::identity::AuthCtx;
use spacetimedb_primitives::TableId;
use spacetimedb_sats::{product, AlgebraicType, AlgebraicValue, ProductValue};
use tempdir::TempDir;

fn create_table_location(db: &RelationalDB) -> Result<TableId, DBError> {
    let schema = &[
        ("entity_id", AlgebraicType::U64),
        ("chunk_index", AlgebraicType::U64),
        ("x", AlgebraicType::I32),
        ("z", AlgebraicType::I32),
        ("dimension", AlgebraicType::U32),
    ];
    let indexes = &[(0.into(), "entity_id"), (1.into(), "chunk_index"), (2.into(), "x")];
    db.create_table_for_test("location", schema, indexes)
}

fn create_table_footprint(db: &RelationalDB) -> Result<TableId, DBError> {
    let footprint = AlgebraicType::sum([
        ("A", AlgebraicType::unit()),
        ("B", AlgebraicType::unit()),
        ("C", AlgebraicType::unit()),
        ("D", AlgebraicType::unit()),
    ]);
    let schema = &[
        ("entity_id", AlgebraicType::U64),
        ("type", footprint),
        ("owner_entity_id", AlgebraicType::U64),
    ];
    let indexes = &[(0.into(), "entity_id"), (2.into(), "owner_entity_id")];
    db.create_table_for_test("footprint", schema, indexes)
}

fn insert_op(table_id: TableId, table_name: &str, row: ProductValue) -> DatabaseTableUpdate {
    DatabaseTableUpdate {
        table_id,
        table_name: table_name.to_string(),
        ops: vec![TableOp::insert(row)],
    }
}

fn eval(c: &mut Criterion) {
    let tmp_dir = TempDir::new("stdb_test").unwrap();
    let db = open_db(&tmp_dir, false, false).unwrap();

    let lhs = create_table_footprint(&db).unwrap();
    let rhs = create_table_location(&db).unwrap();

    let _ = db.with_auto_commit(&ExecutionContext::default(), |tx| -> Result<(), DBError> {
        // 1M rows
        for entity_id in 0u64..1_000_000 {
            let owner = entity_id % 1_000;
            let footprint = AlgebraicValue::sum(entity_id as u8 % 4, AlgebraicValue::unit());
            let row = product!(entity_id, footprint, owner);
            let _ = db.insert(tx, lhs, row)?;
        }
        Ok(())
    });

    let _ = db.with_auto_commit(&ExecutionContext::default(), |tx| -> Result<(), DBError> {
        // 1000 chunks, 1200 rows per chunk = 1.2M rows
        for chunk_index in 0u64..1_000 {
            for i in 0u64..1200 {
                let entity_id = chunk_index * 1200 + i;
                let x = 0i32;
                let z = 0i32;
                let dimension = 0u32;
                let row = product!(entity_id, chunk_index, x, z, dimension);
                let _ = db.insert(tx, rhs, row)?;
            }
        }
        Ok(())
    });

    let entity_id = 1_200_000u64;
    let chunk_index = 5u64;
    let x = 0i32;
    let z = 0i32;
    let dimension = 0u32;

    let footprint = AlgebraicValue::sum(1, AlgebraicValue::unit());
    let owner = 6u64;

    let new_lhs_row = product!(entity_id, footprint, owner);
    let new_rhs_row = product!(entity_id, chunk_index, x, z, dimension);

    let update = DatabaseUpdate {
        tables: vec![
            insert_op(lhs, "footprint", new_lhs_row),
            insert_op(rhs, "location", new_rhs_row),
        ],
    };

    // To profile this benchmark for 30s
    // samply record -r 10000000 cargo bench --bench=subscription --profile=profiling -- full-scan --exact --profile-time=30
    c.bench_function("full-scan", |b| {
        // Iterate 1M rows.
        let scan = "select * from footprint";
        let auth = AuthCtx::for_testing();
        let tx = db.begin_tx();
        let query = compile_read_only_query(&db, &tx, &auth, scan).unwrap();
        let query: ExecutionSet = query.into();

        b.iter(|| {
            let out = query.eval(&db, &tx, auth).unwrap();
            black_box(out);
        })
    });

    // To profile this benchmark for 30s
    // samply record -r 10000000 cargo bench --bench=subscription --profile=profiling -- full-join --exact --profile-time=30
    c.bench_function("full-join", |b| {
        // Join 1M rows on the left with 12K rows on the right.
        // Note, this should use an index join so as not to read the entire lhs table.
        let join = format!(
            "\
            select footprint.* \
            from footprint join location on footprint.entity_id = location.entity_id \
            where location.chunk_index = {chunk_index}"
        );
        let auth = AuthCtx::for_testing();
        let tx = db.begin_tx();
        let query = compile_read_only_query(&db, &tx, &auth, &join).unwrap();
        let query: ExecutionSet = query.into();

        b.iter(|| {
            let out = query.eval(&db, &tx, AuthCtx::for_testing()).unwrap();
            black_box(out);
        })
    });

    // To profile this benchmark for 30s
    // samply record -r 10000000 cargo bench --bench=subscription --profile=profiling -- incr-select --exact --profile-time=30
    c.bench_function("incr-select", |b| {
        // A passthru executed independently of the database.
        let select_lhs = "select * from footprint";
        let select_rhs = "select * from location";
        let auth = AuthCtx::for_testing();
        let tx = db.begin_tx();
        let query_lhs = compile_read_only_query(&db, &tx, &auth, select_lhs).unwrap();
        let query_rhs = compile_read_only_query(&db, &tx, &auth, select_rhs).unwrap();
        let query = ExecutionSet::from_iter(query_lhs.into_iter().chain(query_rhs));

        b.iter(|| {
            let out = query.eval_incr(&db, &tx, &update, AuthCtx::for_testing()).unwrap();
            black_box(out);
        })
    });

    // To profile this benchmark for 30s
    // samply record -r 10000000 cargo bench --bench=subscription --profile=profiling -- incr-join --exact --profile-time=30
    c.bench_function("incr-join", |b| {
        // Not a passthru - requires reading of database state.
        let join = format!(
            "\
            select footprint.* \
            from footprint join location on footprint.entity_id = location.entity_id \
            where location.chunk_index = {chunk_index}"
        );
        let auth = AuthCtx::for_testing();
        let tx = db.begin_tx();
        let query = compile_read_only_query(&db, &tx, &auth, &join).unwrap();
        let query: ExecutionSet = query.into();

        b.iter(|| {
            let out = query.eval_incr(&db, &tx, &update, AuthCtx::for_testing()).unwrap();
            black_box(out);
        })
    });
}

criterion_group!(benches, eval);
criterion_main!(benches);
