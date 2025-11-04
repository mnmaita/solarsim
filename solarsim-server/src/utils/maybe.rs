// MIT License
// Copyright (c) 2021 Leafwing Studios
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use core::marker::PhantomData;

use bevy::{
    ecs::{
        component::{Immutable, StorageType},
        lifecycle::HookContext,
        system::Command,
        world::DeferredWorld,
    },
    prelude::*,
};

/// A component that when added to an entity, will be removed from the entity and replaced with its contents if [`Some`].
///
/// Under the hood, this is done using component lifecycle hooks.
/// The component is removed from the entity when it is added, and contents are extracted.
/// If the inner value is [`Some`], the contents are then readded to the entity.
///
/// # Example
///
/// ```rust
/// use bevy_ecs::prelude::*;
/// use bevy_ecs::system::RunSystemOnce;
/// use i_cant_believe_its_not_bsn::Maybe;
///
/// #[derive(Component)]
/// struct A;
///
/// #[derive(Bundle)]
/// struct TestBundle {
///    maybe_a: Maybe<A>,
/// }
///
/// let mut world = World::new();
///
/// let entity_with_component = world.run_system_once(|mut commands: Commands| -> Entity {
///     commands
///         .spawn(TestBundle {
///             maybe_a: Maybe::new(Some(A)),
///         })
///         .id()
/// });
/// let entity_ref = world.get_entity(entity_with_component).unwrap();
/// assert!(entity_ref.contains::<A>());
/// assert!(!entity_ref.contains::<Maybe<A>>());
///
/// let entity_without_component = world.run_system_once(|mut commands: Commands| -> Entity {
///     commands
///         .spawn(TestBundle {
///             maybe_a: Maybe::NONE,
///         })
///         .id()
/// });
/// let entity_ref = world.get_entity(entity_without_component).unwrap();
/// assert!(!entity_ref.contains::<A>());
/// assert!(!entity_ref.contains::<Maybe<A>>());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Maybe<B: Bundle>(Option<B>);

impl<B: Bundle> Component for Maybe<B> {
    type Mutability = Immutable;

    /// This is a sparse set component as it's only ever added and removed, never iterated over.
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn on_add() -> Option<bevy::ecs::lifecycle::ComponentHook> {
        Some(maybe_hook::<B>)
    }
}

impl<B: Bundle> Maybe<B> {
    /// Creates a new `Maybe` component of type `B` with no bundle.
    pub const NONE: Self = Self(None);

    /// Creates a new `Maybe` component with the given bundle.
    pub const fn new(bundle: Option<B>) -> Self {
        Self(bundle)
    }

    /// Returns the contents of the `Maybe` component, if any.
    pub fn into_inner(self) -> Option<B> {
        self.0
    }
}

impl<B: Bundle> Default for Maybe<B> {
    /// Defaults to [`Maybe::NONE`].
    fn default() -> Self {
        Self::NONE
    }
}

/// A hook that runs whenever [`Maybe`] is added to an entity.
///
/// Generates a [`MaybeCommand`].
fn maybe_hook<B: Bundle>(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().queue(MaybeCommand {
        entity,
        _phantom: PhantomData::<B>,
    });
}

struct MaybeCommand<B> {
    entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Bundle> Command for MaybeCommand<B> {
    fn apply(self, world: &mut World) {
        let Ok(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("Entity with Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(maybe_component) = entity_mut.take::<Maybe<B>>() else {
            #[cfg(debug_assertions)]
            panic!("Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        if let Some(bundle) = maybe_component.into_inner() {
            entity_mut.insert(bundle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct A;

    #[derive(Bundle)]
    struct TestBundle {
        maybe_a: Maybe<A>,
    }

    #[test]
    fn maybe_some() {
        let mut world = World::new();
        let entity = world
            .spawn(TestBundle {
                maybe_a: Maybe::new(Some(A)),
            })
            .id();

        // FIXME: this should not be needed!
        world.flush();

        assert!(world.get::<A>(entity).is_some());
        assert!(world.get::<Maybe<A>>(entity).is_none());
    }

    #[test]
    fn maybe_none() {
        let mut world = World::new();
        let entity = world
            .spawn(TestBundle {
                maybe_a: Maybe::NONE,
            })
            .id();

        // FIXME: this should not be needed!
        world.flush();

        assert!(world.get::<A>(entity).is_none());
        assert!(world.get::<Maybe<A>>(entity).is_none());
    }

    #[test]
    fn maybe_system() {
        use bevy::ecs::system::RunSystemOnce;

        let mut world = World::new();

        let entity_with_component = world
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(TestBundle {
                        maybe_a: Maybe::new(Some(A)),
                    })
                    .id()
            })
            .unwrap();

        let entity_ref = world.get_entity(entity_with_component).unwrap();
        assert!(entity_ref.contains::<A>());
        assert!(!entity_ref.contains::<Maybe<A>>());

        let entity_without_component = world
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(TestBundle {
                        maybe_a: Maybe::NONE,
                    })
                    .id()
            })
            .unwrap();

        let entity_ref = world.get_entity(entity_without_component).unwrap();
        assert!(!entity_ref.contains::<A>());
        assert!(!entity_ref.contains::<Maybe<A>>());
    }
}
