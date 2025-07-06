<script lang="ts">
  import { todos, type Todo } from "$lib/stores.ts";
  import TodoItem from "$lib/TodoItem.svelte";
  import { flip } from "svelte/animate";
  import { dndzone } from "svelte-dnd-action";
  import { onMount, onDestroy } from "svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let showNewTodoInput = false;
  let newTodoText = "";
  let newTodoInputElement: HTMLInputElement;
  let timeRemaining = "";
  let dayInterval: number;
  let isPinned = false;

  $: activeTodos = $todos.filter((t) => !t.completed);
  $: completedTodos = $todos.filter((t) => t.completed);

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
      const padding = 20;

      let totalHeight = headerHeight + padding;
      totalHeight += activeTodos.length * todoHeight;
      totalHeight += inputHeight;
      
      if (completedTodos.length > 0) {
        totalHeight += separatorHeight;
        totalHeight += completedTodos.length * todoHeight;
      }

      // Constrain height between min and max
      const minHeight = 120;
      const maxHeight = 600;
      const finalHeight = Math.max(minHeight, Math.min(maxHeight, totalHeight));

      await window.setSize({ width: 350, height: finalHeight });
    } catch (error) {
      console.error("Failed to resize window:", error);
    }
  }

  function updateTimeRemaining() {
    const now = new Date();
    const tomorrow = new Date(now);
    tomorrow.setDate(tomorrow.getDate() + 1);
    tomorrow.setHours(2, 0, 0, 0);
    
    const diff = tomorrow.getTime() - now.getTime();
    const hours = Math.floor(diff / (1000 * 60 * 60));
    const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
    const seconds = Math.floor((diff % (1000 * 60)) / 1000);
    
    timeRemaining = `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }

  async function togglePin() {
    isPinned = !isPinned;
    try {
      const window = getCurrentWindow();
      await window.setAlwaysOnTop(isPinned);
    } catch (error) {
      console.error("Failed to set always-on-top:", error);
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

  onMount(() => {
    updateTimeRemaining();
    dayInterval = setInterval(updateTimeRemaining, 1000);
    updateWindowSize();
  });

  onDestroy(() => {
    if (dayInterval) {
      clearInterval(dayInterval);
    }
  });
</script>

<svelte:window on:keydown={handleKeyDown} />

<div id="app">
  <header>
    <div data-tauri-drag-region class="drag-handle"></div>
    <div class="today-header">
      <h1>Today</h1>
      <span class="time-remaining">{timeRemaining}</span>
    </div>
    <div class="header-buttons">
      <button
        class="pin-btn"
        class:pinned={isPinned}
        on:click={togglePin}
        aria-label="Pin window on top"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"></path>
        </svg>
      </button>
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
  </main>
</div>

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

  .pin-btn {
    background: transparent;
    color: white;
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
    opacity: 0.7;
  }

  .pin-btn:hover {
    opacity: 1;
    background-color: rgba(255, 255, 255, 0.1);
  }

  .pin-btn.pinned {
    opacity: 1;
    color: var(--neon-green);
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
</style>
