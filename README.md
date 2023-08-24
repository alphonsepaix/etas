## ETAS

A simple implementation of the Epidemic-Type Aftershock Sequence
stochastic model

### Usage

```shell
Programme de simulation pour le modèle ETAS

Usage: etas.exe [OPTIONS]

Options:
      --mu <MU>              [default: 1]
      --alpha <ALPHA>        [default: 2]
      --bar-n <BAR_N>        [default: 0.9]
      --p <P>                [default: 1.1]
      --c <C>                [default: 0.000000001]
      --beta <BETA>          [default: 2.3025851]
      --t-end <T_END>        [default: 1000]
      --max-len <MAX_LEN>
      --filename <FILENAME>  [default: data.csv]
      --verbose
  -h, --help                 Print help
```

### Installation

```shell
git clone https://github.com/alphonsepaix/etas.git
cd etas
cargo build --release
```
