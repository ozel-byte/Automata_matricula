    mod style {
        use iced::{
            button, container, radio, text_input,
        };

        pub enum Theme{
            Light,
            Dark,
        }

        impl Theme {
            pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
        }

        impl Default for Theme {
            fn default() -> Theme {
                Theme::Light
            }
        }

        impl<'a> From<Theme> for Box<dyn container::StyleSheet + 'a> {
            fn from(theme: Theme) -> Self {
                match theme {
                    Theme::Light => Default::default(),
                    Theme::Dark => dark::Container.into(),
                }
            }
        }

        impl<'a> From<Theme> for Box<dyn radio::StyleSheet + 'a> {
            fn from(theme: Theme) -> Self {
                match theme {
                    Theme::Light => Default::default(),
                    Theme::Dark => dark::Container.into(),
                }
            }
        }

        impl<'a> From<Theme> for Box<dyn text_input::StyleSheet + 'a> {
            fn from(theme: Theme) -> Self {
                match theme {
                    Theme::Light => Default::default(),
                    Theme::Dark => dark::TextInput.into(),
                }
            }
        }

        impl<'a> From<Theme> for Box<dyn button::StyleSheet + 'a> {
            fn from(theme: Theme) -> Self {
                match theme {
                    Theme::Light => light::Button.into(),
                    Theme::Dark => dark::Button.into(),
                }
            }
        }

    }