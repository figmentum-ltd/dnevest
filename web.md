# Rust CSR Web Frameworks

## 1. Yew
### Strengths
- Highly performant: Leverages Rust's safety and performance benefits, making it efficient for handling complex frontend logic.
- Component-based architecture: Inspired by React, making it familiar to frontend developers.
- Integration with WebAssembly (Wasm): Seamless integration with Wasm for highly performant web applications.
- Active community and good documentation: A large and active community that continuously develops the framework.

### Weaknesses
- Limited ecosystem compared to JS frameworks: Although growing, the ecosystem of third-party libraries and tools is not as mature as JavaScript-based frameworks.

### Use Cases
- Single-page applications (SPAs): Yew is designed for building SPAs with highly interactive user interfaces.
- High-performance applications: Use cases where performance is critical, such as games or data-heavy web applications.
- Developer tools and dashboards: Great for building dashboards or other internal tools that benefit from Rust’s safety and performance.

### Rendering
Yew compiles the Rust code into WebAssembly. This Wasm code is then executed within the browser, allowing Rust code to interact with web APIs and manipulate the DOM.
Yew creates a new virtual DOM tree and compares it with the previous version. This process is called "diffing."

  - Diffing the virtual DOM: Yew identifies the differences between the new virtual DOM and the previous one, finding only the parts that need to be updated.
  - Updating the real DOM: Once the differences are identified, Yew updates the actual DOM with minimal operations, only making the necessary changes.

### Setup
1. Install the Rust Toolchain Installer

    * follow the instructions on [https://rustup.rs/](https://rustup.rs/), or
    * **[preferred]** install through your system's package manager, e.g. on ArchLinux use `sudo pacman -S rustup`

2. Add the `wasm32` target

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

3. Install the `trunk` 

   ```sh
   # note that this might take a while to install because it compiles everything from scratch
   # Trunk also provides prebuilt binaries for a number of major package managers
   # See https://trunkrs.dev/#install for further details
   cargo install trunk
   ```

4. Create a new project 

5. Update Cargo.toml
Add Yew to the list of the dependencies.

   ```toml
   [package]
   name = "yew-app"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   # this is the development version of Yew
   yew = { git = "https://github.com/yewstack/yew/", features = ["csr"] }
   ```

6. Build and serve the application locally

   ```sh
   trunk serve
   ```
---

## 2. Sycamore
### Strengths
- Fine-grained reactivity: Uses fine-grained reactive primitives, providing more efficient updates by minimizing DOM changes.
- Lightweight: Small and fast, reducing overhead compared to some other frameworks.
- Flexible architecture: Allows building with components, but also supports functional and reactive programming styles.
- No JavaScript required.
- Wasm-based: Like Yew, Sycamore benefits from Rust's Wasm integration, offering great performance.

### Weaknesses
- Small community: Not as widely adopted as Yew, so there may be fewer resources and third-party libraries.
- Documentation gaps: Some users report that documentation is not as comprehensive as other Rust frameworks.

### Use Cases
- Interactive web apps: Ideal for building highly interactive, performant web apps that need fine-grained reactivity.
- Smaller projects: Good for small to medium-sized projects that need a lightweight framework.
- Data-driven applications: Great for applications where efficient data reactivity and UI updates are crucial, such as dynamic dashboards.

### Rendering
Instead of relying on a Virtual DOM (VDOM), Sycamore uses fine-grained reactivity to keep the DOM and state in sync. 
Sycamore’s reactivity system can be used on its own without pulling in all the DOM rendering part. It just turns out that fine-grained reactivity and UI rendering are a great match which is the whole point of Sycamore.

### Setup
1. Install the Rust Toolchain Installer

    * follow the instructions on [https://rustup.rs/](https://rustup.rs/), or
    * **[preferred]** install through your system's package manager, e.g. on ArchLinux use `sudo pacman -S rustup`

2. Add the `wasm32` target

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

3. Install the `trunk` 

   ```sh
   # Install via cargo.
   cargo install --locked trunk
   ```
4. Create a new project 

5. Update Cargo.toml
   
   Add Sycamore to the list of the dependencies.

   ```toml
   sycamore = "0.9.0-beta.4"
   ```

6. Build and serve the application locally

   ```sh
   trunk serve
   ```
---

## 3. Dioxus
### Strengths
- Multiplatform: Dioxus allows you to target web, desktop, and mobile with the same codebase.
- React-like API: Inspired by React, it’s easier for developers with React experience to transition to Dioxus.
- Well-documented: Comprehensive documentation for getting started and building applications.
- Good performance: Takes advantage of Rust and Wasm for performant web applications.

### Weaknesses
- Young framework: Still in its early stages, so the ecosystem and community are not as large as more established Rust frameworks.
- Limited in-browser debugging: Since it is a Wasm-based framework, the browser debugging experience is not as seamless as with JavaScript-based tools.

### Use Cases
- Cross-platform apps: Great for projects where the goal is to build apps for both web and desktop or mobile platforms.
- React developers switching to Rust: The React-like API makes it easier for those familiar with React to build in Rust.
- UI-heavy applications: Ideal for applications that require rich user interfaces across different platforms.

### Rendering
Dioxus renders client-side data by compiling Rust code into WebAssembly (Wasm). It uses a virtual DOM to manage and update the UI. When the state of a component changes, Dioxus compares the current virtual DOM with the previous one
 and updates only the necessary parts of the real DOM. This ensures efficient rendering and minimal DOM manipulation.

### Setup
1. Install the Rust Toolchain Installer

    * follow the instructions on [https://rustup.rs/](https://rustup.rs/), or
    * **[preferred]** install through your system's package manager, e.g. on ArchLinux use `sudo pacman -S rustup`

2. Check platform-specific dependencies

     Most platforms don't require any additional dependencies, but if you are targeting desktop, you can install the following dependencies (for Linux):

     Webview Linux apps require WebkitGtk. When distributing, this can be part of your dependency tree in your .rpm or .deb. However, likely, your users will already have WebkitGtk.

     ```sh
     sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libappindicator3-dev
     ```

     If you run into issues, make sure you have all the basics installed, as outlined in the Tauri docs.

3. Add Dioxus and the desktop renderer as dependencies (this will edit your Cargo.toml): 

   ```sh
   cargo install dioxus-cli
   ```

4. Create a new project 

5. Update Cargo.toml
    Add Sycamore to the list of the dependencies.

   ```toml
   sycamore = "0.9.0-beta.4"
   ```

6. Build and serve the application locally

   ```sh
   trunk serve
   ```

---

## 4. Leptos
### Strengths
- Reactive Programming: Leptos follows a reactive programming model, making it highly efficient for updating UIs based on data changes.
- Optimized for performance: Designed to work efficiently with Wasm, providing high-performance web apps.
- Flexible: Supports a range of use cases, from simple to complex applications, with a fine-tuned balance of flexibility and performance.

### Weaknesses
- Small but growing community.
- Less beginner-friendly: Might be more challenging for newcomers due to its focus on reactive programming.

### Use Cases
- Data-driven applications: Particularly well-suited for applications that rely heavily on real-time data or frequent updates.
- Performance-sensitive applications: Where performance is crucial, such as games or interactive simulations.
- Reactive UI development: Ideal for building applications that benefit from a reactive programming approach.

### Rendering
Leptos is unique in that it is built around a reactive programming model. Instead of using a virtual DOM (like Yew or React), Leptos focuses on fine-grained reactivity. This means that when a piece of reactive state changes, only the specific part of the DOM that depends on that state is updated, rather than re-rendering an entire component or performing a virtual DOM diff.

    * Signal-based state management: Leptos uses signals to track state changes. A signal is a reactive value that, when updated, triggers a re-render of only the parts of the UI that depend on that value.

### Setup
1. Install the Rust Toolchain Installer

    * follow the instructions on [https://rustup.rs/](https://rustup.rs/), or
    * **[preferred]** install through your system's package manager, e.g. on ArchLinux use `sudo pacman -S rustup`

2. Add the `wasm32` target

   ```sh
   rustup target add wasm32-unknown-unknown
   ```

3. Install the `trunk` 

   ```sh
   cargo install trunk
   ```
4. Create a new project 

5. cd into your new leptos-tutorial project and add leptos as a dependency
   
   ```sh
   cargo add leptos --features=csr
   ```

6. Build and serve the application locally

   ```sh
   trunk serve --open
   ```
