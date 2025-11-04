# Solarsim

Solarsim is a client-server simple simulation of a solar thermal system.

The simulation contemplates the following variables:

- Ambient temperature and solar irradiance.
- Incoming water load (water entering the tank) temperature and mass flow rate.
- Solar panel area, efficiency, losses and water intake temperature.
- Pipe surface and heat transfer.
- Pump flow rate.
- Tank temperature, heat loss, dimensions and water mass.

## Requirements

- OS: macOS, Windows (including WSL), or Linux.
- [Rust](https://rust-lang.org/learn/get-started/)
- [Node.js](https://nodejs.org/en/download) 20.9 or later.
- [`just`](https://github.com/casey/just?tab=readme-ov-file#installation)

## Quick Start

Run these commands on two separate terminals

```sh
just dev server
```

```sh
just dev client
```

Now you can alter the simulation parameters either on the server window or through the website (http://localhost:3000 by default).

To see a list of all the available recipes in the project `just` run

```sh
just
```
