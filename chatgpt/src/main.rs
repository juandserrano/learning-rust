use bevy::prelude::*;

fn main() {
App::build()
.add_resource(Msaa { samples: 4 })
.add_plugins(DefaultPlugins)
.add_startup_system(setup.system())
.run();
}

fn setup(mut commands: Commands) {
commands.spawn(Camera2dComponents::default());
commands.spawn(TextComponents {
    style: Style {
        background_color: Color::rgb(0.0, 1.0, 0.0),
        ..Default::default()
    },
    text: Text {
        value: "\
                @    @       @       @        @\n\
               @ @   @       @       @        @\n\
              @   @  @       @       @        @\n\
             @ @ @ @ @       @       @        @\n\
            @       @        @       @        @\n\
           @       @         @       @        @\n\
          @       @          @       @        @\n\
         @       @           @       @        @\n\
        @       @            @       @        @\n",
        ..Default::default()
    },
    ..Default::default()
});
}
