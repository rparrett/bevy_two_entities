use bevy::{
    ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData},
    prelude::{Entity, Query},
};

/// Extension trait for a read-only `Query`.
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

/// Extension trait for a mutable `Query`.
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
}
