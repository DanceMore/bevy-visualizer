use bevy::app::AppExit;
//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

use bevy_embedded_assets::EmbeddedAssetPlugin;

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_fundsp::prelude::*;

use bevy::render::render_resource::ShaderType;

use bevy::{
    reflect::{TypePath, TypeUuid},
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
    },
};

use bevy::audio::AudioPlugin;

use fundsp::hacker32::*; // Import the appropriate Backend32

use std::ops::DerefMut;
use uuid::Uuid;

// I'm coming back to put more state here
#[allow(dead_code)]
#[derive(Resource, Default)]
struct UiState {
    label: String,
    value: f32,
    loaded_wav: bool,
}

#[derive(Resource, Default)]
struct Pause(bool);

fn main() {
    App::new()
        .init_resource::<Pause>()
        .init_resource::<UiState>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(DspManager::default())
        .insert_resource(ShaderData {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            _pad: 0.0,
        })
        .add_plugins((
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
 //           LogDiagnosticsPlugin::default(),
 //           FrameTimeDiagnosticsPlugin::default(),
        ))
        //.add_plugins(AudioPlugin { global_volume: 0.8 as GlobalVolume})
        .add_plugins(DrawableDspPlugin)
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_plugins(EguiPlugin)
        .add_asset::<CustomMaterial>()
        .add_systems(Startup, setup_scene)
        // examples of other stages I can use
        //.add_systems(Startup, init_assets)
        //.add_systems(PostStartup, play_sine)
        .add_systems(Update, quit_on_escape)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, prepare_my_material)
        .run();
}

// does nothing but I probably need it soon
fn init_assets(mut commands: Commands) {
    //commands.insert_resource(AudioPlugin::default());
    //let handle = assets.add(Sine);
    //commands.insert_resource(SineHandle(handle));
}

fn quit_on_escape(input: Res<Input<KeyCode>>, mut exit_events: ResMut<Events<AppExit>>) {
    // Check if the Escape key is pressed
    if input.just_pressed(KeyCode::Escape) {
        // Send an exit event to quit the application
        exit_events.send(AppExit);
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "b8e1724f-3311-4d4f-a5ad-e167b78436e0"]
struct CustomMaterial {
    #[uniform(0)]
    uniforms: ShaderData,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/haxor.wgsl".into()
    }
}

#[derive(Clone, Debug, TypeUuid, TypePath, ShaderType, Component, Resource)]
#[uuid = "ed5396f9-26cc-4f40-9123-2b302d729ecf"]
struct ShaderData {
    r: f32,
    g: f32,
    b: f32,
    _pad: f32,
}

impl Default for ShaderData {
    fn default() -> Self {
        ShaderData {
            r: 0.1,
            g: 0.0,
            b: 0.0,
            _pad: 0.0,
        }
    }
}

// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut s_materials: ResMut<Assets<StandardMaterial>>,
    mut c_materials: ResMut<Assets<CustomMaterial>>,
    mut shader_data: ResMut<ShaderData>,
    asset_server: Res<AssetServer>,
) {
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 5.0, 4.0),
        ..default()
    });

    // cube1
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.7 })),
        transform: Transform::from_xyz(2.0, 0.5, -1.0),
        material: s_materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        ..default()
    });

    // cube2, shader boogaloo
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        //material: s_materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        material: c_materials.add(CustomMaterial {
            uniforms: ShaderData {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                _pad: 0.0,
            },
        }),
        ..default()
    });

    println!("[-] drawing camera");
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn ui_example_system(
    time: Res<Time>,
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    dsp_manager: Res<DspManager>,
    mut shader_data: ResMut<ShaderData>,
) {
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Side Panel");

            ui.add(egui::Slider::new(&mut ui_state.value, 20.0..=24000.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            // these used to be plumbed directly to the shader data
            // I'll set that up again later
            ui.add(egui::Slider::new(&mut shader_data.r, 0.0..=1.0).text("value"));
            ui.add(egui::Slider::new(&mut shader_data.g, 0.0..=1.0).text("value"));
            ui.add(egui::Slider::new(&mut shader_data.b, 0.0..=1.0).text("value"));
        });

    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn prepare_my_material(
    mut material_assets: ResMut<Assets<CustomMaterial>>,
    mut shader_data: ResMut<ShaderData>,
    mut audio_network: ResMut<AudioNetwork>,
) {
    for (handle, mut material) in material_assets.iter_mut() {
        let material = material.deref_mut(); // Dereference the mutable reference
        let sample = audio_network.backend.get_mono();
        //println!("[+] writing material value: {:?}", sample);
        material.uniforms.r = sample;

        //material.uniforms.r = shader_data.r;
        //material.uniforms.g = shader_data.g;
        //material.uniforms.b = shader_data.b;
    }
}

struct DrawableDsp<F>(F);

impl<T: AudioUnit32 + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for DrawableDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit32> {
        Box::new((self.0)())
    }
}

#[derive(Debug, Resource)]
struct DrawableDspId(Uuid);

use std::sync::Arc;

pub struct DrawableDspPlugin;
impl Plugin for DrawableDspPlugin {
    fn build(&self, app: &mut App) {
        // demonstration of using a more traditional DSP Graph
        //let piano = square_hz(449.0) * 0.2;
        //let piano_clone = piano.clone();

        let wavefile = Wave64::load("test.wav").unwrap();
        let cloned_wave3 = Arc::new(wavefile.clone());
        let piano = bevy_fundsp::prelude::wave64(&cloned_wave3, 0, Some(0));
        let piano_clone = piano.clone();

        let piano_dsp = DrawableDsp(move || piano.clone());
        let piano_id = piano_dsp.id();

        // Wrap the closure in a Box
        let custom_graph: Box<dyn AudioUnit32> = Box::new(piano_clone);

        // Initialize and store the AudioNetwork as a resource.
        println!("[+] setup AudioNetwork...");
        let mut audio_network = AudioNetwork::new();
        audio_network.connect_graph(custom_graph);

        app.add_plugins((DspPlugin::default(),))
            .add_dsp_source(piano_dsp, SourceType::Dynamic)
            .insert_resource(audio_network)
            .insert_resource(DrawableDspId(piano_id))
            .add_systems(PostStartup, drawable_dsp_start);
            //.add_systems(Update, drawable_dsp_debugprint);
    }
}

fn drawable_dsp_start(
    mut cmd: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    dsp_id: Res<DrawableDspId>,
) {
    let source = assets.add(
        dsp_manager
            .get_graph_by_id(&dsp_id.0)
            .unwrap_or_else(|| panic!("DSP source not found!")),
    );
    cmd.spawn(AudioSourceBundle {
        source,
        ..default()
    });
}

fn drawable_dsp_debugprint(mut cmd: Commands, mut audio_network: ResMut<AudioNetwork>) {
    let backend = audio_network.backend.get_stereo();
    println!("backend data: {:?}", backend);

    let frontend = audio_network.frontend.get_stereo();
    println!("frontend data: {:?}", frontend);
}

// AudioNetwork, using FunDSP Network object
// this lets me share data between playback (backend??) and shader (frontend??)
// in a sensible manner
#[derive(Resource)]
struct AudioNetwork {
    frontend: fundsp::hacker32::Net32,
    backend: fundsp::hacker32::NetBackend32,
}

impl AudioNetwork {
    fn new() -> Self {
        let mut frontend = fundsp::hacker32::Net32::new(0, 1);
        let mut backend = frontend.backend();

        AudioNetwork { frontend, backend }
    }

    // TODO: maybe rename this idk
    // basically just chains the DSP Graph into itself for sharing
    pub fn connect_graph(&mut self, custom_graph: Box<dyn AudioUnit32>) {
        let _noise_id = self.frontend.chain(custom_graph);
        self.frontend.commit();
    }
}
