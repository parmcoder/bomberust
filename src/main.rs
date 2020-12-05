use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    audio::DjSystemDesc
};
use amethyst::renderer::RenderDebugLines;

mod state;
mod audio;
mod constants;
mod entities;
mod events;
mod systems;

use crate::{
    systems::{RenderSystem, PieceSpawnSystem, PieceInputSystem, LineClearSystem, DroppingSystem}
};
use crate::audio::Music;
use amethyst::audio::AudioBundle;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    /* Here is how we make everything run together.
    with_bundle will take all the systems and run those systems in parallel
    */
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(AudioBundle::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderDebugLines::default()),

        )?.with_bundle(UiBundle::<StringBindings>::new())?
        .with(
            PieceInputSystem::new(),
            "piece_input_system",
            &["input_system"],
        )
        .with(DroppingSystem::new(), "piece_drop_system", &[])
        .with(PieceSpawnSystem::new(), "piece_spawn_system", &[])
        .with(LineClearSystem::new(), "line_clear_system", &[])
        .with(RenderSystem, "render_system", &[])
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        );

    let mut game = Application::new(resources, state::GameState, game_data)?;
    game.run();

    Ok(())
}
