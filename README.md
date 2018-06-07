# nzsc2p

Two-player NZSC.

## Example

Taken from `tests.rs`:

```rust
extern crate nzsc2p;
use nzsc2p::two_player_game::{ NZSCTwoPlayerGame, WhichPlayer, Phase, };

let mut game = NZSCTwoPlayerGame::new();

game.process_choice(WhichPlayer::PlayerA, "Ninja".to_string()).unwrap();
game.process_choice(WhichPlayer::PlayerB, "Clown".to_string()).unwrap();

if let Phase::BoosterChoosing(ref a, ref b) = &g.phase {
    assert_eq!(a.points, 0);
    assert_eq!(b.points, 1);
} else {
    panic!();
}
```
