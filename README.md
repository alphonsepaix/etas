## ETAS

Implémentation Rust de l'algorithme ETAS (Epidemic Type Aftershock Sequence)

### Utilisation

On peut spécifier les paramètres de la simulation en ligne de commande :

```shell
Usage: etas [OPTIONS]

Options:
      --mu <MU>              [default: 1]
      --alpha <ALPHA>        [default: 2]
      --bar-n <BAR_N>        [default: 0.9]
      --p <P>                [default: 1.1]
      --c <C>                [default: 0.000000001]
      --beta <BETA>          [default: 2.3025851]
      --t-end <T_END>        [default: 1000]
      --filename <FILENAME>  [default: data.csv]
      --verbose              
  -h, --help                 Print help
```

### Installation

L'outil de build Cargo est recommandé pour compiler le programme.

```shell
git clone https://github.com/alphonsepaix/etas.git
cd etas
cargo build --release
```