use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiImage, UiText,
        UiTransform,
    },
    window::ScreenDimensions,
};

use crate::audio::initialise_audio;
use crate::constants::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::entities::{Piece, PieceType, Position};
use crate::events::PieceLandEvent;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::renderer::debug_drawing::DebugLinesComponent;
use log::info;

#[derive(Default)]
pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        //Let's start with I piece everytime
        let mut b = Piece::new(PieceType::I);
        b.rotation = 3;
        world
            .create_entity()
            .with(b)
            .with(Position {
                row: BOARD_HEIGHT as i8 - 4,
                col: 3,
            })
            .build();

        // Setup debug lines as a component and add lines to render axes & grid
        let debug_lines_component = DebugLinesComponent::new();
        world.register::<DebugLinesComponent>();
        world.create_entity().with(debug_lines_component).build();

        // Like I said, data-driven means you have to set up a place for data to store.
        // We put the event channel once the state is run.
        let mut land_channel = EventChannel::<PieceLandEvent>::new();
        land_channel.single_write(PieceLandEvent {});
        world.insert(land_channel);

        // Also, setting up this camera is necessary, it is quite difficult to do it from scratch.
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            BOARD_WIDTH as f32 * 0.5 + 2.0,
            BOARD_HEIGHT as f32 * 0.5,
            1.0,
        );
        world
            .create_entity()
            .with(Camera::standard_2d(
                (BOARD_WIDTH + 4) as f32,
                BOARD_HEIGHT as f32,
            ))
            .with(transform)
            .build();

        // Make sure that we have loaded the assets.
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "sprites/tetriminos/tetris_block.png",
                ImageFormat::default(),
                (),
                &world.read_resource::<AssetStorage<Texture>>(),
            )
        };

        let spritesheet_handle = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "sprites/tetriminos/sprites.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &world.read_resource::<AssetStorage<SpriteSheet>>(),
            )
        };
        world.insert(spritesheet_handle);
        initialise_audio(world);
        create_ui(world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}

/// Creates a simple UI background and a UI text label
/// This is the pure code only way to create UI with amethyst.
pub fn create_ui(world: &mut World) {
    // this creates the simple pink background UI element.
    let ui_background = world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            430.0,
            0.,
            0.,
            250.,
            800.,
        ))
        .build();

    // This simply loads a font from the asset folder and puts it in the world as a resource,
    // we also get a ref to the font that we then can pass to the text label we crate later.
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // This creates the actual label and places it on the screen.
    // Take note of the z position given, this ensures the label gets rendered above the background UI element.
    world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            440.0,
            -40.,
            1.,
            200.,
            60.,
        ))
        .with(UiText::new(
            font.clone(),
            "RUSTRIS".to_string(),
            [1., 0.5, 1., 1.],
            55.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();

    for (x, y) in [
        (-200.0, "How to play"),
        (-240.0, "WASD - move"),
        (-280.0, "K - rotate ccw"),
        (-320.0, "J - rotate cw"),
    ]
    .iter()
    {
        world
            .create_entity()
            .with(UiTransform::new(
                "".to_string(),
                Anchor::TopLeft,
                Anchor::TopLeft,
                440.0,
                x.clone() + 30.,
                1.,
                200.,
                500.,
            ))
            .with(UiText::new(
                font.clone(),
                y.clone().to_string(),
                [1., 1., 1., 1.],
                25.,
                LineMode::Single,
                Anchor::TopLeft,
            ))
            .build();
    }
}
