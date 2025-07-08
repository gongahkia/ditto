[![](https://img.shields.io/badge/ditto_1.0.0-passing-green)](https://github.com/gongahkia/ditto/releases/tag/1.0.0) 

# `Ditto`

A simple [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) Emulator.

Made to practise [this stack](#stack).

## Stack

* *Script*: [Rust](https://www.rust-lang.org/), [Bash](https://www.gnu.org/s/bash/)
* *Package*: [Docker](https://www.docker.com/)

## Usage

The below instructions are for running `Ditto` on your client machine.

1. Execute the below.

```console
$ git clone https://github.com/gongahkia/ditto && cd ditto
```

2. Build the [Docker](./Dockerfile) Image.

```console
$ docker build -t chip8-ditto .
```

3. Run the `Ditto` [CHIP-8 Emulator](./src/chip8/).

```console
$ docker run --rm -it chip8-ditto
```

4. Run `Ditto` [Tests](./tests/).

```console
$ docker run --rm -it chip8-ditto test
```

## Architecture

```mermaid
```

## Reference

The name `Ditto` is in reference to [Ditto](https://bulbapedia.bulbagarden.net/wiki/Ditto_(Pok%C3%A9mon)) (also known as [Metamon](https://en.wikipedia.org/wiki/Ditto_(Pok%C3%A9mon)) メタモン), a [Normal](https://bulbapedia.bulbagarden.net/wiki/Normal_(type))-type [Generation I](https://bulbapedia.bulbagarden.net/wiki/Generation_I) [Pokémon](https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_(species)) able to [breed](https://bulbapedia.bulbagarden.net/wiki/Pok%C3%A9mon_breeding) with most other Pokémon.

<div align="center">
    <img src="./assets/logo/ditto.png" width="35%">
</div>

## Research

* [**]()
* [**]()
* [**]()
* [**]()
* [**]()
* [**]()
* [**]()