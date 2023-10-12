use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ascii_terminal::{code_page_437, prelude::*};
use rand::prelude::ThreadRng;
use rand::Rng;
use bevy::app::AppExit;

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
	.add_systems(Update, quit_on_escape)
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

fn draw_helptext(mut q: Query<&mut Terminal>) {
    for mut term in q.iter_mut() {
		    let top = term.side_index(Side::Top) as i32;
		    term.clear_box([0, top], [30, 1]);
		    term.put_string([0, top], "Press space to pause");
		    term.clear_box([0, top-1], [30, 1]);
		    term.put_string([0, top-1], "Left / Right to change effect");
	    }
}

fn rand_color(rng: &mut ThreadRng) -> Color {
    let r: f32 = rng.gen_range(0.0..=1.0);
    let g: f32 = rng.gen_range(0.0..=1.0);
    let b: f32 = rng.gen_range(0.0..=1.0);
    Color::rgb(r, g, b)
}

fn spam_terminal_a(_time: Res<Time>, mut q: Query<&mut Terminal>) {
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
    }

    draw_helptext(q);
}

fn spam_terminal_b(_time: Res<Time>, mut q: Query<&mut Terminal>) {
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

    }
    draw_helptext(q);
}

fn spam_terminal_c(time: Res<Time>, mut q: Query<&mut Terminal>) {
    let mut rng = rand::thread_rng();
    const SHIFT_MULT : f32 = 60.0;

    for mut term in q.iter_mut() {
        for t in term.iter_mut() {
            let index = rng.gen_range(0..=255) as u8;
            let glyph = code_page_437::index_to_glyph(index);
            let elapsed_time = time.elapsed_seconds();
            let hue = (elapsed_time * SHIFT_MULT) % 360.0;
            let fg = Color::hsl(hue, 1.0, 0.5);
            let bg = Color::BLACK;

            *t = Tile {
                glyph,
                fg_color: fg,
                bg_color: bg,
            }
        }
    }

    draw_helptext(q);
}


type SpamFunction = fn(time: Res<Time>, Query<&mut Terminal>);

fn spam_terminal(
    keys: Res<Input<KeyCode>>,
    mut pause: ResMut<Pause>,
    q: Query<&mut Terminal>,
    mut spammer: ResMut<CurrentSpamFunction>,
    time: Res<Time>, // Add this line to include Time resource
) {
    let spam_functions: Vec<SpamFunction> = vec![spam_terminal_a, spam_terminal_b, spam_terminal_c];

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

    spam_functions[spammer.index](time, q);
}

fn quit_on_escape(
    input: Res<Input<KeyCode>>,
    mut exit_events: ResMut<Events<AppExit>>,
) {
    // Check if the Escape key is pressed
    if input.just_pressed(KeyCode::Escape) {
        // Send an exit event to quit the application
        exit_events.send(AppExit);
    }
}
