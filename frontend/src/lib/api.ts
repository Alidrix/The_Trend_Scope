import { get } from 'svelte/store';
import { token } from './stores/auth';

export const API_BASE = import.meta.env.VITE_API_BASE || 'http://localhost:4443/api/v1';
export function buildApiUrl(path: string) { return path.startsWith('http') ? path : `${API_BASE}${path}`; }

async function request(path: string, options: RequestInit = {}) {
  const current = get(token);
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
    ...(options.headers || {})
  };
  if (current) headers['Authorization'] = `Bearer ${current}`;

  const response = await fetch(`${API_BASE}${path}`, { ...options, headers });
  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw new Error(body.message || 'Request failed');
  }
  return response.json();
}

export const login = (username: string, password: string) => request('/auth/login', { method: 'POST', body: JSON.stringify({ username, password }) });
export const authStatus = () => request('/auth/status', { method: 'GET' });
export const fetchMe = () => request('/me');

export type RegisterPayload = { email?: string; username?: string; password: string; display_name?: string; country?: string; profile_type?: string; accept_terms?: boolean; accept_privacy?: boolean; marketing_opt_in?: boolean; };

export function register(username: string, password: string, extra?: Record<string, unknown>): Promise<unknown>;
export function register(payload: RegisterPayload): Promise<unknown>;
export function register(usernameOrPayload: string | RegisterPayload, password?: string, extra: Record<string, unknown> = {}): Promise<unknown> {
  if (typeof usernameOrPayload === 'string') {
    return request('/auth/register', { method: 'POST', body: JSON.stringify({ username: usernameOrPayload, password, accept_terms: true, accept_privacy: true, ...extra }) });
  }
  const payload = usernameOrPayload;
  const username = payload.username ?? payload.email ?? '';
  return request('/auth/register', { method: 'POST', body: JSON.stringify({ ...payload, username, accept_terms: payload.accept_terms ?? true, accept_privacy: payload.accept_privacy ?? true }) });
}

export const forgotPassword = (email: string) => request('/auth/forgot-password', { method: 'POST', body: JSON.stringify({ email }) });
export const resetPassword = (tokenValue: string, password: string) => request('/auth/reset-password', { method: 'POST', body: JSON.stringify({ token: tokenValue, password }) });

export const saveOnboarding = (payload: { primary_goal: string; platforms: string[]; categories: string[]; regions: string[] }) => request('/me/preferences', { method: 'POST', body: JSON.stringify(payload) });
export const fetchVideos = () => request('/videos');

export const fetchDailyRadar = (filters: { platform?: string; region?: string; category?: string; format?: string } = {}) => {
  const query = new URLSearchParams();
  Object.entries(filters).forEach(([key, value]) => {
    if (value) query.set(key, value);
  });
  return request(`/radar/daily${query.toString() ? `?${query.toString()}` : ''}`);
};

export const fetchPlans = () => request('/plans');
export const fetchBillingStatus = () => request('/billing/status');
export const createCheckout = (plan: 'pro' | 'studio') => request('/billing/checkout', { method: 'POST', body: JSON.stringify({ plan }) });
export const openBillingPortal = () => request('/billing/portal', { method: 'POST' });
export const scanVideos = () => request('/videos/scan', { method: 'POST' });
export const saveNote = (video_id: string, notes: string) => request('/notes', { method: 'POST', body: JSON.stringify({ video_id, notes }) });

export const fetchFavorites = () => request('/favorites');
export const addFavorite = (platform: string, trend_id: string) => request('/favorites', { method: 'POST', body: JSON.stringify({ platform, trend_id }) });
export const deleteFavorite = (platform: string, trend_id: string) => request(`/favorites/${platform}/${trend_id}`, { method: 'DELETE' });

export const fetchWatchlists = () => request('/watchlists');
export const createWatchlist = (payload: Record<string, unknown>) => request('/watchlists', { method: 'POST', body: JSON.stringify(payload) });
export const updateWatchlist = (id: string, payload: Record<string, unknown>) => request(`/watchlists/${id}`, { method: 'PATCH', body: JSON.stringify(payload) });
export const deleteWatchlist = (id: string) => request(`/watchlists/${id}`, { method: 'DELETE' });

export const fetchAlerts = () => request('/alerts');
export const createAlert = (payload: Record<string, unknown>) => request('/alerts', { method: 'POST', body: JSON.stringify(payload) });
export const updateAlert = (id: string, payload: Record<string, unknown>) => request(`/alerts/${id}`, { method: 'PATCH', body: JSON.stringify(payload) });
export const deleteAlert = (id: string) => request(`/alerts/${id}`, { method: 'DELETE' });

export const fetchReports = () => request('/reports');
export const generateReport = (payload: Record<string, unknown> = {}) => request('/reports/generate', { method: 'POST', body: JSON.stringify(payload) });
export const fetchReport = (id: string) => request(`/reports/${id}`);

export const fetchConsents = () => request('/me/consents');
export const saveConsent = (payload: { consent_type: string; granted: boolean; version: string }) => request('/me/consents', { method: 'POST', body: JSON.stringify(payload) });
export const requestDataExport = () => request('/me/data-export', { method: 'POST' });
export const requestDeleteAccount = () => request('/me/delete-request', { method: 'POST' });

export const resendVerification = (email: string) => request('/auth/resend-verification', { method: 'POST', body: JSON.stringify({ email }) });
export const verifyEmail = (tokenValue: string) => request('/auth/verify-email', { method: 'POST', body: JSON.stringify({ token: tokenValue }) });

export type Platform = 'youtube' | 'tiktok' | 'instagram';
export type ReportFormat = 'json' | 'csv' | 'pdf';

export const fetchAdminOverview = () => request('/admin/overview');
export const fetchAdminUsers = (params: Record<string, string | number> = {}) => {
  const query = new URLSearchParams();
  Object.entries(params).forEach(([k, v]) => query.set(k, String(v)));
  return request(`/admin/users${query.toString() ? `?${query.toString()}` : ''}`);
};
export const fetchAdminSources = () => request('/admin/sources');
export const fetchAdminJobs = () => request('/admin/jobs');
export const fetchAdminSystem = () => request('/admin/system');
export const fetchAdminBilling = () => request('/admin/billing');

export const fetchNotifications = () => request('/notifications');
export const fetchUnreadNotificationsCount = () => request('/notifications/unread-count');
export const markNotificationRead = (id: string) => request(`/notifications/${id}/read`, { method: 'POST' });
export const markAllNotificationsRead = () => request('/notifications/read-all', { method: 'POST' });

export const fetchAdminEmailLogs = () => request('/admin/email-logs');
export const fetchAdminNotifications = () => request('/admin/notifications');
export const fetchAdminExports = () => request('/admin/exports');
export const testAdminTelegram = (payload: { chat_id?: string }) =>
  request('/admin/test-telegram', { method: 'POST', body: JSON.stringify(payload) });
export const testAdminSmtp = (payload: { to: string }) =>
  request('/admin/test-smtp', { method: 'POST', body: JSON.stringify(payload) });
