# Sea of Stars TAS

This Tool Assisted Speedrun uses memory reading and branching logic to automate speedrunning Sea of Stars.
We are not necessarily aiming to be faster than the best human players but are trying to dynamically handle any situation the game has to offer.  
This project has been inspired by the [FF4 TAS](https://github.com/aexoden/edge), [FFX TAS](https://github.com/coderwilson/FFX_TAS_Python) and [Evoland TAS](https://github.com/orkaboy/Evoland_TAS) projects that work in a similar fashion.  

This Repo is a recreation of the [original Sea of Stars TAS](https://github.com/shenef/SoS-TAS) in rust

Please join [our Discord](https://discord.gg/ebmfGDP) if you are interested in the project.

## Setup

- Install Rust Nighly
- clone the repo 
- copy `config.toml.example` to `config.toml`
- run `cargo run` or `cargo run --release`

## Contribute

If you want to contribute code, please join our discord!

## Persistence Notes

If you are missing tabs, you may need to delete the `egui` persistence file located for your operating system as described [HERE](https://docs.rs/eframe/latest/eframe/fn.storage_dir.html)
