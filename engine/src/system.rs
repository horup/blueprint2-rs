use core::panic;
use std::{any::{Any, TypeId}, marker::PhantomData};

use crate::{Event, Game, log};
use crate::Context;
pub trait System<G:Game> {
    /*fn on_event(&mut self, event:&Event<G>, context:&mut Context<G>) {
        // nop
    }*/

    fn on_step(&mut self, time:&f32, delta_time:&f32, context:&mut Context<G>) {
        // nop
    }

    fn on_initialize(&mut self, context:&mut Context<G>) {
        // nop
    }

    fn on_draw(&mut self, time:&f32, delta_time:&f32, alpha:&f32, context:&mut Context<G>) {
        // nop
    }

    fn on_game_event(&mut self, game_event:&G::GameEvent, context:&mut Context<G>) {
        //  nop
    }

    fn as_any(&self) -> &dyn Any;
}

#[derive(Default)]
pub struct Systems<G:Game> { 
    systems:Vec<Box<dyn System<G>>>,
    marker:PhantomData<G>
}

impl<G:Game + 'static> Systems<G> {
    pub fn add<T:System<G> + Default + 'static>(&mut self) -> bool {
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
            if let Some(as_t) = any.as_any().downcast_ref::<T>() {
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

    pub fn on_event(&mut self, e:&Event<G>, context:&mut Context<G>) {
        for system in &mut self.systems { 
            match e {
                Event::Initialize => {system.on_initialize(context)}
                Event::Step(time, delta_time) => {system.on_step(time, delta_time, context)}
                Event::Draw(time, delta_time, alpha) => {system.on_draw(time, delta_time, alpha, context)}
                Event::GameEvent(game_event) => {system.on_game_event(game_event, context)}
            }
        }
    }
}