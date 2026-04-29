use axum::{
    http::{self, header, HeaderValue},
    routing::{get, post},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    config::AppConfig,
    error::AppError,
    repositories::users::ensure_seed_user,
    routes::{
        admin::{
            billing as admin_billing, jobs as admin_jobs, overview as admin_overview,
            sources as admin_sources, system as admin_system, users as admin_users,
        },
        alerts::{create_alert, delete_alert, list_alerts, update_alert},
        auth::{
            auth_status, forgot_password, login, register, resend_verification, reset_password,
            verify_email,
        },
        billing::{billing_checkout, billing_portal, billing_status, billing_webhook},
        consents::{get_consents, post_consent},
        exports::download_export,
        favorites::{add_favorite, delete_favorite, list_favorites},
        health::{health, ready},
        me::{data_export_request, delete_request, get_me, patch_me, save_preferences},
        metrics::metrics,
        notes::update_note,
        notifications::{
            list_notifications, mark_all_notifications_read, mark_notification_read,
            unread_notifications_count,
        },
        plans::list_plans,
        radar::daily_radar,
        reports::{generate_report, get_report, list_reports},
        videos::{list_videos, refresh_videos, scan_videos},
        watchlists::{create_watchlist, delete_watchlist, list_watchlists, update_watchlist},
    },
    state::AppState,
    AuthBearer,
};

pub async fn build_state() -> Result<AppState, AppError> {
    let config = AppConfig::from_env()?;
    let state = AppState::from_config(config.clone()).await?;

    apply_bootstrap_migration(&state.pool).await?;
    ensure_seed_user(&state.pool, &state.config.auth).await?;

    Ok(state)
}

pub fn build_router(state: AppState) -> Result<Router, AppError> {
    let frontend_origin: HeaderValue = state
        .config
        .frontend_origin
        .parse()
        .map_err(|_| AppError::Config("FRONTEND_ORIGIN is invalid".into()))?;

    Ok(
        Router::new()
            .route("/api/v1/health", get(health))
            .route("/api/v1/ready", get(ready))
            .route("/metrics", get(metrics))
            .route("/api/v1/auth/login", post(login))
            .route("/api/v1/auth/status", get(auth_status))
            .route("/api/v1/auth/register", post(register))
            .route("/api/v1/auth/forgot-password", post(forgot_password))
            .route("/api/v1/auth/resend-verification", post(resend_verification))
            .route("/api/v1/auth/verify-email", post(verify_email))
            .route("/api/v1/auth/reset-password", post(reset_password))
            .route("/api/v1/plans", get(list_plans))
            .route("/api/v1/billing/status", get(billing_status))
            .route("/api/v1/billing/checkout", post(billing_checkout))
            .route("/api/v1/billing/portal", post(billing_portal))
            .route("/api/v1/billing/webhook", post(billing_webhook))
            .route(
                "/api/v1/radar/daily",
                get(|auth: AuthBearer, state, query| async move {
                    daily_radar(auth, state, query).await
                }),
            )
            .route(
                "/api/v1/videos",
                get(|auth: AuthBearer, state| async move { list_videos(auth, state).await }).post(
                    |auth: AuthBearer, state, payload| async move {
                        refresh_videos(auth, state, payload).await
                    },
                ),
            )
            .route(
                "/api/v1/videos/scan",
                post(|auth: AuthBearer, state| async move { scan_videos(auth, state).await }),
            )
            .route(
                "/api/v1/notes",
                post(|_auth: AuthBearer, state, payload| async move {
                    update_note(state, payload).await
                }),
            )
            .route("/api/v1/me", get(get_me).patch(patch_me))
            .route("/api/v1/me/preferences", post(save_preferences))
            .route("/api/v1/me/consents", get(get_consents).post(post_consent))
            .route("/api/v1/me/data-export", post(data_export_request))
            .route("/api/v1/me/delete-request", post(delete_request))
            .route(
                "/api/v1/favorites",
                get(|auth: AuthBearer, state| async move { list_favorites(auth, state).await })
                    .post(|auth: AuthBearer, state, payload| async move {
                        add_favorite(auth, state, payload).await
                    }),
            )
            .route(
                "/api/v1/favorites/:platform/:trend_id",
                axum::routing::delete(|auth: AuthBearer, state, path| async move {
                    delete_favorite(auth, state, path).await
                }),
            )
            .route(
                "/api/v1/watchlists",
                get(|auth: AuthBearer, state| async move { list_watchlists(auth, state).await })
                    .post(|auth: AuthBearer, state, payload| async move {
                        create_watchlist(auth, state, payload).await
                    }),
            )
            .route(
                "/api/v1/watchlists/:id",
                axum::routing::patch(|auth: AuthBearer, state, path, payload| async move {
                    update_watchlist(auth, state, path, payload).await
                })
                .delete(|auth: AuthBearer, state, path| async move {
                    delete_watchlist(auth, state, path).await
                }),
            )
            .route(
                "/api/v1/alerts",
                get(|auth: AuthBearer, state| async move { list_alerts(auth, state).await }).post(
                    |auth: AuthBearer, state, payload| async move {
                        create_alert(auth, state, payload).await
                    },
                ),
            )
            .route(
                "/api/v1/alerts/:id",
                axum::routing::patch(|auth: AuthBearer, state, path, payload| async move {
                    update_alert(auth, state, path, payload).await
                })
                .delete(|auth: AuthBearer, state, path| async move {
                    delete_alert(auth, state, path).await
                }),
            )
            .route("/api/v1/notifications", get(|auth: AuthBearer, state| async move { list_notifications(auth, state).await }))
            .route("/api/v1/notifications/unread-count", get(|auth: AuthBearer, state| async move { unread_notifications_count(auth, state).await }))
            .route("/api/v1/notifications/:id/read", post(|auth: AuthBearer, state, path| async move { mark_notification_read(auth, state, path).await }))
            .route("/api/v1/notifications/read-all", post(|auth: AuthBearer, state| async move { mark_all_notifications_read(auth, state).await }))
            .route("/api/v1/exports/:filename", get(|auth: AuthBearer, state, path| async move { download_export(auth, state, path).await }))
            .route(
                "/api/v1/reports",
                get(|auth: AuthBearer, state| async move { list_reports(auth, state).await }),
            )
            .route(
                "/api/v1/reports/generate",
                post(|auth: AuthBearer, state, payload| async move {
                    generate_report(auth, state, payload).await
                }),
            )
            .route(
                "/api/v1/reports/:id",
                get(|auth: AuthBearer, state, path| async move {
                    get_report(auth, state, path).await
                }),
            )
            .route(
                "/api/v1/admin/overview",
                get(|auth: AuthBearer, state| async move { admin_overview(auth, state).await }),
            )
            .route(
                "/api/v1/admin/users",
                get(|auth: AuthBearer, state, query| async move { admin_users(auth, state, query).await }),
            )
            .route(
                "/api/v1/admin/sources",
                get(|auth: AuthBearer, state| async move { admin_sources(auth, state).await }),
            )
            .route(
                "/api/v1/admin/jobs",
                get(|auth: AuthBearer, state| async move { admin_jobs(auth, state).await }),
            )
            .route(
                "/api/v1/admin/system",
                get(|auth: AuthBearer, state| async move { admin_system(auth, state).await }),
            )
            .route(
                "/api/v1/admin/billing",
                get(|auth: AuthBearer, state| async move { admin_billing(auth, state).await }),
            )
            .layer(
                CorsLayer::new()
                    .allow_origin(frontend_origin)
                    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                    .allow_methods([
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PATCH,
                        http::Method::DELETE,
                    ]),
            )
            .layer(TraceLayer::new_for_http())
            .with_state(state),
    )
}

pub async fn apply_bootstrap_migration(pool: &sqlx::PgPool) -> Result<(), AppError> {
    const INIT_SQL: &str = include_str!("../../db/migrations/init.sql");
    sqlx::raw_sql(INIT_SQL).execute(pool).await?;
    Ok(())
}
