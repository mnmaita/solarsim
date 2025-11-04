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

Run these commands on separate terminals

```sh
just dev server
```

```sh
just dev server
```

Now you can alter the simulation parameters either on the server window or through the website (http://localhost:3000 by default).

## Server

The server is a [Bevy](https://bevy.org/) app that runs the simulation with a fixed timestep and can be remotely controlled through the [Bevy Remote Protocol](https://docs.rs/bevy_remote/latest/bevy_remote/). It renders an interactive UI with controls to alter the simulation parameters.

### Running the server

To run the development build:

```sh
just dev server
```

To run the release build:

```sh
just run server
```

## Client

The client is a Next.js static website that displays the simulation parameters and outputs. Parameters can also be altered from the client, and the server will be updated through BRP calls.

To run the development build:

```sh
just dev client
```

To build and serve the production build:

```sh
just run client
```

You can change the server host and port by updating the environment variables in the `solarsim-client/.env.local` file.
