# Just Another Rust Discord Bot
This bot serves to be a project for me to learn Rust. For no other purpose than torturing a python developer that hasnt looked at C++ in years.

## Features
### Commands
#### Admin
- [ ] Record - Records audio
- [x] Register - Adds slash commands
- [ ] Stenographer - Speach to Text
#### Fun
- [ ] AddReact - For some regex string, react
- [ ] Coin
- [ ] Poll
- [ ] Roll
- [ ] XKCD
#### Info
- [ ] About 
- [x] Age -> User
- [ ] Author
- [ ] Guild
- [ ] User
#### Moderation
- [ ] Ban
- [ ] Clear
- [ ] Delete - Delete `N` messages
- [ ] Hardkick - Kick and delete all messages by the user.
- [ ] Kick
- [ ] Mute
- [ ] Deafen
- [ ] Purge
- [ ] Tempban
- [ ] Tempmute
- [ ] Timeout
- [ ] Slowmode
- [ ] Undeafen
- [ ] Unmute
- [ ] Warn
#### Regional
- [ ] weather
#### Space
- [ ] Activestorm - Reports information about active named storms
- [ ] Conus - Grabs conus animation for server region. Defaults to GeoColor, 120
- [ ] ISS - Reports information about the ISS

## Getting Started
TODO

### Dependencies
- clap 4.0.19
- poise 0.4.1
- tokio 1.21.2
- toml 0.5.9
- serde 1.0

## Resources
- [Clap docs](https://docs.rs/clap/4.0.19/clap/)
- [Poise docs](https://docs.rs/poise/0.4.1/poise/index.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)
- [Rust docs](https://doc.rust-lang.org/book/)
- [Rust Modules Explained](https://www.sheshbabu.com/posts/rust-module-system/)
- [Serenity docs](https://docs.rs/serenity/0.11.5/serenity/index.html)
- [YAML docs](https://yaml.org/spec/1.2.2/)

## References
Just some miscellaneous bots written in Rust for when I need a little help.

### Scripty
[scripty](https://github.com/tazz4843/scripty) is a discord bot written in Rust that transcribes audio into text. However this bot has been discontinued. This bot has been revived here this [repo](https://github.com/scripty-bot/scripty) and [scripty.org](https://scripty.org). Speech to Text is a feature I would like to add so that it could possibly act as secretary for [Auburn Esports](https://aub.ie/esports).

### Taliyah
[Taliyah](https://github.com/evelynmarie/Taliyah) is kitchen sink bot written in Rust. It does a lot of things and is good refenrece for some of the commands I would like to implement.

## License
[MIT License](license.md)