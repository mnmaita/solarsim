use bevy::{
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, TrackClick},
};

const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn slider(min: f32, max: f32, value: f32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            justify_items: JustifyItems::Center,
            column_gap: px(4),
            height: px(12),
            width: percent(30),
            ..default()
        },
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderValue(value),
        SliderRange::new(min, max),
        TabIndex(0),
        Children::spawn((
            // Slider background rail
            Spawn((
                Node {
                    height: px(6),
                    ..default()
                },
                // Border color for the slider
                BackgroundColor(SLIDER_TRACK),
                BorderRadius::all(px(3)),
            )),
            // Invisible track to allow absolute placement of thumb entity. This is narrower than
            // the actual slider, which allows us to position the thumb entity using simple
            // percentages, without having to measure the actual width of the slider thumb.
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    // Track is short by 12px to accommodate the thumb.
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        // This will be updated by the slider's value
                        left: percent(0),
                        ..default()
                    },
                    BorderRadius::MAX,
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}
