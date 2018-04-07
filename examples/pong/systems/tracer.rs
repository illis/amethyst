use Tracer;
use amethyst::core::timing::Time;
use amethyst::ecs::{Fetch, ParJoin, System, WriteStorage, Entities};
use std::time::Duration;
use rayon::iter::ParallelIterator;

pub struct TracerSystem;

impl<'s> System<'s> for TracerSystem {
    type SystemData = (
        WriteStorage<'s, Tracer>,
        Fetch<'s, Time>,
        Entities<'s>,
        );

    fn run(&mut self, (tracers, time, entities): Self::SystemData) {
        // delete tracers after a period of time
        match time.absolute_time().checked_sub(Duration::from_millis(2000)) {
            Some(eval_time) => {
                (&tracers, &*entities).par_join().for_each(|(tracer, ent)| {
                    if tracer.time <= eval_time {
                        entities.delete(ent).unwrap();
                        println!("ent: {:?} tracer: x:{} time:{:?}", ent, tracer.pos.x, tracer.time);
                    }
                });
            }
            None => ()
        };
    }
}

