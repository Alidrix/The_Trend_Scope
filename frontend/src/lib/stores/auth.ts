import { writable } from 'svelte/store';

const stored = typeof localStorage !== 'undefined' ? localStorage.getItem('token') : null;
export const token = writable<string | null>(stored);

token.subscribe((value) => {
  if (typeof localStorage !== 'undefined') {
    if (value) {
      localStorage.setItem('token', value);
    } else {
      localStorage.removeItem('token');
    }
  }
});
