use bevy::{
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData},
    prelude::{Entity, Query},
};

/// Extension trait for working with tuples of queries.
pub trait TupleQueryExt<'state_a, 'state_b, DataA, DataB, FilterA = (), FilterB = ()>
where
    DataA: QueryData,
    DataB: QueryData,
{
    /// Returns `true` if one input entity matches `self.0` and the other matches
    /// `self.1`, in either order.
    fn both(&self, a: Entity, b: Entity) -> bool;

    /// Returns query items in `(self.0, self.1)` order if one input entity matches
    /// `self.0` and the other matches `self.1`, in either order. Otherwise returns `None`.
    fn get_both<'world>(
        &'world self,
        a: Entity,
        b: Entity,
    ) -> Option<(DataA::Item<'world, 'state_a>, DataB::Item<'world, 'state_b>)>;
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryExt<'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    for (
        &Query<'world_a, 'state_a, DataA, FilterA>,
        &Query<'world_b, 'state_b, DataB, FilterB>,
    )
where
    DataA: ReadOnlyQueryData,
    DataB: ReadOnlyQueryData,
    FilterA: QueryFilter,
    FilterB: QueryFilter,
{
    fn both(&self, a: Entity, b: Entity) -> bool {
        (self.0.contains(a) && self.1.contains(b)) || (self.0.contains(b) && self.1.contains(a))
    }

    fn get_both<'query>(
        &'query self,
        a: Entity,
        b: Entity,
    ) -> Option<(DataA::Item<'query, 'state_a>, DataB::Item<'query, 'state_b>)> {
        self.0
            .get(a)
            .ok()
            .zip(self.1.get(b).ok())
            .or_else(|| self.0.get(b).ok().zip(self.1.get(a).ok()))
    }
}

/// Extension trait for working with tuples of mutable queries.
pub trait TupleQueryMutExt<'state_a, 'state_b, DataA, DataB, FilterA = (), FilterB = ()>
where
    DataA: QueryData,
    DataB: QueryData,
{
    /// Returns query items with mutable access in `(self.0, self.1)` order if one
    /// input entity matches `self.0` and the other matches `self.1`, in either order.
    /// Otherwise returns `None`.
    fn get_both_mut<'world>(
        &'world mut self,
        a: Entity,
        b: Entity,
    ) -> Option<(DataA::Item<'world, 'state_a>, DataB::Item<'world, 'state_b>)>;
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    for (
        &mut Query<'world_a, 'state_a, DataA, FilterA>,
        &mut Query<'world_b, 'state_b, DataB, FilterB>,
    )
where
    DataA: QueryData,
    DataB: QueryData,
    FilterA: QueryFilter,
    FilterB: QueryFilter,
{
    fn get_both_mut<'query>(
        &'query mut self,
        a: Entity,
        b: Entity,
    ) -> Option<(DataA::Item<'query, 'state_a>, DataB::Item<'query, 'state_b>)> {
        let (first, second) = if self.0.contains(a) && self.1.contains(b) {
            (a, b)
        } else {
            (b, a)
        };

        let item_a = self.0.get_mut(first).ok()?;
        let item_b = self.1.get_mut(second).ok()?;

        Some((item_a, item_b))
    }
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    for (
        &mut Query<'world_a, 'state_a, DataA, FilterA>,
        &Query<'world_b, 'state_b, DataB, FilterB>,
    )
where
    DataA: QueryData,
    DataB: ReadOnlyQueryData,
    FilterA: QueryFilter,
    FilterB: QueryFilter,
{
    fn get_both_mut<'query>(
        &'query mut self,
        a: Entity,
        b: Entity,
    ) -> Option<(DataA::Item<'query, 'state_a>, DataB::Item<'query, 'state_b>)> {
        let (first, second) = if self.0.contains(a) && self.1.contains(b) {
            (a, b)
        } else {
            (b, a)
        };

        let item_a = self.0.get_mut(first).ok()?;
        let item_b = self.1.get(second).ok()?;

        Some((item_a, item_b))
    }
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    for (
        &Query<'world_a, 'state_a, DataA, FilterB>,
        &mut Query<'world_b, 'state_b, DataB, FilterA>,
    )
where
    DataA: ReadOnlyQueryData,
    DataB: QueryData,
    FilterA: QueryFilter,
    FilterB: QueryFilter,
{
    fn get_both_mut<'query>(
        &'query mut self,
        a: Entity,
        b: Entity,
    ) -> Option<(DataA::Item<'query, 'state_a>, DataB::Item<'query, 'state_b>)> {
        let (first, second) = if self.0.contains(a) && self.1.contains(b) {
            (a, b)
        } else {
            (b, a)
        };

        let item_a = self.0.get(first).ok()?;
        let item_b = self.1.get_mut(second).ok()?;

        Some((item_a, item_b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::TwoEntitiesMutQueryExt;
    use bevy::{ecs::system::SystemState, prelude::*, state::app::StatesPlugin};

    #[test]
    fn both() {
        #[derive(Component)]
        struct A;
        #[derive(Component)]
        struct B;
        #[derive(Component)]
        struct C;

        let mut world = World::new();
        let a = world.spawn(A).id();
        let b = world.spawn(B).id();
        let c = world.spawn(C).id();

        let mut system_state: SystemState<(Query<Entity, With<A>>, Query<Entity, With<B>>)> =
            SystemState::new(&mut world);

        let (query_a, query_b) = system_state.get_mut(&mut world).unwrap();

        assert!((&query_a, &query_b).both(a, b));
        assert!((&query_a, &query_b).both(b, a));
        assert!(!(&query_a, &query_b).both(a, c));
        assert!(!(&query_a, &query_b).both(c, b));
        assert!(!(&query_a, &query_b).both(a, a));
    }

    #[test]
    fn both_with_overlapping_queries() {
        #[derive(Component)]
        struct A;
        #[derive(Component)]
        struct B;

        let mut world = World::new();
        let a = world.spawn(A).id();
        let b = world.spawn(B).id();
        let ab = world.spawn((A, B)).id();
        let neither = world.spawn_empty().id();

        let mut system_state: SystemState<(Query<(), With<A>>, Query<(), With<B>>)> =
            SystemState::new(&mut world);
        let (query_a, query_b) = system_state.get_mut(&mut world).unwrap();
        let queries = (&query_a, &query_b);

        assert!(!queries.both(ab, neither));
        assert!(!queries.both(neither, ab));
        assert!(queries.both(ab, a));
        assert!(queries.both(a, ab));
        assert!(queries.both(ab, b));
        assert!(queries.both(b, ab));
    }

    #[test]
    fn get_both() {
        #[derive(Component, Debug, PartialEq, Eq)]
        struct A(u32);
        #[derive(Component, Debug, PartialEq, Eq)]
        struct B(u32);

        let mut world = World::new();
        let a = world.spawn(A(1)).id();
        let b = world.spawn(B(2)).id();
        let c = world.spawn_empty().id();

        let mut system_state: SystemState<(Query<&A>, Query<&B>)> = SystemState::new(&mut world);
        let (query_a, query_b) = system_state.get_mut(&mut world).unwrap();
        let queries = (&query_a, &query_b);

        assert_eq!(queries.get_both(a, b), Some((&A(1), &B(2))));
        assert_eq!(queries.get_both(b, a), Some((&A(1), &B(2))));
        assert_eq!(queries.get_both(a, c), None);
        assert_eq!(queries.get_both(c, b), None);
    }

    #[test]
    fn getters_with_overlapping_queries() {
        #[derive(Component)]
        struct A(u32);
        #[derive(Component)]
        struct B(u32);

        let mut world = World::new();
        let a = world.spawn(A(0)).id();
        let b = world.spawn(B(0)).id();
        let ab = world.spawn((A(0), B(0))).id();
        let ab2 = world.spawn((A(0), B(0))).id();
        let neither = world.spawn_empty().id();
        let cases = [
            (ab, a, Some((a, ab))),
            (a, ab, Some((a, ab))),
            (ab, b, Some((ab, b))),
            (b, ab, Some((ab, b))),
            (ab, neither, None),
            (neither, ab, None),
            (ab, ab2, Some((ab, ab2))),
            (ab2, ab, Some((ab2, ab))),
        ];

        let mut system_state: SystemState<(Query<(Entity, &mut A)>, Query<(Entity, &mut B)>)> =
            SystemState::new(&mut world);
        {
            let (mut query_a, mut query_b) = system_state.get_mut(&mut world).unwrap();
            for (first, second, expected) in cases {
                {
                    let read_a = query_a.as_readonly();
                    let read_b = query_b.as_readonly();
                    let queries = (&read_a, &read_b);
                    let result = queries
                        .get_both(first, second)
                        .map(|((a, _), (b, _))| (a, b));
                    assert_eq!(result, expected);
                    assert_eq!(queries.both(first, second), result.is_some());
                }
                {
                    let mut queries = (&mut query_a, &mut query_b);
                    let result = queries.get_both_mut(first, second).map(
                        |((a, mut a_value), (b, mut b_value))| {
                            a_value.0 += 1;
                            b_value.0 += 1;
                            (a, b)
                        },
                    );
                    assert_eq!(result, expected);
                }
                {
                    let read_b = query_b.as_readonly();
                    let mut queries = (&mut query_a, &read_b);
                    let result =
                        queries
                            .get_both_mut(first, second)
                            .map(|((a, mut a_value), (b, _))| {
                                a_value.0 += 1;
                                (a, b)
                            });
                    assert_eq!(result, expected);
                }
                {
                    let read_a = query_a.as_readonly();
                    let mut queries = (&read_a, &mut query_b);
                    let result =
                        queries
                            .get_both_mut(first, second)
                            .map(|((a, _), (b, mut b_value))| {
                                b_value.0 += 1;
                                (a, b)
                            });
                    assert_eq!(result, expected);
                }
            }
        }
        // Each successful pairing mutates each selected component twice.
        assert_eq!(world.get::<A>(a).unwrap().0, 4);
        assert_eq!(world.get::<A>(ab).unwrap().0, 6);
        assert_eq!(world.get::<A>(ab2).unwrap().0, 2);
        assert_eq!(world.get::<B>(b).unwrap().0, 4);
        assert_eq!(world.get::<B>(ab).unwrap().0, 6);
        assert_eq!(world.get::<B>(ab2).unwrap().0, 2);
    }

    #[test]
    fn get_both_mut() {
        #[derive(Component, Debug, PartialEq, Eq)]
        struct A(u32);
        #[derive(Component, Debug, PartialEq, Eq)]
        struct B(u32);
        #[derive(Component, Debug, PartialEq, Eq)]
        struct C(u32);

        let mut world = World::new();
        let a = world.spawn(A(1)).id();
        let b = world.spawn(B(2)).id();
        let c = world.spawn(C(3)).id();

        {
            let mut system_state: SystemState<(Query<&mut A>, Query<&mut B>, Query<&mut C>)> =
                SystemState::new(&mut world);

            let (mut query_a, mut query_b, mut query_c) = system_state.get_mut(&mut world).unwrap();

            {
                let mut queries = (&mut query_a, &mut query_c);
                assert!(queries.get_both_mut(a, b).is_none());
                assert!(queries.get_both_mut(b, a).is_none());
            }

            {
                let mut queries = (&mut query_a, &mut query_b);
                assert!(queries.get_both_mut(a, c).is_none());
                assert!(queries.get_both_mut(c, b).is_none());

                let (mut a_item, mut b_item) = queries.get_both_mut(a, b).unwrap();

                assert_eq!(*a_item, A(1));
                assert_eq!(*b_item, B(2));

                a_item.0 = 10;
                b_item.0 = 20;
            }
        }

        {
            let mut system_state: SystemState<(Query<&mut A>, Query<&mut B>, Query<&mut C>)> =
                SystemState::new(&mut world);

            let (mut query_a, mut query_b, mut _query_c) =
                system_state.get_mut(&mut world).unwrap();

            let mut queries = (&mut query_a, &mut query_b);
            let (a_item, b_item) = queries.get_both_mut(b, a).unwrap();

            assert_eq!(*a_item, A(10));
            assert_eq!(*b_item, B(20));
        }
    }

    #[test]
    fn example() {
        #[derive(Component)]
        struct HitPoints(u32);
        #[derive(Component)]
        struct Collision(Entity, Entity);
        #[derive(Component)]
        struct Player;
        #[derive(Component)]
        struct Enemy;
        #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
        enum GameState {
            #[default]
            GameOver,
        }

        fn system_one(
            collisions: Query<&Collision>,
            mut players: Query<&mut HitPoints, (With<Player>, Without<Enemy>)>,
            mut enemies: Query<&mut HitPoints, (With<Enemy>, Without<Player>)>,
        ) {
            for collision in &collisions {
                if let Some((mut player, other)) =
                    players.get_either_mut_with_other(collision.0, collision.1)
                    && let Ok(mut enemy) = enemies.get_mut(other)
                {
                    player.0 -= 1;
                    enemy.0 -= 1;
                }
            }
        }

        fn system_two(
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

        fn system_three(
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

        fn system_four(
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

        App::new()
            .add_plugins(StatesPlugin)
            .init_state::<GameState>()
            .add_systems(Update, (system_one, system_two, system_three, system_four))
            .run();
    }
}
