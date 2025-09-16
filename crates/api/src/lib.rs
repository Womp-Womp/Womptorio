use anyhow::Result;
use mlua::{Function, Lua, Table};
use womporio_core::tick::TimeState;

/// Lua execution host used for mod scripting.
pub struct ScriptHost {
    lua: Lua,
}

impl ScriptHost {
    pub fn new() -> Result<Self> {
        Ok(Self { lua: Lua::new() })
    }

    /// Loads and executes an `on_tick` callback.
    pub fn run_on_tick(&self, source: &str, ctx: &ScriptContext) -> Result<()> {
        let func: Function = self.lua.load(source).eval()?;
        let table = self.build_context_table(ctx)?;
        func.call::<_, ()>(table)?;
        Ok(())
    }

    fn build_context_table(&self, ctx: &ScriptContext) -> Result<Table<'_>> {
        let table = self.lua.create_table()?;
        table.set("tick", ctx.tick)?;
        table.set("delta", ctx.delta)?;
        Ok(table)
    }
}

/// Lightweight context exposed to scripts each tick.
#[derive(Debug, Clone)]
pub struct ScriptContext {
    pub tick: u64,
    pub delta: u32,
}

impl ScriptContext {
    pub fn from_time(time: &TimeState) -> Self {
        Self {
            tick: time.tick,
            delta: time.fixed_time_step,
        }
    }
}
