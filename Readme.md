# Build

`cargo build --release`

# Example

To generate a simple melody in G-Flat-Minor run the following.

`target/release/music_generator --scale-tonic Gb --scale-kind minor --output gen.wav AxBCxDExFGHxxGFxEDxCBA`

# Roadmap

- [x] Remove assumption about 12-tone-system from the implementation of the Key struct and move it to the implementation of EqualTemperament.
- Implement an easy way to generate a type of JustIntonation for an arbitrary instance of the Key struct.
- Modify the Temperament trait for easy use in the Key implementation such that no additional mapping is necessary.
