<script lang="ts">
  import { todos, appStore, type Todo } from "$lib/stores.ts";
  import TodoItem from "$lib/TodoItem.svelte";
  import { flip } from "svelte/animate";
  import { dndzone } from "svelte-dnd-action";
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let showNewTodoInput = false;
  let newTodoText = "";
  let newTodoInputElement: HTMLInputElement;
  let totalFocusedTime = 0;
  let timeInterval: number;
  let isEditingTitle = false;
  let titleInputElement: HTMLInputElement;
  let showHotkeyModal = false;
  let hotkeyCapture = {
    keys: new Set(),
    modifiers: new Set(),
    isCapturing: false,
    finalCombination: ""
  };

  $: activeTodos = $todos.filter((t) => !t.completed);
  $: completedTodos = $todos.filter((t) => t.completed);
  $: appData = $appStore;

  // Calculate total focused time from all tasks
  $: {
    totalFocusedTime = $todos.reduce((total, todo) => {
      let taskTime = todo.elapsedTime;
      if (todo.timerActive && todo.timerStart) {
        const currentTime = Date.now();
        const sessionTime = currentTime - todo.timerStart;
        taskTime += sessionTime;
      }
      return total + taskTime;
    }, 0);
  }

  // Dynamic height calculation
  $: {
    updateWindowSize();
  }

  async function updateWindowSize() {
    try {
      const window = getCurrentWindow();
      const headerHeight = 50;
      const todoHeight = 50;
      const separatorHeight = 20;
      const inputHeight = showNewTodoInput ? 50 : 0;
      const hotkeyButtonHeight = 40;
      const padding = 20;

      let totalHeight = headerHeight + padding;
      totalHeight += activeTodos.length * todoHeight;
      totalHeight += inputHeight;
      totalHeight += hotkeyButtonHeight;
      
      if (completedTodos.length > 0) {
        totalHeight += separatorHeight;
        totalHeight += completedTodos.length * todoHeight;
      }

      // Constrain height between min and max
      const minHeight = 160;
      const maxHeight = 600;
      const finalHeight = Math.max(minHeight, Math.min(maxHeight, totalHeight));

      await window.setSize({ width: 350, height: finalHeight });
    } catch (error) {
      console.error("Failed to resize window:", error);
    }
  }

  function formatTime(milliseconds: number) {
    const totalSeconds = Math.floor(milliseconds / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }

  function handleTitleDoubleClick() {
    isEditingTitle = true;
    setTimeout(() => {
      titleInputElement?.focus();
      titleInputElement?.select();
    }, 0);
  }

  function handleTitleBlur() {
    if (titleInputElement.value.trim()) {
      appStore.setTitle(titleInputElement.value.trim());
    }
    isEditingTitle = false;
  }

  function handleTitleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      titleInputElement.blur();
    } else if (event.key === "Escape") {
      isEditingTitle = false;
    }
  }

  async function handleKeyDown(event: KeyboardEvent) {
    if (event.ctrlKey && event.shiftKey && ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(event.key)) {
      event.preventDefault();
      const window = getCurrentWindow();
      const moveDistance = 10; // pixels to move
      
      try {
        const position = await window.outerPosition();
        let newX = position.x;
        let newY = position.y;
        
        switch (event.key) {
          case 'ArrowUp':
            newY -= moveDistance;
            break;
          case 'ArrowDown':
            newY += moveDistance;
            break;
          case 'ArrowLeft':
            newX -= moveDistance;
            break;
          case 'ArrowRight':
            newX += moveDistance;
            break;
        }
        
        await window.setPosition({ x: newX, y: newY });
      } catch (error) {
        console.error("Failed to move window:", error);
      }
    }
  }

  function handleActiveDndConsider(e: CustomEvent) {
    const { items } = e.detail;
    const reorderedTodos = [...completedTodos, ...items];
    todos.reorder(reorderedTodos);
  }

  function handleActiveDndFinalize(e: CustomEvent) {
    const { items } = e.detail;
    const reorderedTodos = [...completedTodos, ...items];
    todos.reorder(reorderedTodos);
  }

  function handleCompletedDndConsider(e: CustomEvent) {
    const { items } = e.detail;
    const reorderedTodos = [...activeTodos, ...items];
    todos.reorder(reorderedTodos);
  }

  function handleCompletedDndFinalize(e: CustomEvent) {
    const { items } = e.detail;
    const reorderedTodos = [...activeTodos, ...items];
    todos.reorder(reorderedTodos);
  }

  function showInput() {
    showNewTodoInput = true;
    setTimeout(() => {
      newTodoInputElement?.focus();
    }, 0);
  }

  function handleInputBlur() {
    if (newTodoText.trim()) {
      todos.add(newTodoText.trim());
    }
    newTodoText = "";
    showNewTodoInput = false;
  }

  function handleInputKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      newTodoInputElement.blur();
    } else if (event.key === "Escape") {
      newTodoText = "";
      showNewTodoInput = false;
    }
  }

  function openHotkeyModal() {
    showHotkeyModal = true;
    hotkeyCapture = {
      keys: new Set(),
      modifiers: new Set(),
      isCapturing: false,
      finalCombination: ""
    };
  }

  function handleHotkeyKeyDown(event: KeyboardEvent) {
    if (!showHotkeyModal) return;
    
    event.preventDefault();
    event.stopPropagation();
    
    hotkeyCapture.isCapturing = true;
    
    // Track modifiers
    if (event.ctrlKey) hotkeyCapture.modifiers.add('Ctrl');
    if (event.shiftKey) hotkeyCapture.modifiers.add('Shift');
    if (event.altKey) hotkeyCapture.modifiers.add('Alt');
    if (event.metaKey) hotkeyCapture.modifiers.add('Meta');
    
    // Track main keys (excluding modifiers themselves)
    if (!['Control', 'Shift', 'Alt', 'Meta'].includes(event.key)) {
      hotkeyCapture.keys.add(event.code);
    }
    
    // Update display
    updateHotkeyDisplay();
  }

  async function handleHotkeyKeyUp(event: KeyboardEvent) {
    if (!showHotkeyModal || !hotkeyCapture.isCapturing) return;
    
    event.preventDefault();
    event.stopPropagation();
    
    // Check if all modifier keys are released
    const allModifiersReleased = !event.ctrlKey && !event.shiftKey && !event.altKey && !event.metaKey;
    
    if (allModifiersReleased && hotkeyCapture.keys.size > 0) {
      // Capture is complete
      const combination = buildHotkeyString();
      if (combination) {
        await appStore.setHotkey(combination);
        showHotkeyModal = false;
      }
    }
  }

  function updateHotkeyDisplay() {
    const modifierArray = Array.from(hotkeyCapture.modifiers);
    const keyArray = Array.from(hotkeyCapture.keys).map(code => formatKeyCode(code));
    const combination = [...modifierArray, ...keyArray].join('+');
    hotkeyCapture.finalCombination = combination;
  }

  function buildHotkeyString() {
    const modifierArray = Array.from(hotkeyCapture.modifiers);
    const keyArray = Array.from(hotkeyCapture.keys);
    
    if (keyArray.length === 0) return "";
    
    // Create the combination string for backend
    return [...modifierArray, ...keyArray].join('+');
  }

  function formatKeyCode(code: string): string {
    // Convert key codes to readable format
    const keyMap: Record<string, string> = {
      'KeyA': 'A', 'KeyB': 'B', 'KeyC': 'C', 'KeyD': 'D', 'KeyE': 'E',
      'KeyF': 'F', 'KeyG': 'G', 'KeyH': 'H', 'KeyI': 'I', 'KeyJ': 'J',
      'KeyK': 'K', 'KeyL': 'L', 'KeyM': 'M', 'KeyN': 'N', 'KeyO': 'O',
      'KeyP': 'P', 'KeyQ': 'Q', 'KeyR': 'R', 'KeyS': 'S', 'KeyT': 'T',
      'KeyU': 'U', 'KeyV': 'V', 'KeyW': 'W', 'KeyX': 'X', 'KeyY': 'Y', 'KeyZ': 'Z',
      'Digit1': '1', 'Digit2': '2', 'Digit3': '3', 'Digit4': '4', 'Digit5': '5',
      'Digit6': '6', 'Digit7': '7', 'Digit8': '8', 'Digit9': '9', 'Digit0': '0',
      'Space': 'Space', 'Enter': 'Enter', 'Escape': 'Esc', 'Tab': 'Tab',
      'Backspace': 'Backspace', 'Delete': 'Delete', 'Insert': 'Insert',
      'Home': 'Home', 'End': 'End', 'PageUp': 'PgUp', 'PageDown': 'PgDn',
      'ArrowUp': '↑', 'ArrowDown': '↓', 'ArrowLeft': '←', 'ArrowRight': '→',
      'F1': 'F1', 'F2': 'F2', 'F3': 'F3', 'F4': 'F4', 'F5': 'F5', 'F6': 'F6',
      'F7': 'F7', 'F8': 'F8', 'F9': 'F9', 'F10': 'F10', 'F11': 'F11', 'F12': 'F12'
    };
    
    return keyMap[code] || code;
  }

  function formatHotkeyForDisplay(hotkey: string): string {
    if (!hotkey) return 'None';
    
    const parts = hotkey.split('+');
    return parts.map(part => {
      // If it's a key code, format it
      if (part.startsWith('Key') || part.startsWith('Digit') || ['Space', 'Enter', 'Escape', 'Tab', 'Backspace', 'Delete', 'Insert', 'Home', 'End'].includes(part)) {
        return formatKeyCode(part);
      }
      return part;
    }).join('+');
  }

  function updateTotalTime() {
    // This will trigger reactivity to update totalFocusedTime
    totalFocusedTime = $todos.reduce((total, todo) => {
      let taskTime = todo.elapsedTime;
      if (todo.timerActive && todo.timerStart) {
        const currentTime = Date.now();
        const sessionTime = currentTime - todo.timerStart;
        taskTime += sessionTime;
      }
      return total + taskTime;
    }, 0);
  }

  onMount(async () => {
    // Set always on top by default
    try {
      const window = getCurrentWindow();
      await window.setAlwaysOnTop(true);
    } catch (error) {
      console.error("Failed to set always-on-top:", error);
    }

    // Update time display every second
    timeInterval = setInterval(updateTotalTime, 1000);
    updateWindowSize();
  });

  onDestroy(() => {
    if (timeInterval) {
      clearInterval(timeInterval);
    }
  });
</script>

<svelte:window on:keydown={handleKeyDown} on:keydown={handleHotkeyKeyDown} on:keyup={handleHotkeyKeyUp} />

<div id="app">
  <header>
    <div data-tauri-drag-region class="drag-handle"></div>
    <div class="today-header">
      {#if isEditingTitle}
        <input
          bind:this={titleInputElement}
          type="text"
          value={appData.title}
          on:blur={handleTitleBlur}
          on:keydown={handleTitleKeydown}
          class="title-input"
        />
      {:else}
        <h1 on:dblclick={handleTitleDoubleClick}>{appData.title}</h1>
      {/if}
      <span class="time-remaining">{formatTime(totalFocusedTime)}</span>
    </div>
    <div class="header-buttons">
      <button class="add-btn" on:click={showInput} aria-label="Add new to-do">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="20"
          height="20"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>
    </div>
  </header>

  <main>
    <div 
      class="todos-container"
      use:dndzone={{ items: activeTodos, flipDurationMs: 300 }}
      on:consider={handleActiveDndConsider}
      on:finalize={handleActiveDndFinalize}
    >
      {#each activeTodos as todo (todo.id)}
        <div animate:flip={{ duration: 300 }}>
          <TodoItem {todo} />
        </div>
      {/each}

      {#if showNewTodoInput}
        <div class="new-todo-input-wrapper">
          <input
            bind:this={newTodoInputElement}
            type="text"
            bind:value={newTodoText}
            placeholder="New To-Do..."
            on:blur={handleInputBlur}
            on:keydown={handleInputKeydown}
          />
        </div>
      {/if}
    </div>

    {#if completedTodos.length > 0}
      <div class="completed-separator"></div>
      <div 
        class="todos-container completed"
        use:dndzone={{ items: completedTodos, flipDurationMs: 300 }}
        on:consider={handleCompletedDndConsider}
        on:finalize={handleCompletedDndFinalize}
      >
        {#each completedTodos as todo (todo.id)}
          <div animate:flip={{ duration: 300 }}>
            <TodoItem {todo} />
          </div>
        {/each}
      </div>
    {/if}

    <div class="hotkey-button-container">
      <button class="hotkey-btn" on:click={openHotkeyModal} title="Change hotkey">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <rect x="2" y="3" width="20" height="14" rx="2" ry="2"></rect>
          <line x1="8" y1="21" x2="16" y2="21"></line>
          <line x1="12" y1="17" x2="12" y2="21"></line>
        </svg>
        <span>Hotkey: {formatHotkeyForDisplay(appData.hotkey)}</span>
      </button>
    </div>
  </main>
</div>

<!-- Hotkey Modal -->
{#if showHotkeyModal}
  <div class="modal-overlay" on:click={() => showHotkeyModal = false}>
    <div class="modal-content" on:click|stopPropagation>
      <h3>Press the hotkey combination you want</h3>
      <p class="modal-instruction">Current: {formatHotkeyForDisplay(appData.hotkey)}</p>
      {#if hotkeyCapture.finalCombination}
        <p class="modal-preview">Preview: {hotkeyCapture.finalCombination}</p>
      {/if}
      <p class="modal-instruction">Press and hold modifiers (Ctrl, Shift, Alt) + a key, then release all keys...</p>
    </div>
  </div>
{/if}

<style>
  #app {
    height: auto;
    min-height: 100vh;
  }

  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px 8px;
    flex-shrink: 0;
    position: relative;
  }

  .drag-handle {
    position: absolute;
    top: 6px;
    left: 50%;
    transform: translateX(-50%);
    width: 40px;
    height: 4px;
    background-color: rgba(255, 255, 255, 0.3);
    border-radius: 2px;
    cursor: move;
  }

  .today-header {
    display: flex;
    align-items: baseline;
    gap: 12px;
    margin-right: auto;
  }

  .today-header h1 {
    font-size: 20px;
    font-weight: 600;
    margin: 0;
    user-select: none;
    cursor: pointer;
  }

  .title-input {
    background: transparent;
    border: none;
    outline: none;
    color: white;
    font-size: 20px;
    font-weight: 600;
    font-family: var(--font-sans);
  margin: 0;
    padding: 0;
    user-select: text;
  }

  .time-remaining {
    font-size: 14px;
    color: rgba(255, 255, 255, 0.6);
    font-family: monospace;
    user-select: none;
  }

  .header-buttons {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .add-btn {
    background: var(--neon-green);
    color: black;
    border: none;
    border-radius: 50%;
    width: 28px;
    height: 28px;
  display: flex;
  justify-content: center;
    align-items: center;
    cursor: pointer;
    padding: 0;
    transition: all 0.2s ease;
  }

  .new-todo-input-wrapper {
    padding: 12px 20px;
    display: flex;
    align-items: center;
  }

  .new-todo-input-wrapper input {
    background: transparent;
    border: none;
    outline: none;
    color: white;
    font-size: 1em;
    font-family: var(--font-sans);
    width: 100%;
    padding: 0;
    user-select: text;
  }

  .new-todo-input-wrapper input::placeholder {
    color: rgba(255, 255, 255, 0.4);
  }

  .completed-separator {
    height: 1px;
    background-color: var(--grey-line);
    margin: 5px 20px;
  }

  .hotkey-button-container {
    padding: 12px 20px;
    display: flex;
    justify-content: center;
  }

  .hotkey-btn {
    background: var(--neon-green);
    color: black;
    border: none;
    border-radius: 8px;
    padding: 8px 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 12px;
    font-weight: 500;
    transition: all 0.2s ease;
    opacity: 0.8;
  }

  .hotkey-btn:hover {
    opacity: 1;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(57, 255, 20, 0.3);
  }

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background: rgba(20, 20, 20, 0.95);
    backdrop-filter: blur(20px);
    border-radius: 12px;
    padding: 24px;
    text-align: center;
    border: 1px solid rgba(255, 255, 255, 0.1);
    min-width: 250px;
  }

  .modal-content h3 {
    margin: 0 0 16px 0;
    color: white;
    font-size: 18px;
  }

  .modal-instruction {
    margin: 8px 0;
    color: rgba(255, 255, 255, 0.7);
    font-size: 14px;
  }

  .modal-preview {
    margin: 12px 0;
    color: var(--neon-green);
    font-size: 16px;
    font-weight: 600;
    font-family: monospace;
  }
</style>
