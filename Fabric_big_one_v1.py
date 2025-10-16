# scaffold.py
"""
Scaffold du projet Veille YouTube/TikTok.

➡️ Exécution conseillée :
    python3 scaffold.py

Ce script crée l'arborescence complète du projet (fichiers Python, templates, static,
Dockerfile, requirements, tests, .gitignore, .env.example). Ensuite tu peux :

    cp .env.example .env   # mettre ta clé YOUTUBE_API_KEY
    docker build -t veille-tiktok . && \
    docker run -d --name veille-tiktok -p 4443:4443 --env-file .env veille-tiktok

Le code ci-dessous NE contient PAS de texte brut hors Python (pour éviter les SyntaxError).
"""
from __future__ import annotations
import os
from pathlib import Path

ROOT = Path.cwd()

FILES: dict[str, str] = {}

def add(path: str, content: str) -> None:
    FILES[path] = content.lstrip("\n")

# ---------------------------
# .gitignore
# ---------------------------
add(
    ".gitignore",
    r"""
# secrets & runtime
.env
*.db
__pycache__/
*.pyc
venv/
.venv/
.pytest_cache/
    """,
)

# ---------------------------
# requirements.txt
# ---------------------------
add(
    "requirements.txt",
    r"""
Flask==3.0.3
Flask-SQLAlchemy==3.1.1
SQLAlchemy==2.0.32
google-api-python-client==2.137.0
python-dotenv==1.0.1
APScheduler==3.10.4
requests==2.32.3
gunicorn==22.0.0
isodate==0.6.1
pytest==8.3.3
    """,
)

# ---------------------------
# .env.example
# ---------------------------
add(
    ".env.example",
    r"""
# Sécurité minimale
APP_USERNAME=admin
APP_PASSWORD=adminpass
SECRET_KEY=change-me

# YouTube
YOUTUBE_API_KEY=COLLE_TA_CLE_ICI

# Horaires
SCHEDULE_MORNING=08:00
SCHEDULE_NOON=13:00
SCHEDULE_EVENING=19:00

# Régions (FR,US,ES)
REGIONS=FR,US,ES

# Thèmes (modifiable)
THEMES=nourriture,voiture,business,drôle,influenceurs

# Alerte flash (minutes)
ALERT_INTERVAL_MIN=15
    """,
)

# ---------------------------
# config.py
# ---------------------------
add(
    "config.py",
    r"""
import os
from dotenv import load_dotenv

load_dotenv()

class Config:
    SECRET_KEY = os.getenv("SECRET_KEY", "change-me")
    SQLALCHEMY_DATABASE_URI = os.getenv("DATABASE_URL", "sqlite:///data.db")
    SQLALCHEMY_TRACK_MODIFICATIONS = False

    # Security (basic auth)
    APP_USERNAME = os.getenv("APP_USERNAME", "admin")
    APP_PASSWORD = os.getenv("APP_PASSWORD", "adminpass")

    # YouTube API
    YT_API_KEY = os.getenv("YOUTUBE_API_KEY", "")

    # Scheduling (HH:MM, 24h)
    SCHEDULE_MORNING = os.getenv("SCHEDULE_MORNING", "08:00")
    SCHEDULE_NOON = os.getenv("SCHEDULE_NOON", "13:00")
    SCHEDULE_EVENING = os.getenv("SCHEDULE_EVENING", "19:00")

    # Regions à interroger (FR/US/ES)
    REGIONS = [r.strip() for r in os.getenv("REGIONS", "FR,US,ES").split(",") if r.strip()]

    # Catégories/Thèmes (liste séparée par virgule)
    THEMES = [t.strip() for t in os.getenv(
        "THEMES",
        "nourriture,voiture,business,drôle,influenceurs"
    ).split(",") if t.strip()]

    # Alertes: intervalle en minutes (surveillance flash)
    ALERT_INTERVAL_MIN = int(os.getenv("ALERT_INTERVAL_MIN", "15"))
    """,
)

# ---------------------------
# models.py
# ---------------------------
add(
    "models.py",
    r"""
from datetime import datetime
from flask_sqlalchemy import SQLAlchemy
from sqlalchemy import Index

db = SQLAlchemy()

class Video(db.Model):
    __tablename__ = "videos"
    id = db.Column(db.Integer, primary_key=True)
    yt_id = db.Column(db.String(32), unique=True, nullable=False)
    title = db.Column(db.String(512), nullable=False)
    channel = db.Column(db.String(256))
    url = db.Column(db.String(512))
    thumbnail = db.Column(db.String(512))
    category_id = db.Column(db.String(16))
    region = db.Column(db.String(8))
    language = db.Column(db.String(8))
    published_at = db.Column(db.DateTime)

    # Metrics à l'ajout
    views_initial = db.Column(db.Integer, default=0)
    likes_initial = db.Column(db.Integer, default=0)

    # Dernières métriques connues
    views_current = db.Column(db.Integer, default=0)
    likes_current = db.Column(db.Integer, default=0)

    # Drapeaux
    is_short = db.Column(db.Boolean, default=False)
    used = db.Column(db.Boolean, default=False)

    created_at = db.Column(db.DateTime, default=datetime.utcnow)

    def views_per_hour(self) -> float:
        if not self.published_at or not self.views_current:
            return 0.0
        hours = max((datetime.utcnow() - self.published_at).total_seconds() / 3600.0, 0.016)
        return float(self.views_current) / hours

Index("idx_video_used_created", Video.used, Video.created_at.desc())

class Note(db.Model):
    __tablename__ = "notes"
    id = db.Column(db.Integer, primary_key=True)
    video_id = db.Column(db.Integer, db.ForeignKey("videos.id"), nullable=False)
    content = db.Column(db.Text, default="")
    created_at = db.Column(db.DateTime, default=datetime.utcnow)

class StatSnapshot(db.Model):
    __tablename__ = "stat_snapshots"
    id = db.Column(db.Integer, primary_key=True)
    video_id = db.Column(db.Integer, db.ForeignKey("videos.id"), nullable=False)
    views = db.Column(db.Integer, default=0)
    likes = db.Column(db.Integer, default=0)
    captured_at = db.Column(db.DateTime, default=datetime.utcnow)
    """,
)

# ---------------------------
# youtube_client.py
# ---------------------------
add(
    "youtube_client.py",
    r"""
from datetime import datetime, timezone
from typing import List, Dict, Any
from googleapiclient.discovery import build

# Catégories YouTube utiles (ID officiels)
YOUTUBE_CATEGORIES = {
    "autos": "2",
    "comedy": "23",
    "education": "27",
    "entertainment": "24",
    "howto": "26",
    "people": "22",
    "sports": "17",
    "gaming": "20",
    "music": "10",
}

# Mapping simple thèmes -> catégories ou mots-clés
THEME_RULES = {
    "voiture": {"category_ids": [YOUTUBE_CATEGORIES["autos"]], "keywords": ["car", "voiture", "auto", "tuning"]},
    "drôle": {"category_ids": [YOUTUBE_CATEGORIES["comedy"]], "keywords": ["funny", "prank", "drôle", "humour"]},
    "business": {"category_ids": [YOUTUBE_CATEGORIES["education"], YOUTUBE_CATEGORIES["people"]], "keywords": ["business", "finance", "entrepreneur"]},
    "nourriture": {"category_ids": [YOUTUBE_CATEGORIES["howto"], YOUTUBE_CATEGORIES["people"]], "keywords": ["food", "recette", "cuisine", "street food"]},
    "influenceurs": {"category_ids": [YOUTUBE_CATEGORIES["entertainment"], YOUTUBE_CATEGORIES["people"]], "keywords": ["vlog", "influence", "storytime"]},
    "gaming": {"category_ids": [YOUTUBE_CATEGORIES["gaming"]], "keywords": ["game", "gaming", "let's play"]},
    "sport": {"category_ids": [YOUTUBE_CATEGORIES["sports"]], "keywords": ["match", "goal", "highlights"]},
    "musique": {"category_ids": [YOUTUBE_CATEGORIES["music"]], "keywords": ["official video", "lyrics", "clip"]},
}

class YouTubeClient:
    def __init__(self, api_key: str):
        self.api_key = api_key
        self.service = build("youtube", "v3", developerKey=api_key)

    def most_popular(self, region: str, max_results: int = 50) -> List[Dict[str, Any]]:
        req = self.service.videos().list(
            part="id,snippet,statistics,contentDetails",
            chart="mostPopular",
            regionCode=region,
            maxResults=max_results,
        )
        res = req.execute()
        return res.get("items", [])

    @staticmethod
    def parse_published_at(s: str) -> datetime | None:
        try:
            return datetime.fromisoformat(s.replace("Z", "+00:00")).astimezone(timezone.utc).replace(tzinfo=None)
        except Exception:
            return None
    """,
)

# ---------------------------
# app.py (fix: route parenthesis + safe compare)
# ---------------------------
add(
    "app.py",
    r"""
from __future__ import annotations
import hmac
from datetime import datetime, timedelta
from typing import List
from flask import Flask, render_template, redirect, request, url_for, jsonify, Response

from config import Config
from models import db, Video, Note, StatSnapshot
from youtube_client import YouTubeClient, THEME_RULES

app = Flask(__name__)
app.config.from_object(Config)

db.init_app(app)
with app.app_context():
    db.create_all()

yt = YouTubeClient(Config.YT_API_KEY) if Config.YT_API_KEY else None

# --- Simple Basic Auth ---

def _eq(a: str, b: str) -> bool:
    return hmac.compare_digest(a or "", b or "")


def check_auth(username, password):
    return _eq(username, Config.APP_USERNAME) and _eq(password, Config.APP_PASSWORD)


def authenticate():
    return Response(
        "Authentication required", 401, {"WWW-Authenticate": 'Basic realm="Login Required"'}
    )


def requires_auth(f):
    from functools import wraps

    @wraps(f)
    def decorated(*args, **kwargs):
        auth = request.authorization
        if not auth or not check_auth(auth.username, auth.password):
            return authenticate()
        return f(*args, **kwargs)

    return decorated


# --- Core logic ---

REGION_LANG = {"FR": "fr", "US": "en", "ES": "es"}
THEME_LIST = [t.strip() for t in Config.THEMES if t.strip()]

def _now():
    return datetime.utcnow()


def pick_theme_video(items, theme_name: str):
    rules = THEME_RULES.get(theme_name, {})
    cat_ids = set(rules.get("category_ids", []))
    keywords = [k.lower() for k in rules.get("keywords", [])]

    best = None
    best_score = -1.0

    for it in items:
        snip = it.get("snippet", {})
        stats = it.get("statistics", {})
        title = snip.get("title", "")
        cat = snip.get("categoryId")
        views = int(stats.get("viewCount", 0))
        published_at = snip.get("publishedAt")
        published_dt = YouTubeClient.parse_published_at(published_at) if published_at else None

        hours = max((_now() - published_dt).total_seconds() / 3600.0, 0.016) if published_dt else 1.0
        vph = views / hours

        title_l = title.lower()
        kw_match = any(k in title_l for k in keywords) if keywords else False
        cat_match = cat in cat_ids if cat_ids else False

        score = vph * (1.4 if (kw_match or cat_match) else 1.0)

        if score > best_score:
            best_score = score
            best = it

    return best


def compute_is_short(content_details) -> bool:
    dur = content_details.get("duration") if content_details else None
    if not dur:
        return False
    from isodate import parse_duration
    try:
        seconds = int(parse_duration(dur).total_seconds())
        return seconds <= 60
    except Exception:
        return False


@app.context_processor
def inject_now_and_next():
    now = _now()
    times = [Config.SCHEDULE_MORNING, Config.SCHEDULE_NOON, Config.SCHEDULE_EVENING]
    next_dt = None
    for t in times:
        h, m = [int(x) for x in t.split(":")]
        target = now.replace(hour=h, minute=m, second=0, microsecond=0)
        if target > now:
            next_dt = target
            break
    if not next_dt:
        h, m = [int(x) for x in Config.SCHEDULE_MORNING.split(":")]
        next_dt = (now + timedelta(days=1)).replace(hour=h, minute=m, second=0, microsecond=0)

    return {"server_now": now, "next_refresh": next_dt}


@app.route("/")
@requires_auth
def index():
    videos = Video.query.filter_by(used=False).order_by(Video.created_at.desc()).limit(5).all()
    return render_template("index.html", videos=videos, themes=THEME_LIST)


@app.route("/history")
@requires_auth
def history():
    videos = Video.query.filter_by(used=True).order_by(Video.created_at.desc()).all()
    last_snaps = {}
    for v in videos:
        snap = (
            StatSnapshot.query.filter_by(video_id=v.id)
            .order_by(StatSnapshot.captured_at.desc())
            .first()
        )
        last_snaps[v.id] = snap
    return render_template("history.html", videos=videos, snaps=last_snaps)


@app.route("/use/<int:vid>", methods=["POST"])  # FIX: retiré parenthèse en trop
@requires_auth
def mark_used(vid):
    v = Video.query.get_or_404(vid)
    v.used = True
    db.session.commit()
    return redirect(url_for("index"))


@app.route("/note/<int:vid>", methods=["POST"]) 
@requires_auth
def add_note(vid):
    content = request.form.get("content", "").strip()
    if content:
        n = Note(video_id=vid, content=content)
        db.session.add(n)
        db.session.commit()
    return redirect(url_for("history"))


@app.route("/api/refresh", methods=["POST"]) 
@requires_auth
def api_refresh():
    count = refresh_trending()
    return jsonify({"status": "ok", "added": count})


@app.route("/api/stats-refresh", methods=["POST"]) 
@requires_auth
def api_stats_refresh():
    refresh_stats()
    return jsonify({"status": "ok"})


# --- Worker functions (used by scheduler & API) ---

def refresh_trending() -> int:
    if yt is None:
        return 0

    added = 0
    items_all: List[dict] = []
    for region in Config.REGIONS:
        try:
            items = yt.most_popular(region=region, max_results=50)
            for it in items:
                it["_region"] = region
            items_all.extend(items)
        except Exception as e:
            print("YouTube API error:", e)

    for theme in THEME_LIST:
        best = pick_theme_video(items_all, theme)
        if not best:
            continue
        yt_id = best["id"]
        if Video.query.filter_by(yt_id=yt_id).first():
            continue

        snip = best.get("snippet", {})
        stats = best.get("statistics", {})
        cont = best.get("contentDetails", {})
        region = best.get("_region")

        v = Video(
            yt_id=yt_id,
            title=snip.get("title", ""),
            channel=snip.get("channelTitle"),
            url=f"https://www.youtube.com/watch?v={yt_id}",
            thumbnail=(snip.get("thumbnails", {}).get("medium", {}).get("url")
                       or snip.get("thumbnails", {}).get("default", {}).get("url")),
            category_id=snip.get("categoryId"),
            region=region,
            language=REGION_LANG.get(region, ""),
            published_at=YouTubeClient.parse_published_at(snip.get("publishedAt")),
            views_initial=int(stats.get("viewCount", 0)),
            likes_initial=int(stats.get("likeCount", 0)) if stats.get("likeCount") else 0,
            views_current=int(stats.get("viewCount", 0)),
            likes_current=int(stats.get("likeCount", 0)) if stats.get("likeCount") else 0,
            is_short=compute_is_short(cont),
        )
        db.session.add(v)
        db.session.commit()
        snap = StatSnapshot(video_id=v.id, views=v.views_current, likes=v.likes_current)
        db.session.add(snap)
        db.session.commit()
        added += 1

        if added >= 5:
            break

    print(f"[refresh_trending] Added {added} videos at {datetime.utcnow()} UTC")
    return added


def refresh_stats():
    if yt is None:
        return
    vids = Video.query.all()
    for i in range(0, len(vids), 50):
        chunk = vids[i:i+50]
        ids = [v.yt_id for v in chunk]
        try:
            res = yt.service.videos().list(part="id,statistics", id=",".join(ids)).execute()
            by_id = {it["id"]: it for it in res.get("items", [])}
            for v in chunk:
                it = by_id.get(v.yt_id)
                if not it:
                    continue
                stats = it.get("statistics", {})
                v.views_current = int(stats.get("viewCount", v.views_current or 0))
                v.likes_current = int(stats.get("likeCount", v.likes_current or 0)) if stats.get("likeCount") else v.likes_current
                db.session.add(v)
                db.session.flush()
                db.session.add(StatSnapshot(video_id=v.id, views=v.views_current, likes=v.likes_current))
            db.session.commit()
        except Exception as e:
            print("YouTube stats refresh error:", e)


def flash_alert_scan():
    if yt is None:
        return
    try:
        items = []
        for region in Config.REGIONS:
            items.extend(yt.most_popular(region=region, max_results=25))
        threshold = 500_000.0
        best = None
        best_vph = 0.0
        for it in items:
            stats = it.get("statistics", {})
            snip = it.get("snippet", {})
            views = int(stats.get("viewCount", 0))
            pub = snip.get("publishedAt")
            dt = YouTubeClient.parse_published_at(pub) if pub else None
            if not dt:
                continue
            hours = max((_now() - dt).total_seconds()/3600.0, 0.016)
            vph = views / hours
            if vph > threshold and vph > best_vph:
                best_vph = vph
                best = it
        if best:
            yt_id = best["id"]
            if not Video.query.filter_by(yt_id=yt_id).first():
                snip = best.get("snippet", {})
                stats = best.get("statistics", {})
                cont = best.get("contentDetails", {})
                v = Video(
                    yt_id=yt_id,
                    title=snip.get("title", ""),
                    channel=snip.get("channelTitle"),
                    url=f"https://www.youtube.com/watch?v={yt_id}",
                    thumbnail=(snip.get("thumbnails", {}).get("medium", {}).get("url")
                               or snip.get("thumbnails", {}).get("default", {}).get("url")),
                    category_id=snip.get("categoryId"),
                    region="BONUS",
                    language="",
                    published_at=YouTubeClient.parse_published_at(snip.get("publishedAt")),
                    views_initial=int(stats.get("viewCount", 0)),
                    likes_initial=int(stats.get("likeCount", 0)) if stats.get("likeCount") else 0,
                    views_current=int(stats.get("viewCount", 0)),
                    likes_current=int(stats.get("likeCount", 0)) if stats.get("likeCount") else 0,
                    is_short=compute_is_short(cont),
                )
                db.session.add(v)
                db.session.commit()
                db.session.add(StatSnapshot(video_id=v.id, views=v.views_current, likes=v.likes_current))
                db.session.commit()
                print(f"[flash_alert] Added FLASH video {yt_id} at VPH={best_vph:.0f}")
    except Exception as e:
        print("flash_alert_scan error:", e)


if __name__ == "__main__":
    # Lancement en dev
    from scheduler import start_jobs
    start_jobs()
    app.run(host="0.0.0.0", port=4443, debug=True)
    """,
)

# ---------------------------
# scheduler.py
# ---------------------------
add(
    "scheduler.py",
    r"""
from apscheduler.schedulers.background import BackgroundScheduler
from config import Config
from app import refresh_trending, refresh_stats, flash_alert_scan

scheduler = BackgroundScheduler(timezone="UTC")


def _parse_hhmm(hhmm: str):
    h, m = [int(x) for x in hhmm.split(":")]
    return h, m


def start_jobs():
    h, m = _parse_hhmm(Config.SCHEDULE_MORNING)
    scheduler.add_job(refresh_trending, "cron", hour=h, minute=m, id="refresh_morning")

    h, m = _parse_hhmm(Config.SCHEDULE_NOON)
    scheduler.add_job(refresh_trending, "cron", hour=h, minute=m, id="refresh_noon")

    h, m = _parse_hhmm(Config.SCHEDULE_EVENING)
    scheduler.add_job(refresh_trending, "cron", hour=h, minute=m, id="refresh_evening")

    scheduler.add_job(refresh_stats, "interval", minutes=60, id="refresh_stats")
    scheduler.add_job(flash_alert_scan, "interval", minutes=Config.ALERT_INTERVAL_MIN, id="flash_alert")

    scheduler.start()
    """,
)

# ---------------------------
# templates
# ---------------------------
add(
    "templates/base.html",
    r"""
<!doctype html>
<html lang="fr">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Veille YouTube & TikTok</title>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/css/bootstrap.min.css">
  <link rel="stylesheet" href="{{ url_for('static', filename='styles.css') }}">
</head>
<body class="bg-pastel">
<nav class="navbar navbar-expand-lg navbar-dark bg-gradient p-3">
  <div class="container-fluid">
    <a class="navbar-brand fw-bold" href="/">📈 Veille Trends</a>
    <div>
      <a class="btn btn-sm btn-light" href="/history">Historique</a>
    </div>
  </div>
</nav>
<main class="container py-4">
  {% block content %}{% endblock %}
</main>
<script>
(function(){
  const nextRefreshTs = new Date("{{ next_refresh.isoformat() }}Z").getTime();
  const el = document.getElementById('countdown');
  if(!el) return;
  setInterval(()=>{
    const now = new Date().getTime();
    let diff = Math.max(0, nextRefreshTs - now);
    const h = Math.floor(diff/3600000); diff%=3600000;
    const m = Math.floor(diff/60000); diff%=60000;
    const s = Math.floor(diff/1000);
    el.textContent = `${String(h).padStart(2,'0')}h ${String(m).padStart(2,'0')}m ${String(s).padStart(2,'0')}s`;
  }, 1000);
})();
</script>
</body>
</html>
    """,
)

add(
    "templates/index.html",
    r"""
{% extends 'base.html' %}
{% block content %}
<div class="d-flex justify-content-between align-items-center mb-3">
  <h3 class="mb-0">Sélection actuelle</h3>
  <div class="text-end">
    <div class="small text-muted">Prochain refresh dans</div>
    <div id="countdown" class="fw-semibold h5 mb-0">--</div>
  </div>
</div>

{% if videos|length == 0 %}
  <div class="alert alert-info">Aucune vidéo pour le moment.
    <form class="d-inline" method="post" action="/api/refresh">
      <button class="btn btn-primary btn-sm">Rafraîchir maintenant</button>
    </form>
  </div>
{% endif %}

<div class="row g-3">
  {% for v in videos %}
  <div class="col-md-6">
    <div class="card shadow-sm border-0">
      <div class="row g-0">
        <div class="col-4">
          <a href="{{ v.url }}" target="_blank"><img src="{{ v.thumbnail }}" class="img-fluid rounded-start" alt="thumb"></a>
        </div>
        <div class="col-8">
          <div class="card-body">
            <a href="{{ v.url }}" target="_blank" class="stretched-link text-decoration-none"><h6 class="card-title">{{ v.title }}</h6></a>
            <div class="text-muted small">{{ v.channel }} • {{ v.region }} {% if v.is_short %}<span class="badge text-bg-warning ms-1">Short</span>{% endif %}</div>
            <div class="mt-2">
              <span class="badge text-bg-primary">{{ '{:,}'.format(v.views_current).replace(',', ' ') }} vues</span>
              {% set vph = '%.0f' % v.views_per_hour() %}
              <span class="badge text-bg-success">🚀 {{ vph }} vues/h</span>
            </div>
            <form class="mt-2" method="post" action="/use/{{ v.id }}">
              <button class="btn btn-sm btn-outline-secondary">Marquer comme utilisée</button>
            </form>
          </div>
        </div>
      </div>
    </div>
  </div>
  {% endfor %}
</div>
{% endblock %}
    """,
)

add(
    "templates/history.html",
    r"""
{% extends 'base.html' %}
{% block content %}
<h3>Historique</h3>
<div class="row g-3">
  {% for v in videos %}
  <div class="col-md-6">
    <div class="card shadow-sm border-0">
      <div class="card-body">
        <div class="d-flex align-items-start">
          <img src="{{ v.thumbnail }}" width="96" height="54" class="rounded me-3" alt="thumb">
          <div class="flex-grow-1">
            <a href="{{ v.url }}" target="_blank" class="text-decoration-none"><h6 class="mb-1">{{ v.title }}</h6></a>
            <div class="small text-muted">{{ v.channel }} • {{ v.region }}</div>
            {% set latest = snaps.get(v.id) %}
            <div class="mt-2">
              <span class="badge text-bg-primary">Initial: {{ '{:,}'.format(v.views_initial).replace(',', ' ') }}</span>
              <span class="badge text-bg-info">Actuel: {{ '{:,}'.format(v.views_current).replace(',', ' ') }}</span>
              {% if latest %}
                {% set hours = ((latest.captured_at - v.created_at).total_seconds() / 3600.0) | round(1) %}
                <span class="badge text-bg-success">Δ {{ (v.views_current - v.views_initial) | int }} vues en {{ hours }}h</span>
              {% endif %}
            </div>
          </div>
        </div>
        <form class="mt-3" method="post" action="/note/{{ v.id }}">
          <div class="input-group">
            <input type="text" name="content" class="form-control" placeholder="Ajouter une note (idée TikTok, résultat, etc.)">
            <button class="btn btn-outline-primary">Ajouter</button>
          </div>
        </form>
      </div>
    </div>
  </div>
  {% endfor %}
</div>
{% endblock %}
    """,
)

# ---------------------------
# static
# ---------------------------
add(
    "static/styles.css",
    r"""
:root{
  --pastel-bg: #f3f5ff;
  --grad-a: #7a8cff;
  --grad-b: #b18cff;
}
.bg-pastel{ background: var(--pastel-bg); }
.bg-gradient{ background: linear-gradient(90deg, var(--grad-a), var(--grad-b)); }
.card{ border-radius: 16px; }
img{ object-fit: cover; }
    """,
)

# ---------------------------
# Dockerfile
# ---------------------------
add(
    "Dockerfile",
    r"""
FROM python:3.11-slim

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 4443

CMD ["gunicorn", "-w", "2", "-b", "0.0.0.0:4443", "app:app"]
    """,
)

# ---------------------------
# tests (ajout de tests unitaires)
# ---------------------------
add(
    "tests/test_core.py",
    r"""
import math
from datetime import datetime, timedelta

from models import Video


def test_views_per_hour_zero_when_missing_data():
    v = Video(views_current=0, published_at=None)
    assert v.views_per_hour() == 0.0


def test_views_per_hour_positive():
    # 3600 vues en 1h => 3600 v/h
    v = Video(views_current=3600, published_at=datetime.utcnow() - timedelta(hours=1))
    assert 3590 <= v.views_per_hour() <= 3610


def test_schedule_env_format_example():
    # Juste un test de format attendu HH:MM
    def is_hhmm(s):
        try:
            h, m = [int(x) for x in s.split(":")]
            return 0 <= h < 24 and 0 <= m < 60
        except Exception:
            return False
    assert is_hhmm("08:00")
    assert is_hhmm("13:00")
    assert is_hhmm("19:00")
    """,
)

# ---------------------------
# Écriture des fichiers
# ---------------------------
for path, content in FILES.items():
    full = ROOT / path
    full.parent.mkdir(parents=True, exist_ok=True)
    full.write_text(content, encoding="utf-8")

print("✅ Projet généré.")
print("➡️ Étapes :\n  1) cp .env.example .env && nano .env  (mets YOUTUBE_API_KEY)\n  2) docker build -t veille-tiktok . && docker run -d --name veille-tiktok -p 4443:4443 --env-file .env veille-tiktok\n  3) http://localhost:4443 (Basic Auth)")
