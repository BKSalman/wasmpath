use component::wasmpath::game::{Host, HostState, Position, State};
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

impl Host for Playground {}

#[async_trait::async_trait]
impl HostState for Playground {
    async fn log_state(&mut self, self_: wasmtime::component::Resource<State>) {
        let grid = self.slab.get(self_.rep() as usize).unwrap();
        println!("{grid:#?}");
    }

    async fn target_position(&mut self, self_: wasmtime::component::Resource<State>) -> Position {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.target_pos()
    }

    async fn player_position(&mut self, self_: wasmtime::component::Resource<State>) -> Position {
        let grid = self.slab.get_mut(self_.rep() as usize).unwrap();
        grid.player_pos()
    }

    async fn adjacent_cells(
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
