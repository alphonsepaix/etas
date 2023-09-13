## ETAS

A simple implementation of the Epidemic-Type Aftershock Sequence
stochastic model

### Usage

```shell
A simple Epidemic-Type Aftershock Sequence model implementation

Usage: etas.exe [OPTIONS]

Options:
      --mu <MU>              [default: 1]
      --alpha <ALPHA>        [default: 2]
      --bar-n <BAR_N>        [default: 0.9]
      --p <P>                [default: 1.1]
      --c <C>                [default: 0.000000001]
      --beta <BETA>          [default: 2.3025851]
      --t-end <T_END>        The end of the interval [default: 1000]
      --max-len <MAX_LEN>    The maximum number of elements in the generated sequence
      --filename <FILENAME>  The output filename [default: data.csv]
      --verbose              Display a progress bar during simulation
      --seed <SEED>          Create the PRNG using the given seed
  -h, --help                 Print help
  -V, --version              Print versionl
```

### Installation

```shell
git clone https://github.com/alphonsepaix/etas.git
cd etas
cargo build --release
```