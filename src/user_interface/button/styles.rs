use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

pub const CLICKED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.), Val::Px(80.)),
    ..Style::DEFAULT
};
