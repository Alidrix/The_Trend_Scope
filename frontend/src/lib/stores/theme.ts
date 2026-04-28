import { browser } from '$app/environment';
import { writable } from 'svelte/store';

type Theme = 'light' | 'dark';

const STORAGE_KEY = 'trend-scope-theme';

function detectTheme(): Theme {
  if (!browser) {
    return 'light';
  }

  const saved = localStorage.getItem(STORAGE_KEY);
  if (saved === 'light' || saved === 'dark') {
    return saved;
  }

  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

export const theme = writable<Theme>(detectTheme());

export function applyTheme(value: Theme) {
  if (!browser) {
    return;
  }

  document.documentElement.setAttribute('data-theme', value);
  localStorage.setItem(STORAGE_KEY, value);
}

export function toggleTheme(current: Theme): Theme {
  return current === 'dark' ? 'light' : 'dark';
}

if (browser) {
  const media = window.matchMedia('(prefers-color-scheme: dark)');
  media.addEventListener('change', () => {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (!saved) {
      theme.set(media.matches ? 'dark' : 'light');
    }
  });
}
