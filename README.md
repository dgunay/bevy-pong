## Bevy Pong clone

Making a Pong clone is a common rite of passage for budding game developers,
so I went with just that.

This is a simple Pong clone made using Bevy. It features:
* An absolute barebones main menu
* Screen shake based on relative collision velocity
* Local multiplayer

You can [try it out here](https://dgunay.github.io/bevy-pong/). You may have
to scroll down to see the game - seems like it positions the viewport on the 
bottom left of the page for some reason.

Stuff I want to maybe try later:
* Gamepad support
* Online p2p multiplayer with rollback
* Additional game mechanics (temporary obstacles, powerups, etc)
* More advanced collision (non-rectangular paddles, rotating paddles, etc)
* Better visuals (particles, sprites, animated background, etc)

### Useful plugins

The modules under `plugins` have potential uses in other games; just follow 
their documentation to make use of them.

### Other tidbits

The music is a rag that I composed many years ago while I was still taking
music semi-seriously.