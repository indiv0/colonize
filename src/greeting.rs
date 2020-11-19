use bevy::prelude::*;

struct Person;
struct Name(String);

fn add_people(commands: &mut Commands) {
    commands
        .spawn((Person, Name("Elaina Proctor".to_string())))
        .spawn((Person, Name("Renzo Hume".to_string())))
        .spawn((Person, Name("Zayna Nieves".to_string())));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<(&Person, &Name)>) {
    // Update our timer with the time elapsed since the last update.
    timer.0.tick(time.delta_seconds);

    // Check to see if the timer has finished. If it has, print out message.
    if timer.0.finished {
        for (_person, name) in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}

pub(crate) struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}
