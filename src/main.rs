use bevy::prelude::*;

const IN_DEVELOPMENT: bool = true;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(Counter::default())
        .add_systems(Startup, on_startup)
        .add_systems(Update, on_update);

    if IN_DEVELOPMENT {
        app.add_systems(Update, exit_on_esc);
    }

    app.run();
}

#[derive(Resource, Default)]
struct Counter(i32);

fn on_startup(counter: Res<Counter>) {
    println!("The counter is: {}", counter.0);
}

fn on_update(keys: Res<ButtonInput<KeyCode>>, mut counter: ResMut<Counter>) {
    if keys.just_pressed(KeyCode::Space) {
        counter.0 += 10;
        println!("Counter updated: {}", counter.0);
    }
}

fn exit_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
