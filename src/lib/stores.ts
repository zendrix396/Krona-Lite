import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

export interface Todo {
    id: string;
    text: string;
    completed: boolean;
    timerActive: boolean;
    timerStart: number | null;
    elapsedTime: number;
}

const FOLDER_NAME = 'Krona';
const TODOS_FILE_NAME = 'todos.json';

async function saveTodosToFile(todos: Todo[]) {
  try {
    await invoke('save_todos', { todos: JSON.stringify(todos, null, 2) });
  } catch (error) {
    console.error("Failed to save todos:", error);
  }
}

async function loadTodosFromFile(): Promise<Todo[]> {
  try {
    const todosJson = await invoke('load_todos') as string;
    if (todosJson) {
      return JSON.parse(todosJson);
    }
  } catch (error) {
    console.error("Failed to load todos:", error);
  }
  return [];
}

function createTodos() {
  const { subscribe, set, update } = writable<Todo[]>([]);

  const updateAndSave = (updater: (todos: Todo[]) => Todo[]) => {
    update(todos => {
      const newTodos = updater(todos);
      saveTodosToFile(newTodos);
      return newTodos;
    });
  };
  
  const setAndSave = (todos: Todo[]) => {
    set(todos);
    saveTodosToFile(todos);
  }

  // Load todos on initialization
  loadTodosFromFile().then(loadedTodos => {
    set(loadedTodos);
  });

  return {
    subscribe,
    add: (text: string) =>
      updateAndSave((todos) => [
        ...todos,
        {
          id: crypto.randomUUID(),
          text,
          completed: false,
          timerActive: false,
          timerStart: null,
          elapsedTime: 0,
        },
      ]),
    delete: (id: string) => updateAndSave((todos) => todos.filter((t) => t.id !== id)),
    toggle: (id: string) =>
      updateAndSave((todos) =>
        todos.map((todo) =>
          todo.id === id ? { ...todo, completed: !todo.completed } : todo
        )
      ),
    update: (id: string, text: string) =>
      updateAndSave((todos) =>
        todos.map((todo) => (todo.id === id ? { ...todo, text } : todo))
      ),
    toggleTimer: (id: string) =>
      updateAndSave((todos) =>
        todos.map((todo) => {
          if (todo.id === id) {
            if (todo.timerActive) {
              // Stop timer
              const now = Date.now();
              const sessionTime = todo.timerStart ? now - todo.timerStart : 0;
              return {
                ...todo,
                timerActive: false,
                timerStart: null,
                elapsedTime: todo.elapsedTime + sessionTime,
              };
            } else {
              // Start timer
              return {
                ...todo,
                timerActive: true,
                timerStart: Date.now(),
              };
            }
          }
          return todo;
        })
      ),
    reorder: (items: Todo[]) => setAndSave(items),
  };
}

export const todos = createTodos(); 