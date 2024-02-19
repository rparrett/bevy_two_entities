use bevy::{
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData},
    prelude::{Entity, Query},
};

use crate::query::{TwoEntitiesMutQueryExt, TwoEntitiesQueryExt};

// Extension trait for working with tuples of queries
pub trait TupleQueryExt<'world, DataA, DataB, FilterA = (), FilterB = ()>
where
    DataA: QueryData,
    DataB: QueryData,
{
    /// Returns true if `a` and `b` both match either `self.0` or `self.1`.
    fn both(&self, a: Entity, b: Entity) -> bool;

    /// Returns Some((a, b)) if both match either `self.0` or `self.1`.
    fn get_both(&self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)>;
}

impl<'world, 'state, DataA, DataB, FilterA, FilterB>
    TupleQueryExt<'world, DataA, DataB, FilterA, FilterB>
    for (
        &Query<'world, 'state, DataA, FilterA>,
        &Query<'world, 'state, DataB, FilterB>,
    )
where
    DataA: ReadOnlyQueryData,
    DataB: ReadOnlyQueryData,
    FilterA: QueryFilter,
    FilterB: QueryFilter,
{
    fn both(&self, a: Entity, b: Entity) -> bool {
        self.0.either(a, b) && self.1.either(a, b)
    }

    fn get_both(&self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)> {
        self.0
            .get_either_with_other(a, b)
            .and_then(|(item_a, other)| match self.1.get(other) {
                Ok(item_b) => Some((item_a, item_b)),
                Err(_) => None,
            })
    }
}

// Extension trait for working with tuples of queries
pub trait TupleQueryMutExt<'world_a, 'world_b, DataA, DataB, FilterA = (), FilterB = ()>
where
    DataA: QueryData,
    DataB: QueryData,
{
    /// Returns Some((a, b)) if both match either `self.0` or `self.1`.
    fn get_both_mut(&mut self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)>;
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'world_a, 'world_b, DataA, DataB, FilterA, FilterB>
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
    fn get_both_mut(&mut self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)> {
        let (item_a, other) = self.0.get_either_mut_with_other(a, b)?;

        let item_b = self.1.get_mut(other).ok()?;

        Some((item_a, item_b))
    }
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'world_a, 'world_b, DataA, DataB, FilterA, FilterB>
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
    fn get_both_mut(&mut self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)> {
        let (item_a, other) = self.0.get_either_mut_with_other(a, b)?;

        let item_b = self.1.get(other).ok()?;

        Some((item_a, item_b))
    }
}

impl<'world_a, 'world_b, 'state_a, 'state_b, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'world_a, 'world_b, DataA, DataB, FilterA, FilterB>
    for (
        &Query<'world_a, 'state_a, DataA, FilterA>,
        &mut Query<'world_b, 'state_b, DataB, FilterB>,
    )
where
    DataA: ReadOnlyQueryData,
    DataB: QueryData,
    FilterA: QueryFilter,
    FilterB: QueryFilter,
{
    fn get_both_mut(&mut self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)> {
        let (item_a, other) = self.0.get_either_with_other(a, b)?;

        let item_b = self.1.get_mut(other).ok()?;

        Some((item_a, item_b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{ecs::system::SystemState, prelude::*};

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

        let (query_a, query_b) = system_state.get_mut(&mut world);

        assert!((&query_a, &query_b).both(a, b));
        assert!((&query_a, &query_b).both(b, a));
        assert!(!(&query_a, &query_b).both(a, c));
        assert!(!(&query_a, &query_b).both(c, b));
        assert!(!(&query_a, &query_b).both(a, a));
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

            let (mut query_a, mut query_b, mut query_c) = system_state.get_mut(&mut world);

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

            let (mut query_a, mut query_b, mut _query_c) = system_state.get_mut(&mut world);

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

        fn _system(
            collisions: Query<&Collision>,
            mut players: Query<&mut HitPoints, (With<Player>, Without<Enemy>)>,
            mut enemies: Query<&mut HitPoints, (With<Enemy>, Without<Player>)>,
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

        fn _system_two(
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
    }
}
