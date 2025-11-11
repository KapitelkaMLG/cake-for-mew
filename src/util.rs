use bevy::prelude::*;

pub fn despawn_all<C: Component>(mut commands: Commands, to_despawn: Query<Entity, With<C>>) {
    for e in &to_despawn {
        commands.entity(e).despawn();
    }
}

pub trait Animation {
    fn index() -> usize;
    fn size() -> usize;
    fn timer(&mut self) -> &mut Timer;
}

pub fn animate<A: Component<Mutability = bevy::ecs::component::Mutable> + Animation>(
    time: Res<Time>,
    mut query: Query<(&mut A, &mut Sprite)>,
) {
    for (mut animation, mut sprite) in &mut query {
        let timer = animation.timer();
        timer.tick(time.delta());
        if timer.is_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = (atlas.index + 1 - A::index()) % A::size() + A::index();
        }
    }
}
