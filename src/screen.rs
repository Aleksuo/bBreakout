use bevy::prelude::*;

#[derive(Component)]
pub struct ScreenRoot;

#[derive(Resource)]
pub struct CurrentScreenRoot {
    world_root: Option<Entity>,
    ui_root: Option<Entity>,
}

pub struct SpawnOnCurrentWorldRoot<T>(pub T)
where
    T: Bundle;

pub struct SpawnOnCurrentUiRoot<T>(pub T)
where
    T: Bundle;

impl<T: Bundle> Command for SpawnOnCurrentWorldRoot<T> {
    fn apply(self, world: &mut World) {
        let screen_root = world.resource::<CurrentScreenRoot>();
        if let Some(root) = screen_root.world_root {
            world.entity_mut(root).with_children(|parent| {
                parent.spawn(self.0);
            });
        }
    }
}

impl<T: Bundle> Command for SpawnOnCurrentUiRoot<T> {
    fn apply(self, world: &mut World) {
        let screen_root = world.resource::<CurrentScreenRoot>();
        if let Some(root) = screen_root.ui_root {
            world.entity_mut(root).with_children(|parent| {
                parent.spawn(self.0);
            });
        }
    }
}

pub trait SpawnOnWorldRootExt {
    fn spawn_on_world_root(&mut self, bundle: impl Bundle);
}

pub trait SpawnOnUiRootExt {
    fn spawn_on_ui_root(&mut self, bundle: impl Bundle);
}

impl<'w, 's> SpawnOnWorldRootExt for Commands<'w, 's> {
    fn spawn_on_world_root(&mut self, bundle: impl Bundle) {
        self.queue(SpawnOnCurrentWorldRoot(bundle));
    }
}

impl<'w, 's> SpawnOnUiRootExt for Commands<'w, 's> {
    fn spawn_on_ui_root(&mut self, bundle: impl Bundle) {
        self.queue(SpawnOnCurrentUiRoot(bundle));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(CurrentScreenRoot {
        world_root: None,
        ui_root: None,
    });
}

pub fn spawn_screen(mut commands: Commands, mut root_res: ResMut<CurrentScreenRoot>) {
    if root_res.world_root.eq(&None) || root_res.ui_root.eq(&None) {
        let world_root = commands
            .spawn((ScreenRoot, Transform::default(), Visibility::default()))
            .id();
        root_res.world_root = Some(world_root);

        let ui_root = commands.spawn((ScreenRoot, Node::default())).id();
        root_res.ui_root = Some(ui_root);
    } else {
        error!("Tried to spawn screen root when screen root already exists");
    }
}

pub fn despawn_screen(mut commands: Commands, mut root_res: ResMut<CurrentScreenRoot>) {
    if let Some(world_root) = root_res.world_root {
        commands.entity(world_root).despawn();
        root_res.world_root = None;
    }

    if let Some(ui_root) = root_res.ui_root {
        commands.entity(ui_root).despawn();
        root_res.ui_root = None;
    }
}
