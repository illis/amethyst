use super::*;
use amethyst_core::bundle::{ECSBundle, Result};
use amethyst_core::specs::{DispatcherBuilder, World};
use amethyst_renderer::WindowMessages;
use amethyst_renderer::mouse::*;
use std::hash::Hash;
use std::marker::PhantomData;

/// The bundle that creates a flying movement system.
/// Note: Will not actually create a moving entity. It will only register the needed resources and systems.
/// The generic parameters A and B are the ones used in InputHandler<A,B>.
/// You might want to add "fly_movement" and "free_rotation" as dependencies of the TransformSystem.
/// Adding this bundle will grab the mouse, hide it and keep it centered.
pub struct FlyControlBundle<A, B> {
    sensitivity_x: f32,
    sensitivity_y: f32,
    speed: f32,
    right_input_axis: Option<A>,
    up_input_axis: Option<A>,
    forward_input_axis: Option<A>,
    _marker: PhantomData<B>,
}

impl<A, B> FlyControlBundle<A, B> {
    pub fn new(
        right_input_axis: Option<A>,
        up_input_axis: Option<A>,
        forward_input_axis: Option<A>,
    ) -> Self {
        FlyControlBundle {
            sensitivity_x: 1.0,
            sensitivity_y: 1.0,
            speed: 1.0,
            right_input_axis,
            up_input_axis,
            forward_input_axis,
            _marker: PhantomData,
        }
    }
    pub fn with_sensitivity(mut self, x: f32, y: f32) -> Self {
        self.sensitivity_x = x;
        self.sensitivity_y = y;
        self
    }
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
}

impl<'a, 'b, A, B> ECSBundle<'a, 'b> for FlyControlBundle<A, B>
where
    A: Send + Sync + Hash + Eq + Clone + 'static,
    B: Send + Sync + Hash + Eq + Clone + 'static,
{
    fn build(
        self,
        world: &mut World,
        builder: DispatcherBuilder<'a, 'b>,
    ) -> Result<DispatcherBuilder<'a, 'b>> {
        world.register::<FlyControlTag>();

        let mut msg = world.res.entry().or_insert_with(|| WindowMessages::new());

        grab_cursor(&mut msg);
        set_mouse_cursor_none(&mut msg);

        Ok(builder
            .add(
                FlyMovementSystem::<A, B>::new(
                    self.speed,
                    self.right_input_axis,
                    self.up_input_axis,
                    self.forward_input_axis,
                ),
                "fly_movement",
                &[],
            )
            .add(
                FreeRotationSystem::<A, B>::new(self.sensitivity_x, self.sensitivity_y),
                "free_rotation",
                &[],
            )
            .add(MouseCenterLockSystem, "mouse_lock", &["free_rotation"]))
    }
}
