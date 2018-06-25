use amethyst_core::specs::prelude::{Component, DenseVecStorage, Join, ReadExpect, System, WriteStorage};
use amethyst_renderer::ScreenDimensions;
use super::*;

pub struct UiTextScaler {
    pub base_size: f32,
    pub scale_mode: ScaleMode,
}

impl Component for UiTextScaler {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemData)]
pub struct UiTextScalerResources<'s> {
    uitexts: WriteStorage<'s, UiText>,
    scalers: WriteStorage<'s, UiTextScaler>,
    screen_dimensions: ReadExpect<'s, ScreenDimensions>,
}

#[derive(Default)]
pub struct UiTextScalerSystem {
    screen_size: (f32, f32),
}

impl<'s> System<'s> for UiTextScalerSystem {
    type SystemData = UiTextScalerResources<'s>;

    fn run(&mut self, mut res: Self::SystemData) {
        let current_screen_size = (res.screen_dimensions.width(), res.screen_dimensions.height());
        let screen_resized = current_screen_size != self.screen_size;
        self.screen_size = current_screen_size;

        if screen_resized {
            for (text, scaler) in (&mut res.uitexts, &res.scalers).join() {
                text.font_size = match scaler.scale_mode {
                    ScaleMode::Pixel => {
                        scaler.base_size
                    },
                    ScaleMode::Percent => {
                        scaler.base_size * res.screen_dimensions.height()
                    },
                }
            }
        }
    }
}
