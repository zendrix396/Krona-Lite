<div align="center">

# Krona-Lite

A minimalist, background-oriented to-do application for focus.

<h3>

[Download Latest Release (v0.1.1)](https://github.com/zendrixate/krona-lite/releases)

</h3>

</div>

---

Krona-Lite is a tiny utility designed for an at-a-glance overview of your tasks. It runs quietly in the background, acting like a super-powered sticky note that appears and disappears instantly with a single keypress.

Press your custom hotkey (`F4` by default) to toggle your to-do list, making it perfect for quick checks and hassle-free time tracking without the need for heavy, distracting applications.

<table>
  <tr>
    <td align="center">
      <img src="readme_media/image1.png" alt="Krona-Lite UI" width="300"/>
      <br>
      <em>Clean interface that stays out of your way.</em>
    </td>
    <td align="center">
      <img src="readme_media/image2.png" alt="Krona-Lite with completed tasks" width="300"/>
      <br>
      <em>Completed tasks are neatly separated.</em>
    </td>
  </tr>
</table>

## Features

-   **Customizable Title:** Double-click the title (defaults to "Krona") to edit it.
-   **Total Time Display:** See the total focused time from all tasks, updated in real-time.
-   **Advanced Hotkey System:** Set any key combination (`F4` by default) to toggle the app.
-   **Always on Top:** The app stays on top by default, with no need for a pin button.
-   **Persistent Data:** Tasks are automatically saved to `Documents/Krona/todos.json`.
-   **Task Timers:** Track time on individual tasks, with only one timer running at a time.
-   **Dynamic Sizing:** The window automatically adjusts its height to fit your tasks.
-   **Drag to Move & Reorder:** Use the top handle to move the window, and drag tasks to reorder them.

### Dynamic Interaction Demo
<video src="https://github.com/user-attachments/assets/896bd87a-b500-4264-aaa7-d9210a59ac3f" autoplay loop muted playsinline title="Drag and drop, focus, and resizing demo"></video>

### Custom Title & Hotkey
<video src="https://github.com/user-attachments/assets/f9fd3cce-b755-4d12-a4e1-096b83891cc7" autoplay loop muted playsinline></video>

## Installation

The latest release is available on the [GitHub Releases](https://github.com/zendrixate/krona-lite/releases) page.

1.  Download `Krona-Lite_0.1.1_x64-setup.exe`.
2.  Run the installer. The app will start automatically in the background.

## Usage

-   **Show/Hide Window:** Press your custom hotkey (`F4` by default).
-   **Move Window:** Click and drag the handle at the top of the window.
-   **Change Hotkey:** Click the "Hotkey" button at the bottom of the app.
-   **Add Task:** Click the `+` icon.
-   **Edit Task:** Double-click the task text.
-   **Complete Task:** Click the checkbox.
-   **Track Time:** Hover over a task and click the watch icon.
-   **Delete Task:** Hover over a task and click the trash icon.

## Technical Details

Krona-Lite is built with a focus on simplicity and performance, using a modern, lightweight stack.

*   **Core Framework:** [Tauri](https://tauri.app/)
    -   Uses Rust for the backend and the system's native WebView, resulting in a tiny, fast, and resource-efficient application.
*   **Frontend:** [Svelte](https://svelte.dev/)
    -   A compiler-based framework that generates highly optimized, vanilla JavaScript for a snappy and responsive UI.
*   **State Management:** [Svelte Stores](https://svelte.dev/tutorial/stores)
    -   A simple, built-in solution for managing the application's reactive state.
*   **Data Persistence:**
    -   Task data is serialized to JSON and saved in `Documents/Krona/todos.json`.
    -   File I/O is handled on the Rust backend for performance and security.

---

## Contributing

Contributions are welcome. This project follows a simple structure:

-   `src/`: Contains all the Svelte frontend components and logic.
-   `src-tauri/`: Contains all the Rust backend logic, including the Tauri setup and native features.
-   `readme_media/`: Contains all the media assets for this README.

## License

This project is licensed under the MIT License.
