<div align="center">

# Krona-Lite

A minimalist, background-oriented to-do application for focus.

<h3>

[Download Latest Release](https://github.com/zendrixate/krona-lite/releases)

</h3>

</div>

---

Krona-Lite is a tiny utility designed for an at-a-glance overview of your tasks. It runs quietly in the background, acting like a super-powered sticky note that appears and disappears instantly with a single keypress.

Press the `END` key to toggle your to-do list, making it perfect for quick checks and hassle-free time tracking without the need for heavy, distracting applications.

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

- **Minimalist UI:** A clean, borderless window that runs in the background.
- **Persistent Data:** Tasks are automatically saved to a `todos.json` file in your `Documents/Krona` folder.
- **Always on Top:** Pin the window to keep it visible over all other applications.
- **Task Timers:** Track the time spent on each individual task.
- **Dynamic Sizing:** The window automatically adjusts its height to fit your tasks.
- **Drag to Move:** Use the subtle handle at the top of the window to move it with your mouse.
- **Drag to Reorder:** Reorganize your active and completed tasks by dragging them.

### Dynamic Interaction Demo
<video src="https://github.com/user-attachments/assets/896bd87a-b500-4264-aaa7-d9210a59ac3f" autoplay loop muted playsinline title="Drag and drop, focus, and resizing demo"></video>


## Installation

The latest release is available on the [GitHub Releases](https://github.com/zendrixate/krona-lite/releases) page.

1.  Download `Krona-Lite_0.1.0_x64-setup.exe`.
2.  Run the installer. The app will start automatically in the background.

## Usage

-   **Show/Hide Window:** Press the `END` key.
-   **Move Window:** Click and drag the handle at the top of the window.
-   **Pin Window:** Click the pin icon in the header.
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
