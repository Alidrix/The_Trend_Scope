import { writable } from 'svelte/store';

export type Notification = {
  id: string;
  title: string;
  body: string;
  level?: 'info' | 'warning' | 'success';
};

export const notifications = writable<Notification[]>([]);
export const notificationsEnabled = writable<boolean>(true);

export function pushNotification(note: Notification) {
  notifications.update((list) => [...list, note]);
  if (typeof Notification !== 'undefined' && Notification.permission === 'granted') {
    new window.Notification(note.title, { body: note.body });
  }
}

export function requestPermission() {
  if (typeof Notification !== 'undefined' && Notification.permission === 'default') {
    Notification.requestPermission();
  }
}
