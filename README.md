# Burning Wood Versus

You're a fire in an online arena with everyone else who's playing. Wood and coal fly in from the edges to make you bigger, water shrinks you, and chests drop powerups you can use on each other. Rounds last five minutes, and whoever's biggest wears the crown.

- Play: No public build
- Made: June to September 2025
- Team: [@mfclinton](https://github.com/mfclinton) (programming), [Margoon](https://jasongertner.com/) (producer), [@MrAozora](https://github.com/MrAozora) (art and audio)
- Engine: Turbo, Rust

This is an export of a private repo with only the code we wrote. Art, audio, fonts, and the web build aren't included. The history is squashed into one commit.

## What I built

- The server is authoritative and runs the whole simulation at 20 ticks per second. Clients only send inputs, and the server clamps movement and throws out anything that arrives out of order.
- I wrote client prediction with server reconciliation for your own fire. Inputs apply locally the moment you press a key and wait in a buffer with sequence numbers. When the server acknowledges one, the client snaps your fire back to the server's state and replays everything after it. Movement is scaled by frame time to match the server's tick.
- Other players get smoothed out between updates. The client keeps the last state the server sent, a predicted state built on top of it, and the state it actually draws, which eases toward the prediction every frame.
- The networking sits on a small entity framework I wrote. Players, projectiles, effects, and events all implement one trait whose setters record which fields changed. The server only sends those deltas, and the client uses the same flags to sync its three states.
- Each entity type has its own send rate. Players go out every tick, effects every 100 ms, and projectiles every 150 ms, and the server packs everything into batches of about 1,200 bytes.
- Late joiners get a snapshot of the round when they connect. If a client ever gets an update for an entity it never saw spawn, it asks the server for just that one.
- Everyone online shares one big arena in real time. You get a random name like Moist Stick that you can reroll, and it's saved for next time.
- I made 11 powerups out of 7 reusable effect types, with each item tuning the numbers. One magnet field effect covers both the blackhole that pulls in wood and coal and the wind that blows water and players away. The rest include a water gun, a metal water gun that aims itself, a water bomb, a bat that drains nearby players, and lightning that wipes out water.
