# DSA Visualization

Project for Rust Course on FIIT STU.

## Authors

- Martin Sivák (Discord: Martiiin#2238, Rust-101 Discord: Martiiin)
- Lukas Castven (Discord: Nesquiko#1631, Rust-101 Discord: Lukas)

## Project proposal

### Idea

GUI for visualizing different data structures and algorithms. We hope to learn
how to implement different data structures and algorithms in Rust, and also
how to build GUI applications in Rust, either in web browser through WASM,
or native ones.

### Requirements

- Implementation of data structures:
  - Linked List
  - Hash Table
  - Graph
- Implementation of algorithms:
  - Depth-First-Search
  - Dijkstra's algorithm
- Visualize data structures and algorithms with a GUI

### Possibly used Crates

For the DSA part of our project, no crates are needed, however for the GUI part
there are many crates that at first glance look like what we need.

For now [egui](https://github.com/emilk/egui) was choosen, because there were
couple public examples we think will help us with our project. Also it will be
built into a WASM binary, because we can easily deploy this binary on Githup pages
for free and anyone can open the project and try it out.

#### Candidates for the GUI:

We would like to hear what our tutors think about these candidates, and gave us
feedback, which would help us choose which crate to use.

- [Leptos](https://github.com/leptos-rs/leptos)
- [egui](https://github.com/emilk/egui)
- [Dioxus](https://github.com/DioxusLabs/dioxus/)
- [Yew](https://github.com/yewstack/yew)
- [Iced](https://github.com/iced-rs/iced)
