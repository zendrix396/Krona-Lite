<script lang="ts">
  import { todos } from "./stores";
  import type { Todo } from "./stores";
  import { onMount, onDestroy } from "svelte";

  export let todo: Todo;

  let editing = false;
  let inputElement: HTMLInputElement;
  let hovering = false;
  let interval: number;

  function handleBlur() {
    if (todo.text !== inputElement.value.trim()) {
      todos.update(todo.id, inputElement.value.trim());
    }
    editing = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      inputElement.blur();
    } else if (event.key === "Escape") {
      inputElement.value = todo.text;
      editing = false;
    }
  }

  function handleDoubleClick() {
    editing = true;
  }

  function handleTimerClick() {
    todos.toggleTimer(todo.id);
  }

  function handleDelete() {
    todos.delete(todo.id);
  }

  function formatTime(milliseconds: number) {
    const totalSeconds = Math.floor(milliseconds / 1000);
    const hours = Math.floor(totalSeconds / 3600);
    const minutes = Math.floor((totalSeconds % 3600) / 60);
    const seconds = totalSeconds % 60;
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  }

  function getCurrentElapsedTime() {
    if (todo.timerActive && todo.timerStart) {
      const currentTime = Date.now();
      const sessionTime = currentTime - todo.timerStart;
      return todo.elapsedTime + sessionTime;
    }
    return todo.elapsedTime;
  }

  $: {
    if (editing && inputElement) {
      inputElement.focus();
      inputElement.select();
    }
  }

  let displayTime = 0;

  onMount(() => {
    interval = setInterval(() => {
      if (todo.timerActive) {
        displayTime = getCurrentElapsedTime();
      } else {
        displayTime = todo.elapsedTime;
      }
    }, 100);
  });

  onDestroy(() => {
    if (interval) {
      clearInterval(interval);
    }
  });

  $: displayTime = getCurrentElapsedTime();
</script>

<div 
  class="todo-item" 
  class:completed={todo.completed}
  on:mouseenter={() => hovering = true}
  on:mouseleave={() => hovering = false}
  role="listitem"
>
  <div
    class="checkbox-wrapper"
    role="button"
    tabindex="0"
    on:click={() => todos.toggle(todo.id)}
    on:keydown={(e) => (e.key === "Enter" || e.key === " ") && todos.toggle(todo.id)}
  >
    <div class="checkbox" class:checked={todo.completed}>
      {#if todo.completed}
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="var(--neon-green)"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
          ><polyline points="20 6 9 17 4 12" /></svg
        >
      {/if}
    </div>
  </div>

  {#if editing}
    <input
      bind:this={inputElement}
      type="text"
      value={todo.text}
      on:blur={handleBlur}
      on:keydown={handleKeydown}
      class="edit-input"
    />
  {:else}
    <span 
      class="text" 
      on:dblclick={handleDoubleClick}
      role="button"
      tabindex="0"
      on:keydown={(e) => (e.key === "Enter" || e.key === " ") && handleDoubleClick()}
    >
      {todo.text}
    </span>
  {/if}

  <div class="controls" class:visible={hovering || todo.timerActive || displayTime > 0}>
    {#if !todo.completed && (displayTime > 0 || todo.timerActive)}
      <span class="timer-display">{formatTime(displayTime)}</span>
    {/if}
    {#if !todo.completed}
      <button class="timer-btn" on:click={handleTimerClick} class:running={todo.timerActive} aria-label="Toggle timer">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <circle cx="12" cy="12" r="10"/>
          <polyline points="12,6 12,12 16,14"/>
        </svg>
      </button>
    {/if}
    <button class="delete-btn" on:click={handleDelete} aria-label="Delete to-do">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="14"
        height="14"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polyline points="3 6 5 6 21 6"></polyline>
        <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2 2h4a2 2 0 0 1 2 2v2"></path>
        <line x1="10" y1="11" x2="10" y2="17"></line>
        <line x1="14" y1="11" x2="14" y2="17"></line>
      </svg>
    </button>
  </div>
</div>

<style>
  .todo-item {
    display: flex;
    align-items: center;
    padding: 12px 20px;
    border-bottom: 1px solid var(--grey-line);
    transition: all 0.2s ease;
  }

  .todo-item.completed .text {
    text-decoration: line-through;
    opacity: 0.5;
  }

  .checkbox-wrapper {
    cursor: pointer;
    margin-right: 15px;
    display: flex;
    align-items: center;
  }

  .checkbox {
    width: 20px;
    height: 20px;
    border: 2px solid var(--grey-line);
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;
    transition: all 0.2s ease;
  }

  .checkbox.checked {
    border-color: var(--neon-green);
    background-color: rgba(57, 255, 20, 0.1);
  }

  .text {
    flex-grow: 1;
    cursor: text;
    user-select: none;
    color: white;
  }

  .edit-input {
    flex-grow: 1;
    background: transparent;
    border: none;
    outline: none;
    color: white;
    font-size: 1em;
    font-family: var(--font-sans);
    padding: 0;
    margin: 0;
    user-select: text;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-left: 10px;
    opacity: 0;
    transition: opacity 0.2s ease;
  }

  .controls.visible {
    opacity: 1;
  }

  .timer-display {
    font-size: 11px;
    color: rgba(255, 255, 255, 0.6);
    font-family: monospace;
    min-width: 60px;
  }

  .timer-btn, .delete-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    color: rgba(255, 255, 255, 0.4);
    padding: 4px;
    border-radius: 50%;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .timer-btn:hover {
    color: rgba(255, 255, 255, 0.8);
    background: rgba(255, 255, 255, 0.1);
  }
  
  .delete-btn:hover {
    color: var(--light-red);
    background: rgba(255, 77, 77, 0.15);
  }

  .timer-btn.running {
    color: var(--neon-green);
  }
</style> 