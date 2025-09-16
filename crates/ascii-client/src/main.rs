use bracket_terminal::prelude::*;
use womporio_core::{SimConfig, SimWorld};

struct State {
    sim: SimWorld,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let snapshot = self.sim.snapshot();
        ctx.print(1, 1, "Womporio ASCII Prototype");
        ctx.print(1, 3, format!("Tick: {}", snapshot.tick));
        if let Some(machine) = snapshot.machines.first() {
            ctx.print(1, 5, format!("Machine inventory:"));
            let mut row = 6;
            for (item, quantity) in machine.inventory.iter() {
                ctx.print(2, row, format!("{} x{}", item.as_str(), quantity));
                row += 1;
            }
        }
        self.sim.step();
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Womporio ASCII Preview")
        .build()?;

    let state = State {
        sim: SimWorld::new(SimConfig::default()),
    };

    main_loop(context, state)
}
