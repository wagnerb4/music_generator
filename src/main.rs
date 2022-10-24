use anyhow::Result;
use clap::{ArgEnum, ArgGroup, Parser};

use std::collections::HashMap;
use std::rc::Rc;

use fundsp::hacker::*;

use music_generator::musical_notation;
use music_generator::musical_notation::Temperament;

use music_generator::voice::action::{Action, AtomType, NeutralActionState, SimpleAction};
use music_generator::voice::Voice;

use music_generator::l_system::{Atom, Axiom};

#[derive(Clone, ArgEnum)]
enum PitchStandard {
    Baroque,
    Chorton,
    Classical,
    Stuttgart,
}

#[derive(Clone, ArgEnum)]
enum ScaleKind {
    Major,
    Minor,
    Chromatic,
}

#[derive(Clone, ArgEnum)]
enum TemperamentKind {
    EqualTemperament,
    JustIntonation,
}

fn parse_tonic(s: &str) -> Result<musical_notation::Tone, String> {
    musical_notation::Tone::from(s)
}

/// play a voice
#[derive(Parser)]
#[clap(author, version, about)]
#[clap(group(ArgGroup::new("scale").args(&["scale_tonic", "scale_kind"])))]
struct Cli {
    /// the axiom of the voice
    axiom: String,
    /// the output path
    #[clap(parse(from_os_str), short = 'o', long = "output")]
    output: std::path::PathBuf,
    #[clap(arg_enum, short, long, default_value_t = PitchStandard::Stuttgart)]
    pitch_standard: PitchStandard,
    #[clap(long, default_value = "C", value_parser = parse_tonic)]
    scale_tonic: musical_notation::Tone,
    #[clap(arg_enum, long, default_value_t = ScaleKind::Major)]
    scale_kind: ScaleKind,
    #[clap(arg_enum, long, default_value_t = TemperamentKind::EqualTemperament)]
    temperament_kind: TemperamentKind,
}

fn sequence_helper(voice: Voice, dest_path: std::path::PathBuf) -> Result<()> {
    let sample_rate = 44100.0;
    let mut sequencer = Sequencer::new(sample_rate, 2);

    let env = || envelope(|t| cos(t));
    let magic = |pitch: f64| 200.0_f64 * sine_hz(pitch) * env();
    let magic = |pitch: musical_notation::Pitch,
                 volume: musical_notation::Volume|
     -> Box<dyn AudioUnit64> {
        Box::new(volume.get() as f64 * magic(pitch.get_hz()) >> pan(0.0))
    };

    let bpm = 120;
    voice.sequence(&mut sequencer, bpm, magic);

    let duration = voice.get_duration(bpm);

    let wave = Wave64::render(sample_rate, duration, &mut sequencer);
    // let wave = wave.filter(duration, &mut (reverb_stereo(0.1, 2.0) * 3.0));
    let wave = wave.filter_latency(duration, &mut (limiter_stereo((0.01, 0.1))));
    wave.save_wav16(&dest_path)?;

    Ok(())
    /*
    let sample_rate = 44100.0;
    let env = || envelope(|t| cos(t));
    // let test = |pitch: f64| 200.0_f64 * sine_hz(pitch) * env();
    // let test = |pitch: f64| test(pitch) >> pan(0.0);
    let test = |pitch: f64| brown() * pitch >> sine() >> pan(0.0);
    let mut sequencer = Sequencer::new(sample_rate, 2);
    let duration = length as f64 / bpm_hz(bpm) + 2.0;
    let wave = Wave64::render(sample_rate, duration, &mut sequencer);
    let wave = wave.filter(duration, &mut (reverb_stereo(0.1, 2.0) * 3.0));
    let wave = wave.filter_latency(duration, &mut (limiter_stereo((0.01, 0.1))));
    wave.save_wav16(std::path::Path::new("sequence.wav")).unwrap()
    */
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let axiom = Axiom::from(&args.axiom)?;

    let pitch_standard: f64 = match args.pitch_standard {
        PitchStandard::Baroque => musical_notation::BAROQUE_PITCH,
        PitchStandard::Chorton => musical_notation::CHORTON_PITCH,
        PitchStandard::Classical => musical_notation::CLASSICAL_PITCH,
        PitchStandard::Stuttgart => musical_notation::STUTTGART_PITCH,
    };

    let temp = match args.temperament_kind {
        TemperamentKind::EqualTemperament => {
            Rc::new(musical_notation::EqualTemperament::new(pitch_standard))
        }
        TemperamentKind::JustIntonation => panic!("Not implemented!"),
    };

    let key = musical_notation::Key::new(args.scale_tonic, temp);

    let mut atom_types: HashMap<&Atom, AtomType<NeutralActionState>> = HashMap::new();

    let action: Rc<dyn Action<_>> = Rc::new(SimpleAction::new(
        key,
        match args.scale_kind {
            ScaleKind::Major => &musical_notation::ScaleKind::Major,
            ScaleKind::Minor => &musical_notation::ScaleKind::Minor,
            ScaleKind::Chromatic => panic!("Not implemented!"),
        },
    ));

    for atom in axiom.atoms() {
        atom_types.insert(
            atom,
            match atom.symbol {
                _ => AtomType::HasAction {
                    action: Rc::clone(&action),
                },
            },
        );
    }

    let voice = Voice::from(&axiom, atom_types)?;

    Ok(sequence_helper(voice, args.output)?)
}
