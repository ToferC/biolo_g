use bevy::prelude::*;

use crate::components::human::{cardiovascular::Heart, respiratory::Lungs};

#[derive(Component)]
pub struct HeartText;

#[derive(Component)]
pub struct BreathText;

pub fn ui_text_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
            "Heart Rate: ",
            TextStyle { 
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::WHITE,
        },
        ),
        TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            color: Color::GOLD,
        }),
        ])
        .with_text_alignment(TextAlignment::TOP_LEFT)
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(100.0),
                top: Val::Px(200.0),
                bottom: Val::Px(100.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
    .insert(HeartText);

    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
            "Breathing Rate: ",
            TextStyle { 
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                color: Color::WHITE,
        },
        ),
        TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            color: Color::AQUAMARINE,
        }),
        ])
        .with_text_alignment(TextAlignment::TOP_LEFT)
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(100.0),
                top: Val::Px(200.0),
                bottom: Val::Px(150.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
    .insert(BreathText);
}

pub fn breath_text_update(
    mut query: Query<&mut Text, With<BreathText>>,
    lungs: Query<&Lungs>,
) {
    let lungs = lungs.single();

    let breathing_rate = lungs.rate * 60.;

    for mut text in &mut query {
        text.sections[1].value = format!("{:.0}", breathing_rate);
    }
}

pub fn heart_text_update(
    mut query: Query<&mut Text, With<HeartText>>,
    heart: Query<&Heart>,
) {

    let heart = heart.single();

    let heart_rate = heart.rate * 60.;

    for mut text in &mut query {
        text.sections[1].value = format!("{:.0}", heart_rate);
    }
}