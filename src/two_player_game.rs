use super::players::{
    CharacterlessPlayer,
    BoosterlessPlayer,
    MovelessPlayer,
};
use nzsc_core::{
    characters::Character,
    boosters::Booster,
    moves::Move,
    outcomes,
};
use std::str::FromStr;

pub struct NZSCTwoPlayerGame {
    pub phase: Phase,
}

#[derive(Clone)]
pub enum Phase {
    CharacterChoosing(CharacterlessPlayer, CharacterlessPlayer),
    BoosterChoosing(BoosterlessPlayer, BoosterlessPlayer),
    MoveChoosing(MovelessPlayer, MovelessPlayer),
    GameOver(u8, u8),
}

#[derive(Clone, Copy, PartialEq)]
pub enum WhichPlayer {
    PlayerA,
    PlayerB,
}

impl NZSCTwoPlayerGame {
    pub fn new() -> Self {
        Self {
            phase: Phase::CharacterChoosing(
                CharacterlessPlayer::new(),
                CharacterlessPlayer::new()
            ),
        }
    }

    pub fn process_choice(&mut self, chooser: WhichPlayer, choice: String) -> Result<(), ()> {
        let mut new_phase: Option<Phase> = None;

        let return_val = match &mut self.phase {
            &mut Phase::CharacterChoosing(ref mut a, ref mut b) => {
                let tuple = match chooser {
                    WhichPlayer::PlayerA => {
                        (a, b)
                    },
                    WhichPlayer::PlayerB => {
                        (b, a)
                    },
                };
                let mut a = tuple.0;
                let mut b = tuple.1;

                if let Some(_) = a.selected_character {
                    // Cannot repick.
                    Err(())
                } else {
                    if let Ok(character) = Character::from_str(&choice[..]) {
                        if a.character_streak.times == 3
                            && a.character_streak.repeated_character == Some(character)
                        {
                            b.points += a.penalize(3);

                            if b.points >= 5 {
                                new_phase = Some(Phase::GameOver(a.points, b.points));
                            }

                            Ok(())
                        } else {
                            if let Some(b_character) = b.selected_character {
                                if character == b_character {
                                    a.selected_character = None;
                                    b.selected_character = None;
                                    a.character_streak.add(character);
                                    b.character_streak.add(character);
                                    Ok(())
                                } else {
                                    let headstart = outcomes::get_headstart(character, b_character);

                                    a.points += headstart.0;
                                    b.points += headstart.1;

                                    if a.points >= 5 || b.points >= 5 {
                                        new_phase = Some(Phase::GameOver(a.points, b.points));
                                    } else {
                                        new_phase = Some(Phase::BoosterChoosing(
                                            a.to_boosterless_player(character),
                                            b.to_boosterless_player(b_character),
                                        ));
                                    }

                                    Ok(())
                                }
                            } else {
                                a.selected_character = Some(character);
                                Ok(())
                            }
                        }
                    } else {
                        b.points += a.penalize(4);

                        if b.points >= 5 {
                            new_phase = Some(Phase::GameOver(a.points, b.points));
                        }

                        Ok(())
                    }
                }
            },

            &mut Phase::BoosterChoosing(ref mut a, ref mut b) => {
                let tuple = match chooser {
                    WhichPlayer::PlayerA => {
                        (a, b)
                    },
                    WhichPlayer::PlayerB => {
                        (b, a)
                    },
                };
                let mut a = tuple.0;
                let mut b = tuple.1;

                if let Some(_) = a.selected_booster {
                    // Cannot repick.
                    Err(())
                } else {
                    if let Ok(booster) = Booster::from_str(&choice[..]) {
                        if a.available_boosters().contains(&booster) {
                            if let Some(b_booster) = b.selected_booster {
                                new_phase = Some(Phase::MoveChoosing(
                                    a.to_moveless_player(booster),
                                    b.to_moveless_player(b_booster),
                                ));

                                Ok(())
                            } else {
                                a.selected_booster = Some(booster);
                                Ok(())
                            }
                        } else {
                            // Booster from wrong character.
                            b.points += a.penalize(3);

                            if b.points >= 5 {
                                new_phase = Some(Phase::GameOver(a.points, b.points));
                            }

                            Ok(())
                        }
                    } else {
                        b.points += a.penalize(4);

                        if b.points >= 5 {
                            new_phase = Some(Phase::GameOver(a.points, b.points));
                        }

                        Ok(())
                    }
                }
            },

            &mut Phase::MoveChoosing(ref mut a, ref mut b) => {
                let tuple = match chooser {
                    WhichPlayer::PlayerA => {
                        (a, b)
                    },
                    WhichPlayer::PlayerB => {
                        (b, a)
                    },
                };
                let mut a = tuple.0;
                let mut b = tuple.1;

                if let Some(_) = a.selected_move {
                    // Cannot repick.
                    Err(())
                } else {
                    if let Ok(a_move) = Move::from_str(&choice[..]) {
                        if a.available_moves().contains(&a_move) {
                            if let Some(b_move) = b.selected_move {
                                let points = outcomes::get_points(vec![(a.booster, a_move), (b.booster, b_move)]);
                                a.points += points[0];
                                b.points += points[1];
                                a.move_streak.add(a_move);
                                b.move_streak.add(b_move);
                                a.selected_move = None;
                                b.selected_move = None;

                                if a.points >= 5 || b.points >= 5 {
                                    if a.points == b.points {
                                        a.points = 4;
                                        b.points = 4;
                                    } else {
                                        new_phase = Some(Phase::GameOver(a.points, b.points));
                                    }
                                }

                                Ok(())
                            } else {
                                a.selected_move = Some(a_move);
                                Ok(())
                            }
                        } else {
                            if a.destroyed_moves.contains(&a_move) {
                                b.points += a.penalize(4);

                                if b.points >= 5 {
                                    new_phase = Some(Phase::GameOver(a.points, b.points));
                                }

                                Ok(())
                            } else if a.move_streak.times == 3
                                && a.move_streak.repeated_move == Some(a_move)
                            {
                                b.points += a.penalize(3);

                                if b.points >= 5 {
                                    new_phase = Some(Phase::GameOver(a.points, b.points));
                                }

                                Ok(())
                            } else {
                                let mut booster_moves = vec![];
                                for booster in &a.character.get_boosters() {
                                    booster_moves.extend(booster.get_moves());
                                }

                                if booster_moves.contains(&a_move) {
                                    b.points += a.penalize(2);

                                    if b.points >= 5 {
                                        new_phase = Some(Phase::GameOver(a.points, b.points));
                                    }

                                    Ok(())
                                } else {
                                    b.points += a.penalize(3);

                                    if b.points >= 5 {
                                        new_phase = Some(Phase::GameOver(a.points, b.points));
                                    }

                                    Ok(())
                                }
                            }
                        }
                    } else {
                        b.points += a.penalize(4);

                        if b.points >= 5 {
                            new_phase = Some(Phase::GameOver(a.points, b.points));
                        }

                        Ok(())
                    }
                }
            },

            &mut Phase::GameOver(a_points, b_points) => {
                // You can't make a move after the game is over.
                Err(())
            },
        };

        if let Some(new_phase) = new_phase {
            self.phase = if WhichPlayer::PlayerB == chooser {
                new_phase.flip_a_and_b()
            } else {
                new_phase
            };
        }

        return_val
    }
}

impl Phase {
    pub fn flip_a_and_b(&self) -> Self {
        match self {
            &Phase::CharacterChoosing(ref a, ref b) => {
                Phase::CharacterChoosing(b.clone(), a.clone())
            },
            &Phase::BoosterChoosing(ref a, ref b) => {
                Phase::BoosterChoosing(b.clone(), a.clone())
            },
            &Phase::MoveChoosing(ref a, ref b) => {
                Phase::MoveChoosing(b.clone(), a.clone())
            },
            &Phase::GameOver(ref a, ref b) => {
                Phase::GameOver(b.clone(), a.clone())
            },
        }
    }
}
