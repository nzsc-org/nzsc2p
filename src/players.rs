use nzsc_core::{
    characters::Character,
    boosters::Booster,
    moves::Move,
    streaks::{ CharacterStreak, MoveStreak, },
};

#[derive(Clone)]
pub struct CharacterlessPlayer {
    pub points: u8,
    pub waits: u8,
    pub character_streak: CharacterStreak,
    pub selected_character: Option<Character>
}

#[derive(Clone)]
pub struct BoosterlessPlayer {
    pub points: u8,
    pub waits: u8,
    pub character: Character,
    pub selected_booster: Option<Booster>,
}

#[derive(Clone)]
pub struct MovelessPlayer {
    pub points: u8,
    pub waits: u8,
    pub character: Character,
    pub booster: Booster,
    pub move_streak: MoveStreak,
    pub destroyed_moves: Vec<Move>,
    pub selected_move: Option<Move>,
}

impl CharacterlessPlayer {
    pub fn new() -> Self {
        Self {
            points: 0,
            waits: 4,
            character_streak: CharacterStreak::new(),
            selected_character: None,
        }
    }

    pub fn available_characters(&self) -> Vec<Character> {
        let mut characters = vec![
            Character::Ninja,
            Character::Zombie,
            Character::Samurai,
            Character::Clown,
        ];

        if self.character_streak.times == 3 {
            characters.retain(|&c| Some(c) != self.character_streak.repeated_character);
        }

        characters
    }

    pub fn penalize(&mut self, waits: u8) -> u8 {
        if self.waits < waits {
            self.waits = 0;
            return 1;
        } else {
            self.waits -= waits;
            return 0;
        }
    }

    pub fn to_boosterless_player(&self, character: Character) -> BoosterlessPlayer {
        BoosterlessPlayer {
            points: self.points,
            waits: self.waits,
            character,
            selected_booster: None,
        }
    }
}

impl BoosterlessPlayer {
    pub fn available_boosters(&self) -> Vec<Booster> {
        self.character.get_boosters()
    }

    pub fn penalize(&mut self, waits: u8) -> u8 {
        if self.waits < waits {
            self.waits = 0;
            return 1;
        } else {
            self.waits -= waits;
            return 0;
        }
    }

    pub fn to_moveless_player(&self, booster: Booster) -> MovelessPlayer {
        MovelessPlayer {
            points: self.points,
            waits: self.waits,
            character: self.character,
            booster,
            move_streak: MoveStreak::new(),
            destroyed_moves: vec![],
            selected_move: None,
        }
    }
}

impl MovelessPlayer {
    pub fn available_moves(&self) -> Vec<Move> {
        let character_moves = self.character.get_moves();
        let booster_moves = self.booster.get_moves();

        let mut available_moves = character_moves;
        available_moves.extend(booster_moves);

        let destroyed_moves = &self.destroyed_moves;

        available_moves.retain(|&a| !destroyed_moves.contains(&a));

        if let Some(streak_move) = self.move_streak.repeated_move {
            if self.move_streak.times >= 3 {
                available_moves.retain(|&a| a != streak_move);
            }
        }

        available_moves
    }

    pub fn penalize(&mut self, waits: u8) -> u8 {
        if self.waits < waits {
            self.waits = 0;
            return 1;
        } else {
            self.waits -= waits;
            return 0;
        }
    }
}
