import { writable } from 'svelte/store';
import { fetchMe } from '$lib/api';

export type CurrentUser = {
  id: string;
  email?: string;
  username?: string;
  display_name?: string;
  role: 'user' | 'admin';
  plan: 'free' | 'pro' | 'studio';
};

export const currentUser = writable<CurrentUser | null>(null);
export const userLoading = writable(false);

export async function loadCurrentUser() {
  userLoading.set(true);
  try {
    const me = await fetchMe();
    currentUser.set(me as CurrentUser);
    return me as CurrentUser;
  } catch {
    currentUser.set(null);
    return null;
  } finally {
    userLoading.set(false);
  }
}

export function clearCurrentUser() {
  currentUser.set(null);
}
