use slog::Logger;
use crate::PlayingCharacter;
use crate::weapon::{PlasmaCannon, RailGun};

pub struct Player {
    name: String,
    logger: Logger,
    weapon: PlasmaCannon,
}

impl Player {
    pub fn new(logger: &Logger, name: &str) -> Self {
        let player_log = logger.new(o!("Player" => format!("{}",name)));
        let weapon_log = player_log.new(o!("PlasmaCannon"=> "M435"));
        Self {
            name: "".to_string(),
            logger: player_log,
            weapon: PlasmaCannon(weapon_log),
        }
    }
}

impl PlayingCharacter for Player {
    fn shoot(&self) {
        info!(self.logger, "{} shooting with {}", self.name, self.weapon);
        self.weapon.fire();
    }
}