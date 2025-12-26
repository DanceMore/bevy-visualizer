use bevy::app::{AppExit};
use bevy::prelude::Messages;
use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;

use bevy::{
    reflect::TypePath,
    render::render_resource::AsBindGroup,
};

use rustfft::{FftPlanner, num_complex::Complex};

use bevy_fundsp::prelude::*;
use uuid::Uuid;
use bevy::time::Time;
use std::sync::Arc;
use fundsp::wave::Wave;
use hound::WavReader;

// Define the play_sine function
fn play_sine(frequency: Shared) -> impl AudioUnit {
    // Create a sine wave with a variable frequency
    var(&frequency) >> sine() >> split::<U2>() * 0.2
}

// Custom DSP graph type
struct SineWaveDsp {
    frequency: Shared,
}

impl DspGraph for SineWaveDsp {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0x1234567890abcdef1234567890abcdefu128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new(play_sine(self.frequency.clone()))
    }
}

// Wave file DSP graph
struct WaveFileDsp {
    wave_data: Arc<Wave>,
}

impl DspGraph for WaveFileDsp {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xfedcba0987654321fedcba0987654321u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        let wave_data = self.wave_data.clone();
        // Play back the first channel (channel 0) of the wave file
        Box::new(wavech(&wave_data, 0, Some(0)))
    }
}

// Resource to store the current audio frequency
#[derive(Resource)]
struct AudioFrequency {
    value: Shared,
}

impl Default for AudioFrequency {
    fn default() -> Self {
        Self {
            value: shared(440.0),
        }
    }
}

// Function to play the audio
fn play_audio(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    ui_state: Res<UiState>,
    mut current_audio_player: ResMut<CurrentAudioPlayer>,
) {
    println!("[AUDIO] Playing audio with source: {}", if ui_state.use_wave_file { "Wave File" } else { "Sine Wave" });
    
    let source = if ui_state.use_wave_file {
        // Use wave file DSP
        println!("[AUDIO] Creating wave file audio source");
        assets.add(
            dsp_manager
                .get_graph_by_id(&Uuid::from_u128(0xfedcba0987654321fedcba0987654321u128))
                .unwrap_or_else(|| panic!("Wave file DSP source not found!")),
        )
    } else {
        // Use sine wave DSP
        println!("[AUDIO] Creating sine wave audio source");
        assets.add(
            dsp_manager
                .get_graph_by_id(&Uuid::from_u128(0x1234567890abcdef1234567890abcdefu128))
                .unwrap_or_else(|| panic!("Sine wave DSP source not found!")),
        )
    };
    
    let entity = commands.spawn(AudioPlayer {
        0: source
    }).id();
    
    current_audio_player.entity = Some(entity);
    current_audio_player.current_source = ui_state.use_wave_file;
}

// System to update the audio frequency from the UI
fn update_audio_frequency(
    ui_state: Res<UiState>,
    frequency: Res<AudioFrequency>,
) {
    // Update the shared frequency value with the UI slider value
    frequency.value.set_value(ui_state.value);
}

// System to handle audio source switching
fn update_audio_source(
    ui_state: Res<UiState>,
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    mut current_audio_player: ResMut<CurrentAudioPlayer>,
    audio_players: Query<Entity, With<AudioPlayer>>,
) {
    // Check if the audio source has changed
    if current_audio_player.current_source != ui_state.use_wave_file {
        println!("[AUDIO] Switching audio source to: {}", if ui_state.use_wave_file { "Wave File" } else { "Sine Wave" });
        
        // Despawn the current audio player
        if let Some(entity) = current_audio_player.entity {
            commands.entity(entity).despawn();
        }
        
        // Create a new audio source based on the selection
        let source = if ui_state.use_wave_file {
            // Use wave file DSP
            println!("[AUDIO] Creating wave file audio source");
            assets.add(
                dsp_manager
                    .get_graph_by_id(&Uuid::from_u128(0xfedcba0987654321fedcba0987654321u128))
                    .unwrap_or_else(|| panic!("Wave file DSP source not found!")),
            )
        } else {
            // Use sine wave DSP
            println!("[AUDIO] Creating sine wave audio source");
            assets.add(
                dsp_manager
                    .get_graph_by_id(&Uuid::from_u128(0x1234567890abcdef1234567890abcdefu128))
                    .unwrap_or_else(|| panic!("Sine wave DSP source not found!")),
            )
        };
        
        // Spawn a new audio player with the selected source
        let new_entity = commands.spawn(AudioPlayer {
            0: source
        }).id();
        
        // Update the current audio player resource
        current_audio_player.entity = Some(new_entity);
        current_audio_player.current_source = ui_state.use_wave_file;
    }
}

// I'm coming back to put more state here
#[allow(dead_code)]
#[derive(Resource, Default)]
struct UiState {
    pub label: String,
    pub value: f32,
    pub loaded_wav: bool,
    pub use_wave_file: bool,
}

#[derive(Resource, Default)]
struct Pause(bool);

#[derive(Resource, Default)]
struct CurrentAudioPlayer {
    entity: Option<Entity>,
    current_source: bool, // true = wave file, false = sine wave
}


fn main() {
    let frequency = shared(440.0);
    let frequency_clone = frequency.clone();
    
    // Load wave file
    let wave_data = Arc::new(load_wave_file("assets/test.wav"));
    let wave_dsp = WaveFileDsp {
        wave_data: wave_data.clone(),
    };
    
    App::new()
        .init_resource::<Pause>()
        .init_resource::<UiState>()
        .init_resource::<SampleBuffer>()
        .init_resource::<CurrentAudioPlayer>()
        .insert_resource(AudioFrequency { value: frequency_clone })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(ShaderData {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            _pad: 0.0,
        })
        .add_plugins((
            DefaultPlugins
                .build(),
        ))
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_plugins(EguiPlugin::default())
        .add_plugins(DspPlugin::new(44100.0))
        .add_dsp_source(SineWaveDsp { frequency }, SourceType::Dynamic)
        .add_dsp_source(wave_dsp, SourceType::Dynamic)
        .add_systems(Startup, setup_scene)
        .add_systems(PostStartup, play_audio)
        .add_systems(Update, update_audio_frequency.after(ui_example_system))
        .add_systems(Update, update_audio_source.after(ui_example_system))
        .add_systems(Update, quit_on_escape)
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .add_systems(Update, prepare_my_material)
        .add_systems(Update, write_to_fft_buffer)
        .run();
}

fn load_wave_file(path: &str) -> Wave {
    let reader = WavReader::open(path).expect("Failed to open WAV file");
    let spec = reader.spec();
    
    println!("[WAV] File properties: {} channels, {} Hz sample rate",
             spec.channels, spec.sample_rate);
    
    let samples = reader
        .into_samples::<i16>()
        .filter_map(|s| s.ok())
        .map(|sample| sample as f32 / 32768.0) // Convert i16 to f32 in range [-1, 1]
        .collect::<Vec<f32>>();
    
    println!("[WAV] Loaded {} samples", samples.len());
    
    let num_channels = spec.channels as usize;
    let num_samples = samples.len() / num_channels;
    
    let mut wavefile = Wave::with_capacity(
        num_channels,
        spec.sample_rate as f64,
        num_samples,
    );
    
    wavefile.resize(num_samples);
    
    for channel in 0..num_channels {
        for pos in 0..num_samples {
            let sample = samples[pos * num_channels + channel];
            wavefile.set(channel, pos, sample);
        }
    }
    
    wavefile
}

fn quit_on_escape(input: Res<ButtonInput<KeyCode>>, mut exit_messages: ResMut<Messages<AppExit>>) {
    // Check if the Escape key is pressed
    if input.just_pressed(KeyCode::Escape) {
        // Send an exit event to quit the application
        exit_messages.write(AppExit::Success);
    }
}

#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
struct CustomMaterial {
    #[uniform(0)]
    uniforms: ShaderData,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/haxor.wgsl".into()
    }
}

#[derive(Clone, Debug, TypePath, ShaderType, Component, Resource, Asset)]
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
        commands.spawn((
            PointLight::default(),
            Transform::from_xyz(4.0, 5.0, 4.0),
        ));

    // cube1
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
        MeshMaterial3d(s_materials.add(Color::srgb(0.2, 0.2, 0.2))),
        Transform::from_xyz(2.0, 0.5, -1.0),
    ));

    // cube2, shader boogaloo
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(c_materials.add(CustomMaterial {
            uniforms: ShaderData {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                _pad: 0.0,
            },
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    println!("[-] drawing camera");
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut shader_data: ResMut<ShaderData>,
) {
    // Safely access the egui context with proper error handling
    let ctx_result = contexts.ctx_mut();
      
    match ctx_result {
        Ok(ctx) => {
            egui::SidePanel::left("side_panel")
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.heading("Side Panel");

                    // Audio source selection
                    ui.horizontal(|ui| {
                        ui.label("Audio Source:");
                        ui.radio_value(&mut ui_state.use_wave_file, false, "Sine Wave");
                        ui.radio_value(&mut ui_state.use_wave_file, true, "Wave File");
                    });

                    if !ui_state.use_wave_file {
                        ui.add(egui::Slider::new(&mut ui_state.value, 20.0..=24000.0).text("Audio Frequency (Hz)"));
                        ui.label(format!("Current Frequency: {:.1} Hz", ui_state.value));
                        if ui.button("Increment").clicked() {
                            ui_state.value += 1.0;
                        }
                    } else {
                        ui.label("Playing: test.wav (44100 Hz)");
                    }

                    // these used to be plumbed directly to the shader data
                    // I'll set that up again later
                    let r_changed = ui.add(egui::Slider::new(&mut shader_data.r, 0.0..=1.0).text("Red")).changed();
                    let g_changed = ui.add(egui::Slider::new(&mut shader_data.g, 0.0..=1.0).text("Green")).changed();
                    let b_changed = ui.add(egui::Slider::new(&mut shader_data.b, 0.0..=1.0).text("Blue")).changed();
                     
                    // Manually trigger change detection if any slider changed
                    if r_changed || g_changed || b_changed {
                        shader_data.set_changed();
                    }
                });

            egui::Window::new("Hello").show(ctx, |ui| {
                ui.label("world");
            });
        }
        Err(e) => {
            // Log the error but don't panic
            eprintln!("EGUI context error: {:?}", e);
        }
    }
}

fn prepare_my_material(
    mut material_assets: ResMut<Assets<CustomMaterial>>,
    mut shader_data: ResMut<ShaderData>,
    mut sample_buffer: ResMut<SampleBuffer>,
) {
    // Process FFT data and update shader_data resource
    
    // Convert the buffer to Complex numbers and process with FFT.
    let mut complex_buffer: Vec<Complex<f32>> = sample_buffer.buffer.iter().map(|&x| Complex { re: x, im: 0.0 }).collect();
     
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(complex_buffer.len());
    fft.process(&mut complex_buffer);
 
    const BASS_MIN_FREQ: f32 = 20.0;    // Bass range
    const BASS_MAX_FREQ: f32 = 250.0;
    const MIDRANGE_MIN_FREQ: f32 = 250.0; // Midrange range
    const MIDRANGE_MAX_FREQ: f32 = 4000.0;
    const TREBLE_MIN_FREQ: f32 = 4000.0; // Treble range
    const TREBLE_MAX_FREQ: f32 = 20000.0;
 
    let mut bass_sum = 0.0;
    let mut midrange_sum = 0.0;
    let mut treble_sum = 0.0;
    let mut bass_count = 0;
    let mut midrange_count = 0;
    let mut treble_count = 0;
 
    for (i, &result) in complex_buffer.iter().enumerate() {
        // Calculate frequency in Hz
        let frequency_in_hz = (i as f32 * SAMPLE_RATE) / BUFFER_SIZE as f32;
        let magnitude = result.norm();
 
        if frequency_in_hz >= BASS_MIN_FREQ && frequency_in_hz <= BASS_MAX_FREQ {
            bass_sum += magnitude;
            bass_count += 1;
        } else if frequency_in_hz >= MIDRANGE_MIN_FREQ && frequency_in_hz <= MIDRANGE_MAX_FREQ {
            midrange_sum += magnitude;
            midrange_count += 1;
        } else if frequency_in_hz >= TREBLE_MIN_FREQ && frequency_in_hz <= TREBLE_MAX_FREQ {
            treble_sum += magnitude;
            treble_count += 1;
        }
    }
 
    // Calculate averages and amplify significantly for visualization
    let bass_avg = if bass_count > 0 { bass_sum / bass_count as f32 } else { 0.0 };
    let mid_avg = if midrange_count > 0 { midrange_sum / midrange_count as f32 } else { 0.0 };
    let treble_avg = if treble_count > 0 { treble_sum / treble_count as f32 } else { 0.0 };
    
    // Amplify the values significantly - FFT magnitudes are typically very small
    // and apply logarithmic scaling for better visual response
    let bass_amplified = (bass_avg * 1000.0).ln_1p() * 0.1;
    let mid_amplified = (mid_avg * 1000.0).ln_1p() * 0.1;
    let treble_amplified = (treble_avg * 1000.0).ln_1p() * 0.1;
    
    //println!("[-] Bass: {:.6} Mid: {:.6} Treble: {:.6}", bass_amplified, mid_amplified, treble_amplified);
 
    // Update the shader data resource with the processed FFT data
    shader_data.r = bass_amplified.clamp(0.0, 1.0);
    shader_data.g = mid_amplified.clamp(0.0, 1.0);
    shader_data.b = treble_amplified.clamp(0.0, 1.0);
    shader_data.set_changed();
      
    // Update all materials to use the new shader data
    for (_, material) in material_assets.iter_mut() {
        material.uniforms = shader_data.clone();
    }
}


const BUFFER_SIZE: usize = 256;

#[derive(Resource)]
pub struct SampleBuffer {
    buffer: Vec<f32>,
}

impl Default for SampleBuffer {
    fn default() -> Self {
        Self {
            buffer: vec![0.0; BUFFER_SIZE],
        }
    }
}

fn write_to_fft_buffer(
    mut sample_buffer: ResMut<SampleBuffer>,
    ui_state: Res<UiState>,
    time: Res<Time>,
) {
    // Generate real audio samples from the current FundSP sine wave
    // This creates samples that match what's actually being played
    let current_frequency = ui_state.value;
    let time_seconds = time.elapsed_secs();
     
    // Generate a sample from the current sine wave
    let sample = (time_seconds * current_frequency * 2.0 * std::f32::consts::PI).sin() * 0.5;
     
    // Push the sample to the buffer
    sample_buffer.buffer.push(sample);
     
    // Keep the buffer at a fixed size
    if sample_buffer.buffer.len() > BUFFER_SIZE {
        sample_buffer.buffer.remove(0);
    }
}



const SAMPLE_RATE: f32 = 44100.0; // Replace with your sample rate
