use irodori_migration::{
    build_migration_plan, MigrationEngine, MigrationExportFormat, MigrationSpec,
};

fn main() {
    let plan = build_migration_plan(&MigrationSpec {
        source_engine: MigrationEngine::Hive,
        target_engine: MigrationEngine::Snowflake,
        source_table: "legacy.orders".into(),
        target_table: "analytics.orders".into(),
        key_columns: vec!["order_id".into()],
        compare_columns: vec!["order_id".into(), "amount".into(), "updated_at".into()],
        export_format: MigrationExportFormat::Parquet,
        ..MigrationSpec::default()
    });

    println!("{}", plan.title);
    for task in &plan.tasks {
        println!("- {:?}: {}", task.level, task.title);
    }
    println!("\n{}", plan.diff_sql);
}
