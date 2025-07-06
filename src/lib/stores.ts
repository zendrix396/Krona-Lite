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

export interface AppSettings {
  title: string;
  hotkey: string;
  todos: Todo[];
}

const FOLDER_NAME = 'Krona';
const TODOS_FILE_NAME = 'todos.json';

async function saveAppDataToFile(appData: AppSettings) {
  try {
    await invoke('save_todos', { todos: JSON.stringify(appData, null, 2) });
  } catch (error) {
    console.error("Failed to save app data:", error);
  }
}

async function loadAppDataFromFile(): Promise<AppSettings> {
  try {
    const dataJson = await invoke('load_todos') as string;
    if (dataJson) {
      const parsed = JSON.parse(dataJson);
      // Handle migration from old format
      if (Array.isArray(parsed)) {
        return {
          title: 'Krona',
          hotkey: 'Ctrl+Shift+KeyK',
          todos: parsed
        };
      }
      return {
        title: parsed.title || 'Krona',
        hotkey: parsed.hotkey || 'Ctrl+Shift+KeyK',
        todos: parsed.todos || []
      };
    }
  } catch (error) {
    console.error("Failed to load app data:", error);
  }
  return {
    title: 'Krona',
    hotkey: 'Ctrl+Shift+KeyK',
    todos: []
  };
}

function createAppStore() {
  const { subscribe, set, update } = writable<AppSettings>({
    title: 'Krona',
    hotkey: 'Ctrl+Shift+KeyK',
    todos: []
  });

  const updateAndSave = (updater: (appData: AppSettings) => AppSettings) => {
    update(appData => {
      const newAppData = updater(appData);
      saveAppDataToFile(newAppData);
      return newAppData;
    });
  };

  const setAndSave = (appData: AppSettings) => {
    set(appData);
    saveAppDataToFile(appData);
  };

  // Load app data on initialization
  loadAppDataFromFile().then(loadedData => {
    set(loadedData);
  });

  return {
    subscribe,
    setTitle: (title: string) =>
      updateAndSave((appData) => ({ ...appData, title })),
    setHotkey: async (hotkey: string) => {
      updateAndSave((appData) => ({ ...appData, hotkey }));
      try {
        await invoke('update_hotkey', { newHotkey: hotkey });
      } catch (error) {
        console.error("Failed to update hotkey:", error);
      }
    },
    addTodo: (text: string) =>
      updateAndSave((appData) => ({
        ...appData,
        todos: [
          ...appData.todos,
          {
            id: crypto.randomUUID(),
            text,
            completed: false,
            timerActive: false,
            timerStart: null,
            elapsedTime: 0,
          },
        ]
      })),
    deleteTodo: (id: string) => 
      updateAndSave((appData) => {
        const todoToDelete = appData.todos.find(t => t.id === id);
        let deletedTime = 0;
        
        if (todoToDelete) {
          // Calculate total time if timer is running
          if (todoToDelete.timerActive && todoToDelete.timerStart) {
            const sessionTime = Date.now() - todoToDelete.timerStart;
            deletedTime = todoToDelete.elapsedTime + sessionTime;
          } else {
            deletedTime = todoToDelete.elapsedTime;
          }
        }
        
        return {
          ...appData,
          todos: appData.todos.filter((t) => t.id !== id)
        };
      }),
    toggleTodo: (id: string) =>
      updateAndSave((appData) => ({
        ...appData,
        todos: appData.todos.map((todo) =>
          todo.id === id ? { ...todo, completed: !todo.completed } : todo
        )
      })),
    updateTodo: (id: string, text: string) =>
      updateAndSave((appData) => ({
        ...appData,
        todos: appData.todos.map((todo) => (todo.id === id ? { ...todo, text } : todo))
      })),
    toggleTimer: (id: string) =>
      updateAndSave((appData) => ({
        ...appData,
        todos: appData.todos.map((todo) => {
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
              // Start timer (and stop all other timers)
              return {
                ...todo,
                timerActive: true,
                timerStart: Date.now(),
              };
            }
          } else if (todo.timerActive) {
            // Stop other running timers
            const now = Date.now();
            const sessionTime = todo.timerStart ? now - todo.timerStart : 0;
            return {
              ...todo,
              timerActive: false,
              timerStart: null,
              elapsedTime: todo.elapsedTime + sessionTime,
            };
          }
          return todo;
        })
      })),
    reorderTodos: (todos: Todo[]) => 
      updateAndSave((appData) => ({ ...appData, todos })),
  };
}

export const appStore = createAppStore();

// Legacy compatibility - export todos for existing components
export const todos = {
  subscribe: (callback: (todos: Todo[]) => void) => {
    return appStore.subscribe(appData => callback(appData.todos));
  },
  add: (text: string) => appStore.addTodo(text),
  delete: (id: string) => appStore.deleteTodo(id),
  toggle: (id: string) => appStore.toggleTodo(id),
  update: (id: string, text: string) => appStore.updateTodo(id, text),
  toggleTimer: (id: string) => appStore.toggleTimer(id),
  reorder: (todos: Todo[]) => appStore.reorderTodos(todos),
}; 