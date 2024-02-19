# bevy_two_entities

A tiny crate offering a few convenience traits for working with Bevy queries and tuples of Entities.

```rust
fn system(
    collisions: Query<&Collision>,
    mut players: Query<&mut HitPoints, (With<Player>, Without<Enemy>)>,
    mut enemies: Query<&mut HitPoints, (With<Enemy>, Without<Player>)>,
) {
    for collision in &collisions {
        let mut queries = (&mut players, &mut enemies);
        let Some((mut player, mut enemy)) = queries.get_both_mut(collision.0, collision.1)
        else {
            continue;
        };

        player.0 -= 1;
        enemy.0 -= 1;
    }
}
```
