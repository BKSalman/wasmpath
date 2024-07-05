use grid::Grid;
use wasmtime::component::*;
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiView};

pub mod grid;
#[cfg(test)]
mod tests;
pub mod timer;

bindgen!({
    async: true,
    additional_derives: [
        PartialEq,
        Eq,
    ],
});

#[async_trait::async_trait]
impl wasi::logging::logging::Host for Playground {
    async fn log(
        &mut self,
        _level: wasi::logging::logging::Level,
        context: String,
        message: String,
    ) {
        if !context.is_empty() {
            println!("{context}:: {message}");
        } else {
            println!("{message}");
        }
    }
}

#[async_trait::async_trait]
impl HostState for Playground {
    async fn log_state(&mut self, self_: wasmtime::component::Resource<State>) {
        let grid = self.slab.get(self_.rep() as usize).unwrap();
        println!("{grid:?}");
    }

    async fn move_up(&mut self, self_: wasmtime::component::Resource<State>) {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.move_up();
    }

    async fn move_down(&mut self, self_: wasmtime::component::Resource<State>) {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.move_down();
    }

    async fn move_left(&mut self, self_: wasmtime::component::Resource<State>) {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.move_left();
    }

    async fn move_right(&mut self, self_: wasmtime::component::Resource<State>) {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.move_right();
    }

    async fn target_position(&mut self, self_: wasmtime::component::Resource<State>) -> Position {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.target_pos()
    }

    async fn player_position(&mut self, self_: wasmtime::component::Resource<State>) -> Position {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.player_pos()
    }

    async fn adjacent(
        &mut self,
        self_: wasmtime::component::Resource<State>,
        cell: Position,
    ) -> Vec<Position> {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.adjacent(&cell)
    }

    fn drop(&mut self, _rep: wasmtime::component::Resource<State>) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl MainImports for Playground {}

pub struct Playground {
    pub ctx: WasiCtx,
    pub table: ResourceTable,
    pub slab: slab::Slab<Grid>,
}

impl Playground {
    pub fn new(wasi_ctx: WasiCtx) -> Self {
        Self {
            ctx: wasi_ctx,
            table: ResourceTable::new(),
            slab: slab::Slab::new(),
        }
    }
}

impl WasiView for Playground {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
