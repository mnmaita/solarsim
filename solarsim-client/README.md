# Solarsim Client

The Solarsim Client is a Next.js static website that displays the simulation parameters and outputs. Parameters can also be altered from the client, and the server will be updated through BRP calls.

### Getting Started

To run the development build:

```sh
just dev
```

To build and serve the production build:

```sh
just run
```

Then open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can change the server host and port by updating the environment variables in the `solarsim-client/.env.local` file.
