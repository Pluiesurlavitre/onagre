use crate::style::color::OnagreColor;
use iced_style::{text_input, container, Background, Color};
use crate::style::layout::Length;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(default)]
pub struct SearchContainerStyles {
    pub background: OnagreColor,
    pub border_radius: u16,
    pub border_width: u16,
    pub text_color: OnagreColor,
    pub border_color: OnagreColor,
    pub width: Length,
    pub height: Length,
    pub bar: SearchBarStyles,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SearchBarStyles {
    pub border_radius: u16,
    pub border_width: u16,
    pub border_color: OnagreColor,
    pub background: OnagreColor,
    pub placeholder_color: OnagreColor,
    pub value_color: OnagreColor,
    pub selection_color: OnagreColor,
    pub text_width: Length,
}

impl Default for SearchContainerStyles {
    fn default() -> Self {
        Self {
            border_radius: 0,
            border_width: 0,
            text_color: OnagreColor::WHITE,
            border_color: OnagreColor::WHITE,
            background: OnagreColor::RED,
            height: Length::fill(),
            bar: SearchBarStyles {
                border_radius: 0,
                border_width: 0,
                border_color: OnagreColor::GREEN,
                background: OnagreColor::WHITE,
                placeholder_color: OnagreColor::BLUE,
                value_color: OnagreColor::BLACK,
                selection_color: OnagreColor::GREEN,
                text_width: Length::fill(),
            },
            width: Length::fill(),
        }
    }
}

impl container::StyleSheet for &SearchContainerStyles {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.background.into())),
            border_radius: self.border_radius,
            border_width: self.border_radius,
            text_color: Some(self.text_color.into()),
            border_color: self.border_color.into(),
        }
    }
}

impl text_input::StyleSheet for &SearchBarStyles {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }

    fn placeholder_color(&self) -> Color {
        self.placeholder_color.into()
    }

    fn value_color(&self) -> Color {
        self.value_color.into()
    }

    fn selection_color(&self) -> Color {
        self.selection_color.into()
    }

    fn hovered(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(self.background.into()),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color.into(),
        }
    }
}