#![cfg(test)]

use super::two_player_game::{
    NZSCTwoPlayerGame,
    Phase,
    WhichPlayer,
};

#[test]
fn it_works() {
    let mut g = NZSCTwoPlayerGame::new();

    g.process_choice(WhichPlayer::PlayerA, "Ninja".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Clown".to_string()).unwrap();

    if let Phase::BoosterChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 0);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerA, "Shadow".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Moustachio".to_string()).unwrap();

    g.process_choice(WhichPlayer::PlayerA, "Shadow Fireball".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Backwards Moustachio".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 0);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerB, "Backwards Moustachio".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 0);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerB, "Juggling Knives".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 1);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerB, "Juggling Fives".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 2);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerA, "Shadow Slip".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Big Hairy Deal".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerA, "Shadow Slip".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Big Hairy Deal".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerA, "Shadow Slip".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Big Hairy Deal".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 2);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerA, "Shadow Slip".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Big Hairy Deal".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 3);
        assert_eq!(b.points, 1);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerA, "Shadow Slip".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Big Hairy Deal".to_string()).unwrap();

    if let Phase::MoveChoosing(ref a, ref b) = &g.phase {
        assert_eq!(a.points, 4);
        assert_eq!(b.points, 2);
    } else {
        panic!();
    }

    g.process_choice(WhichPlayer::PlayerA, "Shadow Fireball".to_string()).unwrap();
    g.process_choice(WhichPlayer::PlayerB, "Nose".to_string()).unwrap();

    if let Phase::GameOver(a_points, b_points) = &g.phase {
        assert_eq!(*a_points, 5);
        assert_eq!(*b_points, 2);
    } else {
        panic!("Game not over!");
    }
}

#[test]
fn cant_pick_character_twice() {
    let mut g = NZSCTwoPlayerGame::new();

    g.process_choice(WhichPlayer::PlayerA, "Ninja".to_string()).unwrap();
    let e = g.process_choice(WhichPlayer::PlayerA, "Ninja".to_string());

    assert_eq!(e, Err(()));
}
