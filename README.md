# silly-kernel

[README in English](README.en.md)

Resulta que estoy de vacaciones, estrenando mi nueva MacBook Air M4 y quería experimentar con la arquitectura ARM. En el momento de curiosidad y desquicio que tanto me caracteriza, se me ocurrió la idea de encarar este nuevo proyecto.

Para esta ocasión, no tengo tantísima cancha con el desarrollo bare-metal, pero nada que un poco de leer una cantidad obscena de documentación y tratar de entender los flujos de ARM64 no pudieran solucionar. Después de todo, me siento cómodo con proyectos de muy bajo nivel, así que esto de cualquier forma iba a estar muy divertido.

El plan para empezar con este kernel es en principio bastante sencillo: implementar el Linux boot protocol como para poder usar bootloaders existentes, luego interpretar device trees, tener drivers extremadamente simples para los dispositivos detectados como pantallas o monitores seriales (cosa de poder escribir un hola mundo o en todo caso hacer un dibujito), y después, solamente si no me aburro antes, pasar a cosas más avanzadas como gestionar la memoria virtual, tener la abstracción de procesos, tratar de ejecutar programas sencillos, filesystems (aunque sea FAT32) y demás.

La gracia de este código es que, como muchos otros proyectos que nacen de la curiosidad, lo estoy haciendo sin saber (del todo) cómo funciona un kernel. Jamás vi el código de uno desde cero. El plan es hacerlo según mi propia interpretación de cómo creo que sería uno.

Como todo en esta vida, esto lo hago solamente por amor al arte, para aprender y divertirme. Este proyecto seguirá vivo en lo que siga encontrando ese disfrute.

## Requisitos

Hace falta Rust stable con target `aarch64-unknown-none`, e idealmente una toolchain GCC para el target `aarch64-none-elf-gcc`. Para arrancar el kernel, se recomienda utilizar QEMU, específicamente `qemu-system-aarch64` (sobre todo si los únicos drivers que voy a escribir van a ser para este mismo). Todo esto ya está configurado en [la devShell de Nix](flake.nix) como para tener un ambiente listo para usar.

Al ser un flake, en caso de querer usar Nix, se puede levantar una devShell de la siguiente forma:

```bash
nix develop
```

## Compilación

Se pueden usar los xtasks para compilar el kernel:

```bash
# Compilar el kernel (debug)
cargo xtask build-kernel

# Compilar el kernel en modo release
cargo xtask build-kernel --release
```

## Ejecución

El kernel se puede arrancar también con sus xtasks:

```bash
# Compilar y ejecutar con símbolos de debug
cargo xtask qemu

# Compilar y ejecutar en modo release
cargo xtask qemu --release

# Compilar y ejecutar con debugging GDB en :1234
cargo xtask debug-qemu
```

También se puede compilar solo la imagen binaria:

```bash
# Compilar imagen binaria (debug)
cargo xtask build-image

# Compilar imagen binaria (release)
cargo xtask build-image --release
```

## Links Útiles

* https://krinkinmu.github.io/2020/12/26/position-independent-executable.html