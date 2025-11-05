use iced::{Background, Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppColorMain {
    Primary,
    Secondary,
    Tertiary,
    AccentPrimary,
    AccentSecondary,
    AccentTertiary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppColorStatus {
    Success,
    Warning,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppColorBackground {
    Primary,
    Secondary,
    Tertiary,
    SurfacePrimary,
    SurfaceSecondary,
    SurfaceTertiary,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppColorForeground {
    Primary,
    Secondary,
    Tertiary,
    SubtextPrimary,
    SubtextSecondary,
    SubtextTertiary,
}

impl From<AppColorMain> for Color {
    fn from(value: AppColorMain) -> Self {
        match value {
            AppColorMain::Primary => Color::from_rgb8(180, 190, 254),
            AppColorMain::Secondary => Color::from_rgb8(137, 180, 250),
            AppColorMain::Tertiary => Color::from_rgb8(116, 199, 236),
            AppColorMain::AccentPrimary => Color::from_rgb8(203, 166, 247),
            AppColorMain::AccentSecondary => Color::from_rgb8(245, 194, 231),
            AppColorMain::AccentTertiary => Color::from_rgb8(242, 205, 205),
        }
    }
}

impl AppColorMain {
    pub fn to_bg(self) -> Background {
        Background::Color(Color::from(self))
    }
}

impl From<AppColorStatus> for Color {
    fn from(value: AppColorStatus) -> Self {
        match value {
            AppColorStatus::Success => Color::from_rgb8(166, 227, 161),
            AppColorStatus::Warning => Color::from_rgb8(249, 226, 175),
            AppColorStatus::Failure => Color::from_rgb8(243, 139, 168),
        }
    }
}

impl AppColorStatus {
    pub fn to_bg(self) -> Background {
        Background::Color(Color::from(self))
    }
}

impl From<AppColorBackground> for Color {
    fn from(value: AppColorBackground) -> Self {
        match value {
            AppColorBackground::Primary => Color::from_rgb8(17, 17, 27),
            AppColorBackground::Secondary => Color::from_rgb8(24, 24, 37),
            AppColorBackground::Tertiary => Color::from_rgb8(30, 30, 46),
            AppColorBackground::SurfacePrimary => Color::from_rgb8(49, 50, 68),
            AppColorBackground::SurfaceSecondary => Color::from_rgb8(69, 71, 90),
            AppColorBackground::SurfaceTertiary => Color::from_rgb8(88, 91, 112),
        }
    }
}

impl AppColorBackground {
    pub fn to_bg(self) -> Background {
        Background::Color(Color::from(self))
    }
}

impl From<AppColorForeground> for Color {
    fn from(value: AppColorForeground) -> Self {
        match value {
            AppColorForeground::Primary => Color::from_rgb8(205, 214, 244),
            AppColorForeground::Secondary => Color::from_rgb8(186, 194, 222),
            AppColorForeground::Tertiary => Color::from_rgb8(166, 173, 200),
            AppColorForeground::SubtextPrimary => Color::from_rgb8(147, 153, 178),
            AppColorForeground::SubtextSecondary => Color::from_rgb8(127, 132, 156),
            AppColorForeground::SubtextTertiary => Color::from_rgb8(108, 112, 134),
        }
    }
}

impl AppColorForeground {
    pub fn to_bg(self) -> Background {
        Background::Color(Color::from(self))
    }
}
