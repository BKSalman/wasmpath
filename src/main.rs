use std::time::Duration;

use anyhow::Context;
use macroquad::prelude::*;
use wasmpath::timer;
use wasmpath::{grid::Grid, Main, Playground};
use wasmtime::component::{Component, Linker, Resource};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::WasiCtxBuilder;

#[macroquad::main("WASM path")]
async fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let wasm_file = args
        .next()
        .ok_or(anyhow::anyhow!("ERROR: Path to wasm file was not provided"))?;
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);
    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;

    Main::add_to_linker(&mut linker, |s: &mut Playground| s)
        .with_context(|| format!("Failed to link component imports"))?;

    let wasi_ctx = WasiCtxBuilder::new()
        .allow_ip_name_lookup(false)
        .allow_tcp(false)
        .allow_udp(false)
        .build();

    let mut playground = Playground::new(wasi_ctx);

    let grid = playground.slab.insert(Grid::new(5, 5));

    let mut store = Store::new(&engine, playground);
    let component = Component::from_file(&engine, wasm_file.clone())?;

    let (component, _) = Main::instantiate_async(&mut store, &component, &linker).await?;

    let mut timer = timer::Timer::new(Duration::from_secs(1));

    loop {
        let reached = store.data().slab.get(grid).is_some_and(|g| g.has_reached());
        clear_background(WHITE);

        if let Some(grid) = store.data().slab.get(grid) {
            let height = grid.rows() as f32 * 55.;
            let width = grid.columns() as f32 * 55.;
            let centered_width = (screen_width() / 2.) - width / 2.;
            let centered_height = (screen_height() / 2.) - height / 2.;
            for i in 0..grid.rows() * grid.columns() {
                let player_pos = grid.player_pos();
                let player_index =
                    player_pos.row as usize * grid.columns() + player_pos.column as usize;
                let target_pos = grid.target_pos();
                let target_index =
                    target_pos.row as usize * grid.columns() + target_pos.column as usize;
                if player_index == i {
                    draw_rectangle(
                        (55. * (i % grid.columns()) as f32) + centered_width,
                        55. * (i / grid.rows()) as f32 + centered_height,
                        50.,
                        50.,
                        BLUE,
                    );
                    // draw_text(
                    //     &i.to_string(),
                    //     55. * (i % grid.columns()) as f32 + centered_width,
                    //     30. + (50. * (i / grid.rows()) as f32) + centered_height,
                    //     20.,
                    //     BLACK,
                    // );
                } else if target_index == i {
                    draw_rectangle(
                        (55. * (i % grid.columns()) as f32) + centered_width,
                        55. * (i / grid.rows()) as f32 + centered_height,
                        50.,
                        50.,
                        RED,
                    );
                    // draw_text(
                    //     &i.to_string(),
                    //     55. * (i % grid.columns()) as f32 + centered_width,
                    //     30. + (50. * (i / grid.rows()) as f32) + centered_height,
                    //     20.,
                    //     BLACK,
                    // );
                } else {
                    draw_rectangle(
                        (55. * (i % grid.columns()) as f32) + centered_width,
                        55. * (i / grid.rows()) as f32 + centered_height,
                        50.,
                        50.,
                        LIGHTGRAY,
                    );
                    // draw_text(
                    //     &i.to_string(),
                    //     55. * (i % grid.columns()) as f32 + centered_width,
                    //     30. + (50. * (i / grid.rows()) as f32) + centered_height,
                    //     20.,
                    //     BLACK,
                    // );
                }
            }
        }

        if !reached && timer.is_finished() {
            component
                .call_step(&mut store, Resource::new_borrow(grid as u32))
                .await?;
            timer.reset();
            timer.start();
        }

        if reached {
            let text_size = measure_text("You reached the goal!", None, 20, 1.);
            draw_text(
                "You reached the goal!",
                (screen_width() / 2.) - (text_size.width / 2.),
                (screen_height() / 2.) - (text_size.height / 2.),
                20.,
                BLACK,
            );
        }

        timer.tick();
        next_frame().await
    }
}
