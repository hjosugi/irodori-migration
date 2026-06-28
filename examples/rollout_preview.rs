use irodori_migration::{expand_contract_rollout, shadow_read_runbook, ShadowReadExperiment};

fn main() {
    let rollout = expand_contract_rollout("orders migration");
    for step in rollout.steps {
        println!("{:?}: {}", step.phase, step.title);
        for gate in step.gates {
            println!("  gate: {} -> {}", gate.name, gate.check);
        }
    }

    let experiment = ShadowReadExperiment::new(
        "orders read parity",
        "select * from legacy.orders where order_id = ?",
        "select * from analytics.orders where order_id = ?",
        vec!["order_id".into()],
    )
    .with_sample_percent(5);

    println!("\n{}", shadow_read_runbook(&experiment));
}
