# Fast Fourier Transforms in Rust

### Parts of the project
- A mechanism for plotting a wave on a chart. Should support:
  - Read a collection of values for the wave, each associated with a timestamp, from a file (.WAV?)
  - Receiving a live input of wave values to be plotted live.
  - [Optional] Plotting multiple waves simultaneously in different colors
- A mechanism for applying a Fast Fourier Transform on a given wave.