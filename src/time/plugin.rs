use bevy::{prelude::*, time::TimeSystems};

use crate::time::{PendingSteps, TickRate, TimeStartupSet, TimeUpdateSet, spawn_menu, step_time, update_tick_rate};

pub(crate) struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Startup, TimeStartupSet::Spawn)
            .configure_sets(Update, (TimeUpdateSet::Input, TimeUpdateSet::Sync).chain())
            .init_resource::<TickRate>()
            .init_resource::<PendingSteps>()
            .add_systems(Startup, spawn_menu.in_set(TimeStartupSet::Spawn))
            .add_systems(
                First,
                update_tick_rate.run_if(resource_changed::<TickRate>).before(TimeSystems),
            )
            .add_systems(First, step_time.after(TimeSystems));
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::{time::TimeUpdateStrategy, ui_widgets::Activate};

    use super::*;

    #[derive(Resource, Default)]
    struct Ticks(u32);

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(10)))
            .init_resource::<Ticks>()
            .add_systems(FixedUpdate, |mut ticks: ResMut<Ticks>| ticks.0 += 1);
        app.update();
        app
    }

    fn activate_button(app: &mut App, label: &str) {
        let mut labels = app.world_mut().query::<(&Text, &ChildOf)>();
        let entity = labels
            .iter(app.world())
            .find_map(|(text, parent)| (text.0 == label).then_some(parent.parent()))
            .expect("time button should exist");
        app.world_mut().trigger(Activate { entity });
    }

    #[test]
    fn pause_stops_ticks_and_each_request_advances_one_tick() {
        let mut app = test_app();
        // Leave a partial tick in the accumulator before pausing.
        app.update();
        app.update();
        let remainder = app.world().resource::<Time<Fixed>>().overstep();
        assert_eq!(remainder, Duration::from_millis(20));

        activate_button(&mut app, "x0");
        for _ in 0..10 {
            app.update();
        }
        assert_eq!(app.world().resource::<Ticks>().0, 0);
        assert_eq!(app.world().resource::<Time<Virtual>>().delta(), Duration::ZERO);
        assert_eq!(app.world().resource::<Time<Real>>().delta(), Duration::from_millis(10));

        // Even a long render frame must not turn one request into several ticks.
        app.insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(250)));
        activate_button(&mut app, "Step");
        activate_button(&mut app, "Step");
        for expected in 1..=2 {
            app.update();
            assert_eq!(app.world().resource::<Ticks>().0, expected);
            assert!(app.world().resource::<Time<Virtual>>().is_paused());
            assert_eq!(app.world().resource::<Time<Fixed>>().delta(), Duration::from_millis(50));
            assert_eq!(app.world().resource::<Time<Fixed>>().overstep(), remainder);
        }
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 2);
        assert_eq!(app.world().resource::<Time<Virtual>>().delta(), Duration::ZERO);
    }

    #[test]
    fn resume_uses_selected_rate_without_catching_up_paused_time() {
        let mut app = test_app();
        activate_button(&mut app, "x0");
        for _ in 0..100 {
            app.update();
        }
        // Resuming also cancels any outstanding step requests.
        activate_button(&mut app, "Step");
        activate_button(&mut app, "x2");
        app.update();
        assert_eq!(app.world().resource::<Ticks>().0, 0);
        assert_eq!(app.world().resource::<PendingSteps>().0, 0);
        for _ in 0..9 {
            app.update();
        }
        assert_eq!(app.world().resource::<Ticks>().0, 1);
        assert_eq!(app.world().resource::<Time<Fixed>>().timestep(), Duration::from_millis(100));
    }
}
