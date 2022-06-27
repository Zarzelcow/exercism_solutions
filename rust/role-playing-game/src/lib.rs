// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

#[derive(Clone, Copy)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        return if self.health > 0 { None } else {
            Some(Self {
                health: 100,
                mana: self.mana.map_or(None, |_| Some(100)),
                level: self.level,
            })
        };
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana_cost > mana {
                return 0;
            }
            self.mana = Some(mana - mana_cost);
            return mana_cost * 2;
        } else {
            if mana_cost > self.health {
                self.health = 0;
            } else {
                self.health -= mana_cost;
            }
            return 0;
        }
    }
}
