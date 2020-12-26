use std::hash::Hash;

use super::Sprite;
use super::SpriteID;

pub trait GameWorld : Default + Clone {
    type Sprite : Default + Copy + Clone;
    type Event;
    type Art : Copy + Clone + Eq + PartialEq + Hash;
    type Texture : Copy + Clone + Eq + PartialEq + Hash;
}


#[derive(Clone)]
pub struct World<W:GameWorld>
{
    pub player_id:u128,
    pub timestamp:f32,
    pub ext:W,
    sprites:Vec<Sprite<W>>
}

impl<W:GameWorld> Default for World<W> {
    fn default() -> Self {
        Self {
            player_id:0,
            timestamp:0.0,
            sprites:Vec::new(),
            ext:W::default()
        }
    }
}

impl<W:GameWorld> World<W> {
    pub fn new_sprite(&mut self, art:W::Art) -> &mut Sprite<W> {

        let mut free:Option<SpriteID> = None;
        for sprite in &self.sprites {
            if !sprite.in_use() {
                let mut id = *sprite.id();
                id.generation += 1;
                free = Some(id);
            }
        }

        if let None = free {
            free = Some(SpriteID {
                generation:0,
                index:self.sprites.len() as u16
            });

            self.sprites.push(Sprite::new(free.unwrap(), art));
            
        }

        let id = free.unwrap();
        let sprite = self.sprites.get_mut(id.index as usize).unwrap();
        *sprite = Sprite::new(id, art);
        sprite
    }

    pub fn delete_sprite(&mut self, id: &SpriteID) {
        if let Some(sprite) = self.sprites.get_mut(id.index as usize) {
            sprite.delete();
        }
    }

    pub fn get_sprite(&self, id:&SpriteID) -> Option<&Sprite<W>> {
        if let Some(sprite) = self.sprites.get(id.index as usize) {
            if sprite.in_use() && sprite.id() == id {
                return Some(sprite);
            }
        }

        None
    }

    pub fn get_sprite_mut(&mut self, id:&SpriteID) -> Option<&mut Sprite<W>> {
        if let Some(sprite) = self.sprites.get_mut(id.index as usize) {
            if sprite.in_use() && sprite.id() == id {
                return Some(sprite);
            }
        }

        None
    }

    pub fn find_sprite_mut(&mut self, predicate:impl FnMut(&&mut Sprite<W>)->bool) -> Option<&mut Sprite<W>> {
        self.sprites.iter_mut().find(predicate)
    }

    pub fn sprites_iter(&self) -> impl Iterator<Item = &Sprite<W>> {
        self.sprites.iter().filter(|x| x.in_use())
    }

    pub fn sprites_iter_mut(&mut self) -> impl Iterator<Item = &mut Sprite<W>> {
        self.sprites.iter_mut().filter(|x| x.in_use())
    }
}