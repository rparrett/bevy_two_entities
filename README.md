# bevy_two_entities

A tiny crate offering a few convenience traits for bevy Queries.

```rust
fn system_three(
    collisions: Query<&Collision>,
    players: Query<Entity, (With<Player>, Without<Enemy>)>,
    enemies: Query<(), (With<Enemy>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for collision in &collisions {
        if (&players, &enemies).both(collision.0, collision.1) {
            next_state.set(GameState::GameOver);
        }
    }
}
```
