use bevy::{
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData},
    prelude::{Entity, Query},
};

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
pub trait TupleQueryMutExt<'world, DataA, DataB, FilterA = (), FilterB = ()>
where
    DataA: QueryData,
    DataB: QueryData,
{
    /// Returns Some((a, b)) if both match either `self.0` or `self.1`.
    fn get_both_mut(&mut self, a: Entity, b: Entity) -> Option<(DataA::Item<'_>, DataB::Item<'_>)>;
}

impl<'world, 'state, DataA, DataB, FilterA, FilterB>
    TupleQueryMutExt<'world, DataA, DataB, FilterA, FilterB>
    for (
        &mut Query<'world, 'state, DataA, FilterA>,
        &mut Query<'world, 'state, DataB, FilterB>,
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

pub trait TwoEntitiesQueryExt<'world, Data, Filter = ()>
where
    Data: ReadOnlyQueryData,
{
    /// Returns `true` if either entity matches the Query.
    fn either(&self, a: Entity, b: Entity) -> bool;
    /// Returns `Some(other_entity)` if either entity matches the Query.
    fn either_with_other(&self, a: Entity, b: Entity) -> Option<Entity>;
    /// Returns the first Query item that matches, or `None` if neither matches.
    fn get_either(&self, a: Entity, b: Entity) -> Option<Data::Item<'_>>;
    /// Returns the first Query item that matches and the other entity, or `None` if neither matches.
    fn get_either_with_other(&self, a: Entity, b: Entity) -> Option<(Data::Item<'_>, Entity)>;
}

impl<'world, 'state, Data, Filter> TwoEntitiesQueryExt<'world, Data, Filter>
    for Query<'world, 'state, Data, Filter>
where
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    fn either(&self, a: Entity, b: Entity) -> bool {
        self.get(a).is_ok() || self.get(b).is_ok()
    }

    fn either_with_other(&self, a: Entity, b: Entity) -> Option<Entity> {
        self.get(a)
            .map(|_| b)
            .or_else(|_| self.get(b).map(|_| a))
            .ok()
    }

    fn get_either(&self, a: Entity, b: Entity) -> Option<Data::Item<'_>> {
        self.get(a).or_else(|_| self.get(b)).ok()
    }

    fn get_either_with_other(&self, a: Entity, b: Entity) -> Option<(Data::Item<'_>, Entity)> {
        self.get(a)
            .map(|item| (item, b))
            .or_else(|_| self.get(b).map(|item| (item, a)))
            .ok()
    }
}

pub trait TwoEntitiesMutQueryExt<'world, Data, Filter = ()>
where
    Data: QueryData,
{
    /// Returns the first Query item that matches, or `None` if neither matches.
    fn get_either_mut(&mut self, a: Entity, b: Entity) -> Option<Data::Item<'_>>;
    /// Returns the first Query item that matches and the other entity, or `None` if neither matches.
    fn get_either_mut_with_other(
        &mut self,
        a: Entity,
        b: Entity,
    ) -> Option<(Data::Item<'_>, Entity)>;
}

impl<'world, 'state, Data, Filter> TwoEntitiesMutQueryExt<'world, Data, Filter>
    for Query<'world, 'state, Data, Filter>
where
    Data: QueryData,
    Filter: QueryFilter,
{
    fn get_either_mut(&mut self, a: Entity, b: Entity) -> Option<Data::Item<'_>> {
        let either = if self.get(a).is_ok() {
            a
        } else if self.get(b).is_ok() {
            b
        } else {
            return None;
        };

        self.get_mut(either).ok()
    }

    fn get_either_mut_with_other(
        &mut self,
        a: Entity,
        b: Entity,
    ) -> Option<(Data::Item<'_>, Entity)> {
        let (item_entity, other_entity) = if self.get(a).is_ok() {
            (a, b)
        } else if self.get(b).is_ok() {
            (b, a)
        } else {
            return None;
        };

        self.get_mut(item_entity)
            .ok()
            .map(|item| (item, other_entity))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{ecs::system::SystemState, prelude::*};

    #[test]
    fn get_either() {
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

        let mut system_state: SystemState<Query<Entity, With<A>>> = SystemState::new(&mut world);

        let query = system_state.get_mut(&mut world);

        assert_eq!(query.either(a, b), true);
        assert_eq!(query.either(b, a), true);
        assert_eq!(query.either(b, c), false);

        assert_eq!(query.either_with_other(a, b), Some(b));
        assert_eq!(query.either_with_other(b, a), Some(b));
        assert_eq!(query.either_with_other(b, c), None);

        assert_eq!(query.get_either(a, b), Some(a));
        assert_eq!(query.get_either(b, a), Some(a));
        assert_eq!(query.get_either(b, c), None);

        assert_eq!(query.get_either_with_other(a, b), Some((a, b)));
        assert_eq!(query.get_either_with_other(b, a), Some((a, b)));
        assert_eq!(query.get_either_with_other(b, c), None);
    }

    #[test]
    fn get_either_mut() {
        #[derive(Component, Eq, PartialEq, Debug, Copy, Clone)]
        struct Val(u32);
        #[derive(Component)]
        struct A;
        #[derive(Component)]
        struct B;
        #[derive(Component)]
        struct C;

        let mut world = World::new();
        let a = world.spawn((Val(1), A)).id();
        let b = world.spawn((Val(2), B)).id();
        let c = world.spawn((Val(3), C)).id();

        let mut system_state: SystemState<Query<&mut Val, With<A>>> = SystemState::new(&mut world);

        let mut query = system_state.get_mut(&mut world);

        assert_eq!(query.get_either_mut(a, b).map(|inner| *inner), Some(Val(1)));
        assert_eq!(query.get_either_mut(b, a).map(|inner| *inner), Some(Val(1)));
        assert_eq!(query.get_either_mut(b, c).map(|inner| *inner), None);

        assert_eq!(
            query
                .get_either_mut_with_other(a, b)
                .map(|(inner, other)| (*inner, other)),
            Some((Val(1), b))
        );
        assert_eq!(
            query
                .get_either_mut_with_other(b, a)
                .map(|(inner, other)| (*inner, other)),
            Some((Val(1), b))
        );
        assert_eq!(
            query
                .get_either_mut_with_other(b, c)
                .map(|(inner, other)| (*inner, other)),
            None
        );
    }

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
