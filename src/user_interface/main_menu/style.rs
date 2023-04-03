use::bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const MAIN_MENU_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
    gap: Size::new(Val::Px(8.), Val::Px(8.)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.), Val::Px(80.)),
    ..Style::DEFAULT
};

pub fn get_text_style(assets: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: assets.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.,
        color: Color::WHITE,
    }
}
