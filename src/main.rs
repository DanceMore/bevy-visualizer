use bevy::app::AppExit;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_ascii_terminal::{code_page_437, prelude::*};
//use rand::prelude::ThreadRng;
//use rand::Rng;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_fundsp::prelude::*;

fn main() {
    App::new()
        .init_resource::<Pause>()
        .init_resource::<UiState>()
        //.init_resource::<DspData>()
        .insert_resource(DspManager::default())
        .insert_resource(CurrentSpamFunction { index: 0 }) // Initialize with your default function
	//.insert_resource(MyShaderData {
	//	r: 0.0,
	//	g: 0.0,
	//	b: 0.0,
	//})
        .add_plugins((
            DefaultPlugins,
            TerminalPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_plugins(DspPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(PianoPlugin)
        .insert_resource(ClearColor(Color::BLACK))
	.add_asset::<CustomMaterial>()
        .add_systems(Startup, setup)
        //.add_systems(Update, spam_terminal)
        .add_systems(Update, quit_on_escape)
        .add_systems(Update, ui_example_system)
        //.add_systems(Update, sync_data)
        //.add_systems(Update, update_dsp_data)
        .run();
}

//fn sync_data(ui_state: ResMut<UiState>,
//             mut shader_data: ResMut<MyShaderData>,
//) {
//	shader_data.r = ui_state.r.clone() as f32;
//	shader_data.g = ui_state.g.clone() as f32;
//	shader_data.b = ui_state.b.clone() as f32;
//
//	//println!("[-] {:?}", shader_data);
//}

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    reflect::{TypePath, TypeUuid},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};


use bevy::asset::LoadState;

#[derive(Reflect, TypeUuid, AsBindGroup, Debug, Clone)]
#[uuid = "b8e1724f-3311-4d4f-a5ad-e167b78436e0"]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        //"shaders/animate_shader.wgsl".into()
        "shaders/haxor.wgsl".into()
    }

    //fn vertex_shader() -> ShaderRef {
    //    "shaders/haxor.vert".into()
    //}

    //fn fragment_shader() -> ShaderRef {
    //    //"shaders/custom_material.frag".into()
    //    "shaders/haxor.frag".into()
    //}

    //// Bevy assumes by default that vertex shaders use the "vertex" entry point
    //// and fragment shaders use the "fragment" entry point (for WGSL shaders).
    //// GLSL uses "main" as the entry point, so we must override the defaults here
    //fn specialize(
    //    _pipeline: &MaterialPipeline<Self>,
    //    descriptor: &mut RenderPipelineDescriptor,
    //    _layout: &MeshVertexBufferLayout,
    //    _key: MaterialPipelineKey<Self>,
    //) -> Result<(), SpecializedMeshPipelineError> {
    //    descriptor.vertex.entry_point = "main".into();
    //    descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
    //    Ok(())
    //}
}

// This is the struct that will be passed to your shader
#[repr(C)]
#[derive(AsBindGroup,Component)]
struct MyShaderData {
  #[uniform(0)]
  r: f32,
  g: f32,
  b: f32,
  _pad: f32,
}

impl Default for MyShaderData {
    fn default() -> Self {
        MyShaderData {
            r: 0.1,
	    g: 0.0,
	    b: 0.0,
	    _pad: 0.0,

        }
    }
}

//#[derive(Resource, Default)]
//struct MyShaderData {
//    some_value: f32,
//    // Add more fields as needed
//}

// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	//mut materials: ResMut<Assets<CustomMaterial>>,
	mut s_materials: ResMut<Assets<StandardMaterial>>,
	mut c_materials: ResMut<Assets<CustomMaterial>>,
	    asset_server: Res<AssetServer>,
	) {

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });

   // cube1
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(2.0, 0.5, -1.0),
	material: s_materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..default()
    });


   // cube2, shader boogaloo
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
	//material: s_materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        material: c_materials.add(CustomMaterial {} ),
        ..default()
    });


    println!("[-] drawing camera");
	// camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});

}

fn ui_example_system(time: Res<Time>, mut contexts: EguiContexts,     mut ui_state: ResMut<UiState>,
		     dsp_manager: Res<DspManager>,
		     //mut shader_data: ResMut<MyShaderData>,
		     ) {

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.add(egui::Slider::new(&mut ui_state.value, 20.0..=24000.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.add(egui::Slider::new(&mut ui_state.shader_data.r, 0.0..=100000.0).text("value"));
            ui.add(egui::Slider::new(&mut ui_state.shader_data.g, 0.0..=1.0).text("value"));
            ui.add(egui::Slider::new(&mut ui_state.shader_data.b, 0.0..=1.0).text("value"));

	    //example_plot(ui, &time, &[1.0,2.0,3.0]);
	    //example_plot(ui, &time);
	    //println!("[+] drew boring plot");
	//    plot_dsp(ui, &time, dsp_manager);
        });

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

#[derive(Resource, Default)]
struct UiState {
    label: String,
    value: f32,
    shader_data: MyShaderData,
}

#[derive(Resource, Default)]
struct Pause(bool);

#[derive(Resource)]
struct CurrentSpamFunction {
    index: usize,
}

//fn setup(mut commands: Commands,
//) {
//    commands.spawn((
//        TerminalBundle::new()
//            .with_size([80, 50])
//            .with_border(Border::single_line()),
//        AutoCamera,
//    ));
//}

fn setup_audio(commands: &mut Commands) {
    commands.insert_resource(DspManager::default());
}

fn quit_on_escape(input: Res<Input<KeyCode>>, mut exit_events: ResMut<Events<AppExit>>) {
    // Check if the Escape key is pressed
    if input.just_pressed(KeyCode::Escape) {
        // Send an exit event to quit the application
        exit_events.send(AppExit);
    }
}

use uuid::Uuid;

struct PianoPlugin;

struct PianoDsp<F>(F);

impl<T: AudioUnit32 + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for PianoDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit32> {
        Box::new((self.0)())
    }
}

#[derive(Debug, Resource)]
struct PianoId(Uuid);

#[derive(Resource)]
struct PitchVar(Shared<f32>);

impl PitchVar {
    fn set_pitch(&self, pitch: Pitch) {
        self.0.set_value(pitch.into());
    }
}

impl PitchVar {
    fn set_freq(&self, freq: f32) {
        self.0.set_value(freq);
    }
}

#[derive(Debug, Clone, Copy)]
enum Pitch {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Pitch {
    fn to_f32(self) -> f32 {
        match self {
            Pitch::C => 261.626,
            Pitch::Cs => 277.183,
            Pitch::D => 293.665,
            Pitch::Ds => 311.127,
            Pitch::E => 329.628,
            Pitch::F => 349.228,
            Pitch::Fs => 369.994,
            Pitch::G => 391.995,
            Pitch::Gs => 415.305,
            Pitch::A => 440.0,
            Pitch::As => 466.164,
            Pitch::B => 493.883,
        }
    }
}

impl From<Pitch> for f32 {
    fn from(pitch: Pitch) -> Self {
        pitch.to_f32()
    }
}

impl Plugin for PianoPlugin {
    fn build(&self, app: &mut App) {
        let pitch = shared(Pitch::C.into());
        let pitch2 = pitch.clone();

        let piano = move || var(&pitch2) >> sine() >> split::<U2>() * 0.2;
        let piano_dsp = PianoDsp(piano.clone());
        let piano_id = piano_dsp.id();

        app.add_dsp_source(piano_dsp, SourceType::Dynamic)
            .insert_resource(PitchVar(pitch))
            .insert_resource(PianoId(piano_id))
            .add_systems(Update, switch_key)
            .add_systems(PostStartup, play_piano);
    }
}

fn switch_key(input: Res<Input<KeyCode>>, pitch_var: Res<PitchVar>, ui_state: Res<UiState>) {
    pitch_var.set_freq(ui_state.value);

    let keypress = |keycode, pitch| {
        if input.just_pressed(keycode) {
            pitch_var.set_pitch(pitch)
        }
    };

    keypress(KeyCode::A, Pitch::C);
    keypress(KeyCode::W, Pitch::Cs);
    keypress(KeyCode::S, Pitch::D);
    keypress(KeyCode::E, Pitch::Ds);
    keypress(KeyCode::D, Pitch::E);
    keypress(KeyCode::F, Pitch::F);
    keypress(KeyCode::T, Pitch::Fs);
    keypress(KeyCode::G, Pitch::G);
    keypress(KeyCode::Y, Pitch::Gs);
    keypress(KeyCode::H, Pitch::A);
    keypress(KeyCode::U, Pitch::As);
    keypress(KeyCode::J, Pitch::B);
}

fn play_piano(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    piano_id: Res<PianoId>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph_by_id(&piano_id.0)
            .unwrap_or_else(|| panic!("DSP source not found!")),
    );
    commands.spawn(AudioSourceBundle {
        source,
        ..default()
    });
}
