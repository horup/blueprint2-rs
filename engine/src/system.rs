use core::panic;
use std::{any::{Any, TypeId}, marker::PhantomData};

use crate::{Event, Game, log};
use crate::Context;
pub trait System<G:Game> {
    fn on_event(&mut self, event:&Event<G>, context:&mut Context<G>) {
        // nop
    }

    fn on_step(&mut self, time:f32, delta_time:f32, context:&mut Context<G>) {
        // nop
    }

    fn on_initialize(&mut self, context:&mut Context<G>) {
        // nop
    }

    fn on_draw(&mut self, time:f32, delta_time:f32, alpha:f32, context:&mut Context<G>) {
        // nop
    }

    fn on_game_event(&mut self, game_event:G::GameEvent, context:&mut Context<G>) {
        //  nop
    }
}

#[derive(Default)]
pub struct Systems<G:Game> { 
    systems:Vec<Box<dyn Any>>,
    marker:PhantomData<G>
}

impl<G:Game + 'static> Systems<G> {
    pub fn add<T:System<G> + Default + 'static>(&mut self) -> bool {
        let t_id = TypeId::of::<Box<T>>();
        log(&format!("add {:?}", t_id));

        if self.has::<T>() == false {
            self.systems.push(Box::new(T::default()));
            return true;
        } else {
            return false;
        }
    }

    pub fn has<T:System<G> + 'static>(&self) -> bool {
        for system in &self.systems {
            let any = system.as_ref();
            if let Some(as_t) = any.downcast_ref::<T>() {
                return true;
            }
        }

        false
    }

    pub fn get_mut<T:System<G> + 'static>(&mut self) -> &mut T {
        let my_id = TypeId::of::<T>();
        for system in &mut self.systems {
            
        }
        panic!();
    }
}