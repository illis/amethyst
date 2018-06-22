use amethyst_core::specs::prelude::{Component, DenseVecStorage, Entities, Entity, Join, Read,
                                    ReadExpect, ReadStorage, Resources, System, Write,
                                    WriteStorage};
use amethyst_renderer::ScreenDimensions;
use super::*;

pub struct UiTextScaler {
    pub base_size: f32,
    pub base_width: f32,
    pub base_height: f32,
}

impl Component for UiTextScaler {
    type Storage = DenseVecStorage<Self>;
}

#[derive(SystemData)]
pub struct UiTextScalerResources<'s> {
    uitexts: WriteStorage<'s, UiText>,
    scalers: WriteStorage<'s, UiTextScaler>,
    screendimensions: ReadExpect<'s, ScreenDimensions>,
}

pub struct UiTextScalerSystem;
impl<'s> System<'s> for UiTextScalerSystem {
    type SystemData = UiTextScalerResources<'s>;

    fn run(&mut self, mut res: Self::SystemData) {
        for (text, scaler) in (&mut res.uitexts, &res.scalers).join() {
            text.font_size = scaler.base_size * 
                ((res.screendimensions.width() / scaler.base_width) + 
                (res.screendimensions.height() / scaler.base_height) /2.);
        }
    }
}
