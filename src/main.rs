use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ascii_terminal::{code_page_437, prelude::*};
use rand::prelude::ThreadRng;
use rand::Rng;

use bevy_fundsp::prelude::*;


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
        .add_plugins(DspPlugin::default())
        .add_dsp_source(white_noise, SourceType::Dynamic)
        .add_dsp_source(generate_sine_wave, SourceType::Dynamic)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, spam_terminal)
	.add_systems(Update, quit_on_escape)
	.add_systems(PostStartup, play_sine_wave)
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

fn setup_audio(commands: &mut Commands) {
    commands.insert_resource(DspManager::default());
}

fn draw_helptext(mut q: Query<&mut Terminal>) {
    for mut term in q.iter_mut() {
		    let top = term.side_index(bevy_ascii_terminal::Side::Top) as i32;
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
	    //let glyph = random_hiragana_char(&mut rng);
            let elapsed_time = time.elapsed_seconds();
            let hue = (elapsed_time * SHIFT_MULT) % 360.0;
            let bg = Color::hsl(hue, 1.0, 0.5);
            let fg = Color::BLACK;

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

fn quit_on_escape(input: Res<Input<KeyCode>>, mut exit_events: ResMut<Events<AppExit>>) {
    // Check if the Escape key is pressed
    if input.just_pressed(KeyCode::Escape) {
        // Send an exit event to quit the application
        exit_events.send(AppExit);
    }
}

//fn random_latin_char(rng: &mut impl Rng) -> char {
//    // Define the Unicode range you want to select characters from
//    // For example, this generates random characters from the basic Latin block
//    let start = 0x0020 as u32; // Start of the basic Latin block
//    let end = 0x007E as u32;   // End of the basic Latin block
//
//    // Generate a random Unicode code point within the specified range
//    let code_point = rng.gen_range(start..=end);
//
//    // Convert the code point to a character
//    char::from_u32(code_point).unwrap_or('?') // Handle invalid code points gracefully
//}

// bevy_ascii_term can't do unicode, only ascii (duh)
//fn random_hiragana_char(rng: &mut impl Rng) -> char {
//    // Define the Unicode range for Hiragana characters
//    let start = 0x3040 as u32;
//    let end = 0x309F as u32;
//
//    // Generate a random Unicode code point within the specified range
//    let code_point = rng.gen_range(start..=end);
//
//    // Convert the code point to a character
//    char::from_u32(code_point).unwrap_or('?') // Handle invalid code points gracefully
//}

fn white_noise() -> impl AudioUnit32 {
    white() >> split::<U2>() * 0.2
}

fn play_noise(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph(white_noise)
            .unwrap_or_else(|| panic!("DSP source not found!"))
            .clone(),
    );
    commands.spawn(AudioSourceBundle {
        source,
        ..default()
    });
}

fn generate_sine_wave() -> impl AudioUnit32 {
    // Generate a sine wave at 440 Hz (A4)
    sine_hz(440.0) * 0.2
}

fn play_sine_wave(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph(generate_sine_wave)
            .unwrap_or_else(|| panic!("DSP source not found!"))
            .clone(),
    );
    commands.spawn(AudioSourceBundle {
        source,
        ..default()
    });
}
