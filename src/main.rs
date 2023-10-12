use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ascii_terminal::{code_page_437, prelude::*};
use rand::prelude::ThreadRng;
use rand::Rng;

fn main() {
    App::new()
        .init_resource::<Pause>()
        .insert_resource(CurrentSpamFunction { index: 0 }) // Initialize with your default function
        .add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, spam_terminal)
        .run();
}

#[derive(Resource, Default)]
struct Pause(bool);

#[derive(Resource)]
struct CurrentSpamFunction {
    index: usize,
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TerminalBundle::new()
            .with_size([80, 50])
            .with_border(Border::single_line()),
        AutoCamera,
    ));
}

fn rand_color(rng: &mut ThreadRng) -> Color {
    let r: f32 = rng.gen_range(0.0..=1.0);
    let g: f32 = rng.gen_range(0.0..=1.0);
    let b: f32 = rng.gen_range(0.0..=1.0);
    Color::rgb(r, g, b)
}

fn spam_terminal_a(mut q: Query<&mut Terminal>) {
    let mut rng = rand::thread_rng();
    for mut term in q.iter_mut() {
        for t in term.iter_mut() {
            let index = rng.gen_range(0..=255) as u8;
            let glyph = code_page_437::index_to_glyph(index);
            let fg = rand_color(&mut rng);
            let bg = rand_color(&mut rng);

            *t = Tile {
                glyph,
                fg_color: fg,
                bg_color: bg,
            }
        }
        let top = term.side_index(Side::Top) as i32;
        term.clear_box([0, top], [25, 1]);
        term.put_string([0, top], "Press space to pause");
    }
}

fn spam_terminal_b(mut q: Query<&mut Terminal>) {
    let mut rng = rand::thread_rng();
    let color_palette = [
        Color::RED,
        Color::GREEN,
        Color::BLUE,
        Color::YELLOW,
        Color::CYAN,
        Color::WHITE,
    ];

    for (mut term, color) in q.iter_mut().zip(color_palette.iter()) {
        let mut current_color_index = 0;

        for t in term.iter_mut() {
            let index = rng.gen_range(0..=255) as u8;
            let glyph = code_page_437::index_to_glyph(index);
            let fg = color_palette[current_color_index];
            let bg = Color::BLACK; // Set background color to black

            *t = Tile {
                glyph,
                fg_color: fg,
                bg_color: bg,
            };

            current_color_index = (current_color_index + 1) % color_palette.len();
        }

        let top = term.side_index(Side::Top) as i32;
        term.clear_box([0, top], [25, 1]);
        term.put_string([0, top], "Press space to pause");
    }
}

type SpamFunction = fn(Query<&mut Terminal>);

fn spam_terminal(
    keys: Res<Input<KeyCode>>,
    mut pause: ResMut<Pause>,
    mut q: Query<&mut Terminal>,
    mut spammer: ResMut<CurrentSpamFunction>,
) {
    let spam_functions: Vec<SpamFunction> = vec![spam_terminal_a, spam_terminal_b];

    if keys.just_pressed(KeyCode::Space) {
        pause.0 = !pause.0;
    }

    if pause.0 {
        return;
    }

    //spam_terminal_b(q);

    if keys.just_pressed(KeyCode::Right) {
        spammer.index = (spammer.index + 1) % spam_functions.len();
    } else if keys.just_pressed(KeyCode::Left) {
        spammer.index = (spammer.index + spam_functions.len() - 1) % spam_functions.len();
    }

    // Call the selected spam_terminal_X function based on the current_spam_function
    spam_functions[spammer.index](q);
}
