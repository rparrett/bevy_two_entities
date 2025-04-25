# bevy_two_entities

[![crates.io](https://img.shields.io/crates/v/bevy_two_entities.svg)](https://crates.io/crates/bevy_two_entities)
[![docs](https://docs.rs/bevy_two_entities/badge.svg)](https://docs.rs/bevy_two_entities)
[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/book/plugin-development/#main-branch-tracking)

A tiny crate offering a few convenience traits on Bevy's `Query` and `(&Query, &Query)` for scenarios involving exactly two entities.

## Examples

```rust
fn game_over(
    collisions: Query<&Collision>,
    players: Query<(), With<Player>>,
    enemies: Query<(), With<Enemy>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for collision in &collisions {
        if (&players, &enemies).both(collision.0, collision.1) {
            next_state.set(GameState::GameOver);
        }
    }
}
```

```rust
fn damage_enemy(
    collisions: Query<&Collision>,
    players: Query<(), With<Player>>,
    mut enemies: Query<&mut HitPoints, With<Enemy>>,
) {
    for collision in &collisions {
        let mut queries = (&players, &mut enemies);
        let Some((_, mut enemy_hp)) = queries.get_both_mut(collision.0, collision.1) else {
            continue;
        };

        enemy_hp.0 -= 1;
    }
}
```

## Compatibility

| `bevy_two_entities` | `bevy` |
| :--                 | :--    |
| `0.4`               | `0.16` |
| `0.3`               | `0.15` |
| `0.2`               | `0.14` |
| `0.1`               | `0.13` |
