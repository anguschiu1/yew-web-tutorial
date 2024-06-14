# Demo Yew Web Tutorial implemented by two styles

This project is a web application built using the [Yew](https://yew.rs/) framework for Rust. The application fetches and displays a list of videos from a JSON endpoint and allows users to select and view details about each video.

Inspired by [Chris Biscardi](https://github.com/rust-adventure/yt-yew-tutorial.git) and his [video](https://youtu.be/S-O9QkrlfYw?si=EL44lSwp6gjdk00c), this tutorial demo is developed using function components (in `main` branch) and struct components (in branch `refactor`). Feel free to checkout both branches and take a look.

## Project Structure

The project has the following structure:

```
Cargo.lock
Cargo.toml
Trunk.toml
index.html
src
    main.rs
    video.rs
```

- `Cargo.toml`: Contains the project metadata and dependencies.
- `Trunk.toml`: Configuration file for Trunk, a WASM web application bundler for Rust.
- `index.html`: The main HTML file for the web application.
- `src/main.rs`: The entry point of the application.
- `src/video.rs`: Contains the `Video` struct and related components.

## Dependencies

The project uses the following dependencies:

- `gloo-net`: For making HTTP requests.
- `serde`: For serializing and deserializing JSON data.
- `wasm-bindgen-futures`: For working with JavaScript Promises in Rust.
- `yew`: The web framework for building client-side web applications in Rust.

## Components

`App` component is the main component of the application, managing the state of the video list and the selected video. It also handles fetching the video data from the server.

The `VideoList` and `VideoDetails` component are defined in the `video.rs` file, which includes the `Video` struct and function components for displaying video details and the video list.

## Running the Project

To run the project, you need to have Rust and Trunk installed. Follow these steps:

1. Install Rust from [rust-lang.org](https://www.rust-lang.org/).
2. Install Trunk by running:
   ```sh
   cargo install trunk
   ```
3. Clone the repository and navigate to the project directory.
4. Run the project using Trunk:
   ```sh
   trunk serve
   ```

This will start a local development server, and you can view the application in your web browser at `http://localhost:8080`.

The `Trunk.toml` also specified that path with `/tutorial` is proxying to https://yew.rs/tutorial, as a workaround to the CORS requirement.

However, it is known that sometimes the endpoint will interact with browsers in a way that returns default HTML rather than the expected `/tutorial/data.json` file, and the erratic HTML file may sometimes be cached (in both ends). You may need to try with different browsers, or use file override (a function in Safari dev inspector) to obtain the correct JSON file.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
