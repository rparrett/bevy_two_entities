# bevy_two_entities

A tiny crate offering a few convenience traits for working with Bevy queries and tuples of Entities.

```rust
fn system(
    collisions: Query<&Collision>,
    mut players: Query<&mut HitPoints, With<Player>>,
    mut enemies: Query<&mut HitPoints, With<Enemy>>,
) {
    for collision in &collisions {
        if let Some((mut player, other)) =
            players.get_either_mut_with_other(collision.0, collision.1)
        {
            if let Ok(mut enemy) = enemies.get_mut(other) {
                player.0 -= 1;
                enemy.0 -= 1;
            }
        }
    }
}
```
