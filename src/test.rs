//! Testing grounds for better understanding bevy
use bevy::prelude::*;

pub struct ConditionalPlugin {
    // condition: Box<dyn Fn() -> dyn ReadOnlySystem<In = (), Out = bool>>,
}

impl Plugin for ConditionalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hello.run_if(every_other_time()));
    }
}

fn hello() {
    print!("Hello!")
}

fn always_fail() -> bool {
    false
}

fn every_other_time() -> impl Condition<()> {
    IntoSystem::into_system(|mut flag: Local<bool>| {
        *flag = !*flag;
        *flag
    })
}

fn main() {
    let mut app = App::new();
    app.set_runner(move |mut app: App| {
        for _ in 0..4 {
            app.update();
        }
    });
    app.add_plugins(ConditionalPlugin {
        // condition: Box::new(always_fail),
    });
    app.run();
}
