# pyramus

A web app to help TTRPG game-masters easily create props and puzzles for their players to enjoy

You can try it out the latest build [here](https://pyramus.pages.dev/).

## Setup

1. Clone the repository

2. Install the dependencies

```bash
npm install
```

3. Install Rust and wasm-pack

```bash
cargo install wasm-pack
cargo install cargo-watch
```

4. Build the frontend

```bash
cd pyramus-gui
npm start
```

## Roadmap

- [ ] Diffentiate between props and stages

  - If a prop is the king's seal, the second layer would be the letter with the seal on it
  - [ ] Stages should have props be easily swappable
  - [ ] Stages should be able to be printed, saved, or shared via a temporary link

- [ ] Create a backend server (Django)

  - [ ] Create a member login / authentication system
  - [ ] Save props and stages to a database

- [ ] Puzzles

  - [ ] Stages should be able to have one or more puzzles
  - [ ] Create an environment where you can solve it online
    - [ ] Collaborative solving
    - [ ] Race solving

- [ ] Campaigns

  - [ ] Stages and props should be able to be grouped into campaigns
    - [ ] Campaign should have stored metadata
  - [ ] Temporary link should be
  - [ ] Campaigns should be able to be shared via a temporary link

- [ ] Tie-ins

  - [ ] Props and stages should be exportable to various tie-ins
    - [ ] Puzzles should be solvable in the tie-ins
  - [ ] OwlBear Rodeo
  - [ ] Roll20
  - [ ] Foundry

- [ ] Search / view / explore
  - [ ] Elasticsearch?
