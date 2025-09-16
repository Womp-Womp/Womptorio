use womporio_core::{SimConfig, SimWorld};

#[test]
fn simulation_advances_ticks_and_recipes() {
    let mut sim = SimWorld::new(SimConfig::default());
    sim.run_for_ticks(10);

    let snapshot = sim.snapshot();
    assert_eq!(snapshot.tick, 10);

    let machine = snapshot
        .machines
        .first()
        .expect("seed world should spawn a machine");

    let mut ore = 0;
    let mut plates = 0;
    for (item, quantity) in machine.inventory.iter() {
        match item.as_str() {
            "iron-ore" => ore = *quantity,
            "iron-plate" => plates = *quantity,
            _ => {}
        }
    }

    assert!(plates > 0, "machine should have crafted at least one plate");
    assert!(ore < 8, "machine should have consumed some ore");
}
