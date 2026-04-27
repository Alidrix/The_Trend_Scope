import { get } from 'svelte/store';
import { token } from './stores/auth';

const API_BASE = import.meta.env.VITE_API_BASE || 'http://localhost:4443/api/v1';

async function request(path: string, options: RequestInit = {}) {
  const current = get(token);
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...(options.headers || {})
  };
  if (current) {
    headers['Authorization'] = `Bearer ${current}`;
  }
  const response = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers
  });
  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw new Error(body.message || 'Request failed');
  }
  return response.json();
}

export function login(username: string, password: string) {
  return request('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ username, password })
  });
}

export function authStatus() {
  return request('/auth/status', { method: 'GET' });
}

export function register(username: string, password: string) {
  return request('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ username, password })
  });
}

export function fetchVideos() {
  return request('/videos');
}

export function fetchDailyRadar() {
  return request('/radar/daily');
}

export function fetchPlans() {
  return request('/plans');
}

export function fetchBillingStatus() {
  return request('/billing/status');
}

export function scanVideos() {
  return request('/videos/scan', {
    method: 'POST'
  });
}

export function saveNote(video_id: string, notes: string) {
  return request('/notes', {
    method: 'POST',
    body: JSON.stringify({ video_id, notes })
  });
}
