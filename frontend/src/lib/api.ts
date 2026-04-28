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

export type RegisterPayload = {
  email?: string;
  username?: string;
  password: string;
  display_name?: string;
  country?: string;
  profile_type?: string;
  accept_terms?: boolean;
  accept_privacy?: boolean;
  marketing_opt_in?: boolean;
};

export function register(username: string, password: string, extra?: Record<string, unknown>): Promise<unknown>;
export function register(payload: RegisterPayload): Promise<unknown>;
export function register(
  usernameOrPayload: string | RegisterPayload,
  password?: string,
  extra: Record<string, unknown> = {}
): Promise<unknown> {
  if (typeof usernameOrPayload === 'string') {
    return request('/auth/register', {
      method: 'POST',
      body: JSON.stringify({
        username: usernameOrPayload,
        password,
        accept_terms: true,
        accept_privacy: true,
        ...extra
      })
    });
  }

  const payload = usernameOrPayload;
  const username = payload.username ?? payload.email ?? '';

  return request('/auth/register', {
    method: 'POST',
    body: JSON.stringify({
      ...payload,
      username,
      accept_terms: payload.accept_terms ?? true,
      accept_privacy: payload.accept_privacy ?? true
    })
  });
}

export async function saveOnboarding(payload: {
  primary_goal: string;
  platforms: string[];
  categories: string[];
  regions: string[];
}) {
  return request('/me/preferences', {
    method: 'POST',
    body: JSON.stringify(payload)
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
