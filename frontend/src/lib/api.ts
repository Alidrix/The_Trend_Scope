import { get } from "svelte/store";
import { token } from "./stores/auth";

// Core request
export const API_BASE =
  import.meta.env.VITE_API_BASE || "http://localhost:4443/api/v1";
export function buildApiUrl(path: string) {
  return path.startsWith("http") ? path : `${API_BASE}${path}`;
}
async function request(path: string, options: RequestInit = {}) {
  const current = get(token);
  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...(options.headers || {}),
  };
  if (current) headers["Authorization"] = `Bearer ${current}`;
  const response = await fetch(`${API_BASE}${path}`, { ...options, headers });
  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw new Error(body.message || "Request failed");
  }
  return response.json();
}

// Auth
export const login = (username: string, password: string) =>
  request("/auth/login", {
    method: "POST",
    body: JSON.stringify({ username, password }),
  });
export const authStatus = () => request("/auth/status", { method: "GET" });
export const fetchMe = () => request("/me");
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
export function register(
  username: string,
  password: string,
  extra?: Record<string, unknown>,
): Promise<unknown>;
export function register(payload: RegisterPayload): Promise<unknown>;
export function register(
  usernameOrPayload: string | RegisterPayload,
  password?: string,
  extra: Record<string, unknown> = {},
): Promise<unknown> {
  if (typeof usernameOrPayload === "string") {
    return request("/auth/register", {
      method: "POST",
      body: JSON.stringify({
        username: usernameOrPayload,
        password,
        accept_terms: true,
        accept_privacy: true,
        ...extra,
      }),
    });
  }
  const payload = usernameOrPayload;
  const username = payload.username ?? payload.email ?? "";
  return request("/auth/register", {
    method: "POST",
    body: JSON.stringify({
      ...payload,
      username,
      accept_terms: payload.accept_terms ?? true,
      accept_privacy: payload.accept_privacy ?? true,
    }),
  });
}
export const forgotPassword = (email: string) =>
  request("/auth/forgot-password", {
    method: "POST",
    body: JSON.stringify({ email }),
  });
export const resetPassword = (tokenValue: string, password: string) =>
  request("/auth/reset-password", {
    method: "POST",
    body: JSON.stringify({ token: tokenValue, password }),
  });
export const saveOnboarding = (payload: {
  primary_goal: string;
  platforms: string[];
  categories: string[];
  regions: string[];
}) =>
  request("/me/preferences", { method: "POST", body: JSON.stringify(payload) });
export const fetchVideos = () => request("/videos");
export const fetchDailyRadar = (
  filters: {
    platform?: string;
    region?: string;
    category?: string;
    format?: string;
  } = {},
) => {
  const query = new URLSearchParams();
  Object.entries(filters).forEach(([key, value]) => {
    if (value) query.set(key, value);
  });
  return request(
    `/radar/daily${query.toString() ? `?${query.toString()}` : ""}`,
  );
};
export const fetchPlans = () => request("/plans");
export const fetchBillingStatus = () => request("/billing/status");
export const createCheckout = (plan: "pro" | "studio") =>
  request("/billing/checkout", {
    method: "POST",
    body: JSON.stringify({ plan }),
  });
export const openBillingPortal = () =>
  request("/billing/portal", { method: "POST" });
export const scanVideos = () => request("/videos/scan", { method: "POST" });
export const saveNote = (video_id: string, notes: string) =>
  request("/notes", {
    method: "POST",
    body: JSON.stringify({ video_id, notes }),
  });
export const fetchFavorites = () => request("/favorites");
export const addFavorite = (platform: string, trend_id: string) =>
  request("/favorites", {
    method: "POST",
    body: JSON.stringify({ platform, trend_id }),
  });
export const deleteFavorite = (platform: string, trend_id: string) =>
  request(`/favorites/${platform}/${trend_id}`, { method: "DELETE" });
export const fetchWatchlists = () => request("/watchlists");
export const createWatchlist = (payload: Record<string, unknown>) =>
  request("/watchlists", { method: "POST", body: JSON.stringify(payload) });
export const updateWatchlist = (id: string, payload: Record<string, unknown>) =>
  request(`/watchlists/${id}`, {
    method: "PATCH",
    body: JSON.stringify(payload),
  });
export const deleteWatchlist = (id: string) =>
  request(`/watchlists/${id}`, { method: "DELETE" });
export const fetchAlerts = () => request("/alerts");
export const createAlert = (payload: Record<string, unknown>) =>
  request("/alerts", { method: "POST", body: JSON.stringify(payload) });
export const updateAlert = (id: string, payload: Record<string, unknown>) =>
  request(`/alerts/${id}`, { method: "PATCH", body: JSON.stringify(payload) });
export const deleteAlert = (id: string) =>
  request(`/alerts/${id}`, { method: "DELETE" });
export const fetchReports = () => request("/reports");
export const generateReport = (payload: Record<string, unknown> = {}) =>
  request("/reports/generate", {
    method: "POST",
    body: JSON.stringify(payload),
  });
export const fetchReport = (id: string) => request(`/reports/${id}`);
export const fetchConsents = () => request("/me/consents");
export const saveConsent = (payload: {
  consent_type: string;
  granted: boolean;
  version: string;
}) =>
  request("/me/consents", { method: "POST", body: JSON.stringify(payload) });
export const requestDataExport = () =>
  request("/me/data-export", { method: "POST" });
export const requestDeleteAccount = () =>
  request("/me/delete-request", { method: "POST" });
export const resendVerification = (email: string) =>
  request("/auth/resend-verification", {
    method: "POST",
    body: JSON.stringify({ email }),
  });
export const verifyEmail = (tokenValue: string) =>
  request("/auth/verify-email", {
    method: "POST",
    body: JSON.stringify({ token: tokenValue }),
  });

// Admin types
export type AdminOverview = {
  users: { total: number; verified: number; admins: number };
  plans: { free: number; pro: number; studio: number };
  subscriptions: { total: number; active: number; inactive: number };
  alerts: {
    rules_enabled: number;
    deliveries_sent_24h: number;
    deliveries_failed_24h: number;
    deliveries_skipped_24h: number;
  };
  reports: { pending: number; completed_24h: number; failed_24h: number };
  notifications: { total: number; unread: number };
  emails: { sent_24h: number; failed_24h: number; skipped_24h: number };
  sources: { youtube: string; tiktok: string; instagram: string };
};
export type AdminSystem = {
  runtime: { env: string; frontend_origin: string };
  services: Record<string, string>;
  integrations: Record<string, string>;
  storage: { local_exports_dir: string; s3: string };
};
export type AdminBilling = {
  subscriptions: {
    total: number;
    active: number;
    inactive: number;
    pro: number;
    studio: number;
  };
  mrr: {
    currency: string;
    estimate_cents: number;
    pro_unit_cents: number;
    studio_unit_cents: number;
  };
  stripe: {
    configured: boolean;
    webhook_configured: boolean;
    price_pro_configured: boolean;
    price_studio_configured: boolean;
  };
};
export type GoLiveItem = {
  key: string;
  label: string;
  status: "ok" | "warning" | "error" | "manual";
  blocking: boolean;
};
export type GoLiveChecklistResponse = { items: GoLiveItem[] };
export type AdminSmoke = {
  ok: boolean;
  checks: Record<string, string>;
  blocking?: Record<string, boolean>;
};
export type AdminEmailLog = {
  recipient: string;
  subject: string;
  status: string;
  error_message?: string | null;
  created_at?: string;
};
export type AdminExport = {
  id: string;
  title: string;
  format: string;
  file_url?: string | null;
  created_at?: string;
};
export type AdminNotificationSnapshot = {
  total: number;
  unread: number;
  latest?: Array<{
    id?: string;
    title?: string;
    body?: string;
    type?: string;
    created_at?: string;
  }>;
};
export type AdminUserRow = {
  id: string;
  username: string;
  role: string;
  plan?: string;
  created_at?: string;
};
export type AdminUsersResponse = {
  users: AdminUserRow[];
};
export type AdminSourceStatus = {
  sources: Record<string, string>;
};
export type AdminJobsSnapshot = {
  pending_reports?: number;
  completed_reports_24h?: number;
  failed_reports_24h?: number;
};

export type AdminEmailLogsResponse = {
  logs: AdminEmailLog[];
};
export type AdminExportsResponse = {
  exports: AdminExport[];
};
export type AdminTestResult = {
  ok?: boolean;
  sent?: boolean;
  reason?: string;
  message?: string;
  configured?: Record<string, boolean>;
};

// Admin API
export const fetchAdminOverview = () =>
  request("/admin/overview") as Promise<AdminOverview>;
export const fetchAdminUsers = (
  params: Record<string, string | number> = {},
): Promise<AdminUsersResponse> => {
  const query = new URLSearchParams();
  Object.entries(params).forEach(([k, v]) => query.set(k, String(v)));
  return request(
    `/admin/users${query.toString() ? `?${query.toString()}` : ""}`,
  ) as Promise<AdminUsersResponse>;
};
export const fetchAdminSources = () =>
  request("/admin/sources") as Promise<AdminSourceStatus>;
export const fetchAdminJobs = () =>
  request("/admin/jobs") as Promise<AdminJobsSnapshot>;
export const fetchAdminSystem = () =>
  request("/admin/system") as Promise<AdminSystem>;
export const fetchAdminBilling = () =>
  request("/admin/billing") as Promise<AdminBilling>;
export const fetchNotifications = () => request("/notifications");
export const fetchUnreadNotificationsCount = () =>
  request("/notifications/unread-count");
export const markNotificationRead = (id: string) =>
  request(`/notifications/${id}/read`, { method: "POST" });
export const markAllNotificationsRead = () =>
  request("/notifications/read-all", { method: "POST" });
export const fetchAdminEmailLogs = () =>
  request("/admin/email-logs") as Promise<AdminEmailLogsResponse>;
export const fetchAdminNotifications = () =>
  request("/admin/notifications") as Promise<AdminNotificationSnapshot>;
export const fetchAdminExports = () =>
  request("/admin/exports") as Promise<AdminExportsResponse>;
export const testAdminTelegram = (payload: { chat_id?: string }) =>
  request("/admin/test-telegram", {
    method: "POST",
    body: JSON.stringify(payload),
  }) as Promise<AdminTestResult>;
export const testAdminSmtp = (payload: { to: string }) =>
  request("/admin/test-smtp", {
    method: "POST",
    body: JSON.stringify(payload),
  }) as Promise<AdminTestResult>;
export const fetchAdminGoLiveChecklist = () =>
  request("/admin/go-live-checklist") as Promise<GoLiveChecklistResponse>;
export const testAdminYoutube = () =>
  request("/admin/test-youtube", { method: "POST" }) as Promise<AdminTestResult>;
export const testAdminStripe = () =>
  request("/admin/test-stripe", { method: "POST" }) as Promise<AdminTestResult>;
export const runAdminYoutubeCheck = testAdminYoutube;
export const runAdminStripeCheck = testAdminStripe;
export const fetchAdminSmoke = () =>
  request("/admin/smoke") as Promise<AdminSmoke>;
