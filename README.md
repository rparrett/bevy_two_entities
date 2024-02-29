# bevy_two_entities

A tiny crate offering a few convenience traits on Bevy's `Query` and `(&Query, &Query)` for scenarios involving exactly two entities.

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
