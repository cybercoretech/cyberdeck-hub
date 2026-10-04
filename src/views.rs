//! Plain string-built HTML — no template engine dependency, matching the
//! minimal-deps approach used across the rest of this ecosystem (same
//! convention as Dockspace's `views.rs`).

use crate::hub::{Checked, Health, Probe};
use crate::present::folder_color;
use crate::routes::ReportEntry;

pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn layout(
    active_theme: &str,
    appearance: &str,
    theme_options: &str,
    nav_active: &str,
    body: &str,
) -> String {
    let nav = |href: &str, label: &str| {
        let active = if nav_active == label { " active" } else { "" };
        format!(r#"<a href="{href}" class="btn-ghost{active}">{label}</a>"#)
    };
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>CYBERDECK</title>
<link rel="icon" type="image/svg+xml" href="/static/favicon.svg">
<link rel="stylesheet" href="/vendor/tokens.css">
<style id="theme-vars"></style>
<link rel="stylesheet" href="/static/ui.css">
<script src="/static/htmx.min.js" defer></script>
</head>
<body>
<div class="bg-scanlines" aria-hidden="true"></div>
<div class="bg-glow" aria-hidden="true"></div>
<header class="topbar">
  <span class="brand">CYBERDECK<span class="brand-accent">//</span></span>
  <div class="topbar-right">
    <nav class="topbar-nav">
      {nav_hub}
      {nav_scans}
      {nav_reports}
    </nav>
      <div class="theme-dropdown-wrap">
      <button type="button" class="btn-ghost" id="theme-picker-btn" onclick="toggleDropdown(event, 'theme-dropdown')" aria-haspopup="listbox" aria-expanded="false">
        <span id="theme-picker-label">THEME</span> <span class="caret-down">&#9662;</span>
      </button>
      <div id="theme-dropdown" class="theme-dropdown" role="listbox" hidden>
        {theme_options}
        <div id="custom-theme-group" hidden></div>
      </div>
      </div>
      <button type="button" class="btn-ghost" id="theme-appearance" onclick="toggleAppearance()">{appearance} mode</button>
      <button class="btn-ghost" onclick="openThemeCreator()" title="Create and save a shared Cybercore theme">+ Theme</button>
  </div>
</header>
<main>
{body}
</main>
<div id="theme-creator-overlay" class="viewer-overlay" hidden onclick="if(event.target===this) closeThemeCreator()">
  <div class="window viewer-window ct-window">
    <div class="titlebar">
      <span class="dot dot-a"></span><span class="dot dot-b"></span><span class="dot dot-c"></span>
      <span class="titlebar-text">custom CYBERGRID theme</span>
      <a href="javascript:void(0)" class="win-close" onclick="closeThemeCreator()" title="Close">&#10005;</a>
    </div>
    <div class="viewer-body">
      <div class="ct-layout">
        <div class="ct-controls">
          <p class="muted" style="margin-top:0;">Create a shared theme for Cybercore apps. Save it to your user theme library or export a portable theme file.</p>
          <label class="field-label" for="ct-name">Name</label>
          <input id="ct-name" class="text-input" placeholder="my-custom-theme" style="width:100%;margin-bottom:0.75rem;">
          <label class="field-label" for="ct-family">Family</label>
          <input id="ct-family" class="text-input" value="custom" maxlength="96">
          <label class="field-label" for="ct-description">Description</label>
          <input id="ct-description" class="text-input" maxlength="256">
          <label class="field-label" for="ct-author">Author</label>
          <input id="ct-author" class="text-input" maxlength="96" style="margin-bottom:0.75rem;">
          <div class="card-actions" style="margin-bottom:0.75rem;">
            <button type="button" onclick="document.getElementById('ct-file-input').click()">Import from file&hellip;</button>
            <button type="button" onclick="exportCustomTheme()">Export to file</button>
            <button type="button" onclick="resetSwatchesToActive()">Reset to active</button>
            <input type="file" id="ct-file-input" accept=".json,application/json" hidden onchange="loadThemeFromFile(event)">
          </div>
          <label class="field-label" for="ct-mode">Appearance</label>
          <select id="ct-mode" class="text-input" onchange="changeThemeMode()"><option value="dark">Dark variant</option><option value="light">Light variant</option></select>
          <label class="field-label" for="ct-font-ui">UI font</label>
          <input id="ct-font-ui" class="text-input" value="Inter" oninput="previewCustomTheme()">
          <label class="field-label" for="ct-font-mono">Monospace font</label>
          <input id="ct-font-mono" class="text-input" value="JetBrains Mono" oninput="previewCustomTheme()">
          <label class="field-label" for="ct-font-display">Display font</label>
          <input id="ct-font-display" class="text-input" value="Oxanium" oninput="previewCustomTheme()">
          <label class="field-label" for="ct-density">Density</label>
          <select id="ct-density" class="text-input" onchange="previewCustomTheme()"><option>compact</option><option selected>comfortable</option><option>spacious</option></select>
          <label class="field-label" for="ct-corners">Corners</label>
          <select id="ct-corners" class="text-input" onchange="previewCustomTheme()"><option>sharp</option><option>soft</option><option>rounded</option></select>
          <label class="field-label" for="ct-motion">Motion</label>
          <select id="ct-motion" class="text-input" onchange="previewCustomTheme()"><option>full</option><option>reduced</option><option>none</option></select>
          <p id="ct-contrast" class="muted" role="status" aria-live="polite"></p>
          <div id="ct-swatches" class="ct-grid"></div>
        </div>
        <div class="ct-preview">
          <p class="field-label" style="margin-bottom:0.6rem;">Live preview</p>
          <div class="dash-head" style="margin-bottom:0.9rem;">
            <h1>Sample</h1>
            <p class="muted">This pane, and everything below it, repaints as you edit.</p>
          </div>
          <div class="card" style="margin-bottom:0.9rem;">
            <div class="card-head">
              <span class="card-title">Sample Tool</span>
              <span class="badge status-running">ONLINE</span>
            </div>
            <p class="card-desc">A card looks like this under your theme.</p>
            <p class="card-meta"><code>~/tools/example</code></p>
            <div class="card-actions">
              <button type="button">Run</button>
              <a class="btn-link" href="javascript:void(0)">View</a>
            </div>
          </div>
          <div style="display:flex; gap:0.4rem; flex-wrap:wrap; margin-bottom:0.9rem;">
            <span class="folder-chip" style="--chip-color:var(--cyan);">cyan</span>
            <span class="folder-chip" style="--chip-color:var(--purple);">purple</span>
            <span class="folder-chip" style="--chip-color:var(--orange);">orange</span>
            <span class="folder-chip-root">root</span>
          </div>
          <div class="markdown-body" style="max-height:none;">
            <h2>Heading</h2>
            <p>Body text with <code>inline code</code> and a <a href="javascript:void(0)">link</a>.</p>
          </div>
        </div>
      </div>
      <div class="viewer-actions">
        <button onclick="saveCustomTheme()">Save</button>
        <button onclick="closeThemeCreator()">Close</button>
      </div>
    </div>
  </div>
</div>
<div id="viewer-overlay" class="viewer-overlay" hidden onclick="if(event.target===this) closeViewer()">
  <div class="window viewer-window">
    <div class="titlebar">
      <span class="dot dot-a"></span><span class="dot dot-b"></span><span class="dot dot-c"></span>
      <span class="titlebar-text" id="viewer-title">report</span>
      <a href="javascript:void(0)" class="win-close" onclick="closeViewer()" title="Close">&#10005;</a>
    </div>
    <div class="viewer-body">
      <div id="viewer-content" class="markdown-body">loading…</div>
      <div class="viewer-actions">
        <a id="viewer-fullpage" class="btn-ghost" href="/reports" target="_blank" rel="noopener">Open Full Page</a>
        <button onclick="closeViewer()">Close</button>
      </div>
    </div>
  </div>
</div>
<script>
let ACTIVE_APPEARANCE = '{appearance}';
function applyTheme(name) {{
  if (name.startsWith('custom:')) {{
    const css = localStorage.getItem(CT_PREFIX + name.slice(7));
    if (css) document.getElementById('theme-vars').textContent = css;
  }} else {{
    fetch('/api/cybergrid/css/' + encodeURIComponent(name) + '?appearance=' + ACTIVE_APPEARANCE)
      .then(r => {{ if (!r.ok) throw new Error('Theme unavailable'); return r.text(); }})
      .then(css => {{ document.getElementById('theme-vars').textContent = css; }})
      .catch(error => console.error(error));
    fetch('/api/cybergrid/active/' + encodeURIComponent(name), {{ method: 'POST' }})
      .catch(error => console.error('Could not save active theme', error));
  }}
  document.getElementById('theme-picker-label').textContent = name.startsWith('custom:') ? name.slice(7) : name;
  document.querySelectorAll('#theme-dropdown .theme-item').forEach(el => {{
    el.classList.toggle('selected', el.dataset.value === name);
  }});
}}
function selectTheme(name) {{
  applyTheme(name);
  closeAllDropdowns();
}}
async function toggleAppearance() {{
  const next = ACTIVE_APPEARANCE === 'dark' ? 'light' : 'dark';
  const response = await fetch('/api/cybergrid/appearance/' + next, {{method: 'POST'}});
  if (!response.ok) {{ alert('Could not switch appearance.'); return; }}
  ACTIVE_APPEARANCE = next;
  document.getElementById('theme-appearance').textContent = next + ' mode';
  const selected = document.querySelector('#theme-dropdown .theme-item.selected')?.dataset.value || '{active_theme}';
  fetch('/api/cybergrid/css/' + encodeURIComponent(selected) + '?appearance=' + next)
    .then(r => {{ if (!r.ok) throw new Error('Theme unavailable'); return r.text(); }})
    .then(css => {{ document.getElementById('theme-vars').textContent = css; }});
}}
// Generic custom dropdown (theme picker, archive format, and any future
// one) — a native <select>'s selected/hovered option always gets forced
// system-blue with zero author override in Chromium, so every picker in
// this app uses this instead.
function toggleDropdown(e, ddId) {{
  e.stopPropagation();
  const dd = document.getElementById(ddId);
  const wasHidden = dd.hidden;
  closeAllDropdowns();
  if (!wasHidden) {{ e.currentTarget.setAttribute('aria-expanded', 'false'); return; }}
  // position: fixed (see .theme-dropdown in ui.css for why) means this
  // has to be placed in real screen coordinates by hand — it no longer
  // auto-anchors to its wrapper the way an absolutely-positioned one did.
  const rect = e.currentTarget.getBoundingClientRect();
  const width = dd.offsetWidth || 220;
  const left = Math.min(rect.left, window.innerWidth - width - 8);
  dd.style.top = (rect.bottom + 6) + 'px';
  dd.style.left = Math.max(8, left) + 'px';
  dd.hidden = false;
  e.currentTarget.setAttribute('aria-expanded', 'true');
}}
function closeAllDropdowns() {{
  document.querySelectorAll('.theme-dropdown').forEach(dd => {{ dd.hidden = true; }});
  document.querySelectorAll('[aria-haspopup="listbox"]').forEach(b => b.setAttribute('aria-expanded', 'false'));
}}
document.addEventListener('click', (e) => {{
  if (!e.target.closest('.theme-dropdown-wrap')) closeAllDropdowns();
}});
document.addEventListener('keydown', (e) => {{
  if (e.key === 'Escape') closeAllDropdowns();
}});

window.addEventListener('DOMContentLoaded', () => {{
  loadCustomThemesIntoPicker();
  const active = document.querySelector('#theme-dropdown .theme-item.selected')?.dataset.value || '{active_theme}';
  applyTheme(active);
  restoreCatState();
}});

// Hub category sections are collapsible <details> — but the whole
// #hub-grid gets replaced every 10s by the htmx status poll, which would
// silently re-expand anything the user just collapsed. Persist per-
// category state in localStorage and re-apply it after every swap.
function saveCatState(details) {{
  localStorage.setItem('cyberdeck-cat-' + details.dataset.cat, details.open ? '1' : '0');
}}
function restoreCatState() {{
  document.querySelectorAll('details.cat-section[data-cat]').forEach(d => {{
    const saved = localStorage.getItem('cyberdeck-cat-' + d.dataset.cat);
    if (saved !== null) d.open = saved === '1';
  }});
}}

function openViewer(path) {{
  const overlay = document.getElementById('viewer-overlay');
  const content = document.getElementById('viewer-content');
  document.getElementById('viewer-title').textContent = path;
  document.getElementById('viewer-fullpage').href = '/reports/' + path;
  content.innerHTML = 'loading…';
  overlay.hidden = false;
  fetch('/api/reports/' + path)
    .then(r => {{
      const title = r.headers.get('X-Report-Title');
      if (title) document.getElementById('viewer-title').textContent = title;
      return r.text();
    }})
    .then(html => {{ content.innerHTML = html; }})
    .catch(() => {{ content.textContent = 'failed to load'; }});
}}
function closeViewer() {{
  document.getElementById('viewer-overlay').hidden = true;
}}
document.addEventListener('keydown', e => {{
  if (e.key === 'Escape') {{ closeViewer(); closeThemeCreator(); }}
}});

// --- Shared CYBERGRID theme creator. Drafts preview locally; saved
// documents and the selected theme are persisted by the shared engine. ---
const CT_ROLES = ['bg', 'fg', 'acid', 'pink', 'purple', 'cyan', 'orange', 'red', 'panel', 'line', 'muted'];
const CT_PREFIX = 'cyberdeck-custom-theme-';

function openThemeCreator() {{
  CT_PREVIOUS_THEME = document.querySelector('#theme-dropdown .theme-item.selected')?.dataset.value || '{active_theme}';
  CT_MODE = 'dark'; CT_VARIANTS = {{}};
  document.getElementById('ct-name').value = '';
  document.getElementById('ct-family').value = 'custom';
  document.getElementById('ct-description').value = '';
  document.getElementById('ct-author').value = '';
  document.getElementById('ct-mode').value = 'dark';
  const grid = document.getElementById('ct-swatches');
  const computed = getComputedStyle(document.documentElement);
  grid.innerHTML = CT_ROLES.map(role => {{
    const current = computed.getPropertyValue('--' + role).trim() || '#888888';
    return '<label class="ct-swatch">' + role +
      '<input type="color" id="ct-' + role + '" value="' + current + '" oninput="previewCustomTheme()"></label>';
  }}).join('');
  previewCustomTheme();
  document.getElementById('theme-creator-overlay').hidden = false;
}}
function closeThemeCreator() {{
  const el = document.getElementById('theme-creator-overlay');
  if (el) el.hidden = true;
  // Revert to whatever was actually saved/active — closing without Save
  // shouldn't leave an unsaved preview painted over the real theme.
  applyTheme(CT_PREVIOUS_THEME);
}}
function customThemeCss() {{
  const doc = customThemeDocument(), palette = CT_MODE === 'light' ? (doc.variants.light || doc.palette) : doc.palette;
  const safeFont = value => /^[A-Za-z0-9 _-]{{1,64}}$/.test(value) ? value : 'Inter';
  const vars = {{'--bg': palette.bg, '--fg': palette.white, '--acid': palette.acid_green, '--pink': palette.hot_pink,
    '--purple': palette.purple, '--cyan': palette.cyan, '--orange': palette.orange, '--red': palette.red,
    '--panel': palette.panel, '--line': palette.line, '--muted': palette.muted,
    '--font-ui': '"' + safeFont(doc.design.font_ui) + '",sans-serif', '--font-mono': '"' + safeFont(doc.design.font_mono) + '",monospace',
    '--font-display': '"' + safeFont(doc.design.font_display) + '",sans-serif', '--density': {{compact: '0.85', comfortable: '1', spacious: '1.15'}}[doc.design.density],
    '--radius': {{sharp: '2px', soft: '8px', rounded: '14px'}}[doc.design.corners],
    '--dur-mid': {{full: '0.24s', reduced: '0.08s', none: '0s'}}[doc.design.motion]}};
  const colors = ['--bg','--fg','--acid','--pink','--purple','--cyan','--orange','--red','--panel','--line','--muted'];
  return ':root{{' + Object.entries(vars).map(([key,value]) => key + ':' + (colors.includes(key) ? '#' + value.replace('#','') : value) + ';').join('') + '}}';
}}
async function previewCustomTheme() {{
  document.getElementById('theme-vars').textContent = customThemeCss();
  try {{
    const response = await fetch('/api/cybergrid/validate', {{method: 'POST', headers: {{'Content-Type': 'application/json'}}, body: JSON.stringify(customThemeDocument())}});
    const report = await response.json();
    const contrast = CT_MODE === 'light' ? (report.light_contrast || report.contrast) : report.contrast;
    const failed = (contrast?.checks || []).filter(check => !check.passes);
    document.getElementById('ct-contrast').textContent = report.error || (failed.length ? 'Contrast needs attention: ' + failed.map(check => check.label + ' (' + check.ratio.toFixed(2) + ':1)').join(', ') : 'Contrast checks pass for tested text and accent pairs.');
  }} catch (_) {{ document.getElementById('ct-contrast').textContent = 'Contrast check unavailable.'; }}
}}
// Accepts the exact same JSON shape as a real cybercore theme file
// (schema/themes/<family>/<slug>.json — bare hex, no leading '#', field
// names bg/white/acid_green/hot_pink/purple/cyan/orange/red/panel/line/
// muted) so any of the 72 shipped themes, or a hand-authored file
// following that schema, can be dropped in directly.
const CT_FILE_KEY_MAP = {{
  bg: 'bg', white: 'fg', acid_green: 'acid', hot_pink: 'pink', purple: 'purple',
  cyan: 'cyan', orange: 'orange', red: 'red', panel: 'panel', line: 'line', muted: 'muted'
}};
let CT_VARIANTS = {{}};
let CT_MODE = 'dark';
let CT_PREVIOUS_THEME = '{active_theme}';

function currentPalette() {{
  const palette = {{}};
  Object.entries(CT_FILE_KEY_MAP).forEach(([jsonKey, role]) => {{ palette[jsonKey] = document.getElementById('ct-' + role).value.replace('#', ''); }});
  return palette;
}}
function showPalette(palette) {{
  Object.entries(CT_FILE_KEY_MAP).forEach(([jsonKey, role]) => {{
    const input = document.getElementById('ct-' + role);
    if (input) input.value = '#' + (palette?.[jsonKey] || '888888').replace('#', '');
  }});
}}
function changeThemeMode() {{
  CT_VARIANTS[CT_MODE] = currentPalette();
  CT_MODE = document.getElementById('ct-mode').value;
  showPalette(CT_MODE === 'light' ? CT_VARIANTS.light : CT_VARIANTS.dark);
  previewCustomTheme();
}}
function designValues() {{
  return {{font_ui: document.getElementById('ct-font-ui').value, font_mono: document.getElementById('ct-font-mono').value,
    font_display: document.getElementById('ct-font-display').value, density: document.getElementById('ct-density').value,
    corners: document.getElementById('ct-corners').value, motion: document.getElementById('ct-motion').value}};
}}
function customThemeDocument() {{
  CT_VARIANTS[CT_MODE] = currentPalette();
  const name = document.getElementById('ct-name').value.trim() || 'Untitled theme';
  const id = name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '').slice(0, 64) || 'custom-theme';
  const dark = CT_VARIANTS.dark || CT_VARIANTS.light || currentPalette();
  const variants = CT_VARIANTS.light ? {{light: CT_VARIANTS.light}} : {{}};
  return {{format_version: 1, metadata: {{id, name, family: document.getElementById('ct-family').value.trim() || 'custom', description: document.getElementById('ct-description').value, author: document.getElementById('ct-author').value}}, palette: dark, variants, design: designValues()}};
}}
function loadThemeFromFile(event) {{
  const file = event.target.files[0];
  if (!file) return;
  const reader = new FileReader();
  reader.onload = () => {{
    let data;
    try {{ data = JSON.parse(reader.result); }} catch (e) {{
      alert('Not valid JSON.'); return;
    }}
    const palette = data.palette || data;
    let applied = 0;
    Object.entries(CT_FILE_KEY_MAP).forEach(([jsonKey, role]) => {{
      const raw = palette[jsonKey];
      if (typeof raw !== 'string' || !raw) return;
      const hex = raw.startsWith('#') ? raw : '#' + raw;
      const input = document.getElementById('ct-' + role);
      if (input) {{ input.value = hex; applied++; }}
    }});
    if (!applied) {{
      alert('That JSON has none of the expected fields (bg, white, acid_green, hot_pink, purple, cyan, orange, red, panel, line, muted) — expected the cybercore palette shape.');
      return;
    }}
    CT_VARIANTS = data.variants && typeof data.variants === 'object' ? data.variants : {{}};
    if (data.palette) {{ CT_VARIANTS.dark = data.palette; CT_MODE = 'dark'; }}
    document.getElementById('ct-mode').value = CT_MODE;
    showPalette(CT_MODE === 'light' ? CT_VARIANTS.light : CT_VARIANTS.dark || palette);
    if (data.metadata) {{
      const metadata = data.metadata;
      document.getElementById('ct-name').value = data.metadata.name || data.metadata.id || file.name.replace(/\.json$/i, '');
      document.getElementById('ct-family').value = metadata.family || 'custom';
      document.getElementById('ct-description').value = metadata.description || '';
      document.getElementById('ct-author').value = metadata.author || '';
    }} else {{
      document.getElementById('ct-name').value = file.name.replace(/\.json$/i, '');
    }}
    if (data.design) {{
      for (const key of ['font_ui','font_mono','font_display']) {{
        if (typeof data.design[key] === 'string') document.getElementById('ct-' + key.replace('_', '-')).value = data.design[key];
      }}
      for (const key of ['density','corners','motion']) {{
        if (typeof data.design[key] === 'string') document.getElementById('ct-' + key).value = data.design[key];
      }}
    }}
    if (!document.getElementById('ct-name').value.trim()) {{
      document.getElementById('ct-name').value = file.name.replace(/\.json$/i, '');
    }}
    previewCustomTheme();
  }};
  reader.readAsText(file);
}}
function exportCustomTheme() {{
  const name = document.getElementById('ct-name').value.trim() || 'custom-theme';
  document.getElementById('ct-name').value = name;
  const data = customThemeDocument();
  const blob = new Blob([JSON.stringify(data, null, 2)], {{ type: 'application/json' }});
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url; a.download = name + '.json';
  document.body.appendChild(a); a.click(); document.body.removeChild(a);
  URL.revokeObjectURL(url);
}}
function resetSwatchesToActive() {{
  // Reads the *actually saved* theme, not whatever's currently painted —
  // this dialog may already be mid-preview of unsaved edits, and "reset"
  // should mean "back to what was really active before I opened this."
  applyTheme(CT_PREVIOUS_THEME);
  const computed = getComputedStyle(document.documentElement);
  CT_ROLES.forEach(role => {{
    const input = document.getElementById('ct-' + role);
    if (input) input.value = computed.getPropertyValue('--' + role).trim() || '#888888';
  }});
}}
async function saveCustomTheme() {{
  const name = document.getElementById('ct-name').value.trim();
  if (!name) {{ alert('Name it first.'); return; }}
  document.getElementById('ct-name').value = name;
  const response = await fetch('/api/cybergrid/themes', {{method: 'POST', headers: {{'Content-Type': 'application/json'}}, body: JSON.stringify(customThemeDocument())}});
  const result = await response.json();
  if (!response.ok) {{ alert(result.error || 'Could not save theme.'); return; }}
  await loadCustomThemesIntoPicker();
  CT_PREVIOUS_THEME = result.id;
  applyTheme(result.id);
  document.getElementById('theme-creator-overlay').hidden = true;
}}
async function loadCustomThemesIntoPicker() {{
  const group = document.getElementById('custom-theme-group');
  let themes = [];
  try {{ const response = await fetch('/api/cybergrid/themes'); if (response.ok) themes = (await response.json()).themes.filter(theme => !theme.builtin); }} catch (_) {{}}
  const keys = Object.keys(localStorage).filter(k => k.startsWith(CT_PREFIX));
  if (!themes.length && !keys.length) {{ group.hidden = true; group.innerHTML = ''; return; }}
  group.hidden = false;
  group.replaceChildren();
  const title = document.createElement('div'); title.className = 'theme-group-label'; title.textContent = 'Custom themes'; group.appendChild(title);
  for (const theme of themes) {{
    const item = document.createElement('div'); item.className = 'theme-item'; item.dataset.value = theme.id; item.textContent = theme.name;
    item.addEventListener('click', () => selectTheme(theme.id)); group.appendChild(item);
  }}
  for (const key of keys) {{
    const name = key.slice(CT_PREFIX.length), item = document.createElement('div');
    item.className = 'theme-item'; item.dataset.value = 'custom:' + name; item.textContent = name;
    item.addEventListener('click', () => selectTheme('custom:' + name)); group.appendChild(item);
  }}
}}
</script>
</body>
</html>"#,
        nav_hub = nav("/", "Hub"),
        nav_scans = nav("/scans", "Scans"),
        nav_reports = nav("/reports", "Reports"),
        appearance = appearance,
    )
}

fn status_class(h: Health) -> &'static str {
    match h {
        Health::Up => "status-running",
        Health::Down => "status-stopped",
        Health::Unknown => "status-unknown",
    }
}

fn status_label(probe: &Probe, h: Health) -> &'static str {
    match (probe, h) {
        (Probe::Port(_), Health::Up) => "ONLINE",
        (Probe::Port(_), Health::Down) => "OFFLINE",
        (Probe::SystemUnit(_) | Probe::AnySystemUnit(_) | Probe::UserUnit(_), Health::Up) => {
            "ACTIVE"
        }
        (Probe::SystemUnit(_) | Probe::AnySystemUnit(_) | Probe::UserUnit(_), Health::Down) => {
            "INACTIVE"
        }
        (Probe::Binary(_), Health::Up) => "INSTALLED",
        (Probe::Binary(_), Health::Down) => "MISSING",
        (_, Health::Unknown) => "UNKNOWN",
    }
}

fn tool_card(c: &Checked) -> String {
    let t = c.tool;
    let cls = status_class(c.health);
    let label = status_label(&t.probe, c.health);

    let mut actions = String::new();
    if let Some(url) = t.open_url {
        actions.push_str(&format!(
            r#"<a class="btn-link" href="{url}" target="_blank" rel="noopener">Open</a>"#
        ));
    }
    if let Some(path) = t.wiki_path {
        actions.push_str(&format!(
            r#"<a class="btn-link" href="http://127.0.0.1:3000/{path}" target="_blank" rel="noopener">Wiki</a>"#
        ));
    }
    if actions.is_empty() {
        actions
            .push_str(r#"<span class="muted" style="font-size:0.75rem;">no wiki page yet</span>"#);
    }

    format!(
        r#"<div class="card">
  <div class="card-head">
    <span class="card-title">{name}</span>
    <span class="badge {cls}">{label}</span>
  </div>
  <p class="card-desc">{desc}</p>
  <p class="card-meta"><code>{path}</code></p>
  <div class="card-actions">{actions}</div>
</div>"#,
        name = t.name,
        desc = escape(t.description),
        path = escape(&crate::hub::display_repo_path(t.repo_path)),
    )
}

/// The category sections alone — used both for the initial page render and
/// as the htmx partial swapped into `#hub-grid` every 10s.
pub fn hub_sections(checked: &[Checked]) -> String {
    // Preserve TOOLS' declared order for category-of-first-appearance,
    // rather than alphabetizing — the registry is already grouped
    // logically (hub apps, then daemons, network, security, ...).
    let mut categories: Vec<&str> = Vec::new();
    for c in checked {
        if !categories.contains(&c.tool.category) {
            categories.push(c.tool.category);
        }
    }

    // Cycled per category so each section reads as its own color-coded
    // group, folder-tree style — matches cybercore's own palette roles
    // rather than inventing new colors.
    const CAT_COLORS: &[&str] = &["acid", "cyan", "purple", "pink", "orange", "red", "muted"];

    categories
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let color = CAT_COLORS[i % CAT_COLORS.len()];
            let cards: String = checked
                .iter()
                .filter(|c| &c.tool.category == cat)
                .map(tool_card)
                .collect();
            let count = checked.iter().filter(|c| &c.tool.category == cat).count();
            format!(
                r#"<details class="cat-section" data-cat="{cat}" open ontoggle="saveCatState(this)">
  <summary class="cat-head" style="--cat-color: var(--{color});"><span class="cat-caret">&#9656;</span><h2>{cat}</h2><span class="cat-count">{count}</span></summary>
  <div class="grid">{cards}</div>
</details>"#
            )
        })
        .collect()
}

pub fn dashboard(checked: &[Checked]) -> String {
    let up = checked.iter().filter(|c| c.health == Health::Up).count();
    let total = checked.len();
    let sections = hub_sections(checked);

    format!(
        r#"<div class="dash-head">
  <h1>Tool Fleet</h1>
  <p class="muted">{up} / {total} reporting up — auto-refreshes every 10s.</p>
</div>
<div id="hub-grid" hx-get="/partial/hub" hx-trigger="every 10s" hx-swap="innerHTML" hx-on::after-swap="restoreCatState()">
{sections}
</div>"#
    )
}

/// (dispatcher command, card title, one-line description, id for DOM/JS,
/// output filename under `diagnostics/`).
pub const SCAN_MODULES: &[(&str, &str, &str, &str, &str)] = &[
    (
        "RunCpuModule",
        "CPU",
        "Model, core count, frequency, load.",
        "cpu",
        "cpu/cpu.md",
    ),
    (
        "RunHardwareModule",
        "Hardware Core",
        "Full hardware inventory sweep.",
        "hw",
        "hardware.md",
    ),
    (
        "RunPowerModule",
        "Power Draw",
        "AC/battery state, wattage via upower.",
        "pwr",
        "power.md",
    ),
    (
        "RunMemoryModule",
        "Memory Banks",
        "RAM total/used, per-module DIMM info.",
        "mem",
        "memory.md",
    ),
    (
        "RunStorageAiModule",
        "Storage AI",
        "Heuristic storage health pass.",
        "stg",
        "storage_ai.md",
    ),
    (
        "RunThermalModule",
        "Thermal Array",
        "Sensor temps, raw + parsed.",
        "thm",
        "thermal_ai.md",
    ),
    (
        "RunNetworkModule",
        "Network Nodes",
        "Interfaces, addresses, link state.",
        "net",
        "network.md",
    ),
    (
        "RunFanModule",
        "Fan Velocity",
        "Fan RPM / PWM readouts.",
        "fan",
        "fan.md",
    ),
    (
        "RunAudioModule",
        "Audio Sinks",
        "PipeWire graph + sink/source summary.",
        "aud",
        "audio.md",
    ),
    (
        "RunBiosModule",
        "BIOS Layers",
        "Firmware, board, CPU, chipset, kernel, power sub-reports.",
        "bios",
        "bios.md",
    ),
    (
        "RunDisksModule",
        "Disk Topology",
        "Block devices, sizes, per-device detail.",
        "disk",
        "disks.md",
    ),
    (
        "RunBatteryModule",
        "Battery Health",
        "Capacity, cycle count, condition.",
        "bat",
        "battery/battery.md",
    ),
    (
        "RunServicesModule",
        "System Daemons",
        "systemd unit inventory.",
        "svc",
        "services.md",
    ),
    (
        "RunMotherboardModule",
        "Mainboard Bus",
        "Board model, vendor, revision.",
        "mobo",
        "motherboard/motherboard.md",
    ),
    (
        "RunGpuModule",
        "GPU",
        "VGA/3D controllers, driver bound, NVIDIA live stats if present.",
        "gpu",
        "gpu.md",
    ),
    (
        "RunUsbModule",
        "USB Devices",
        "Every enumerated USB device, bus/vendor/product.",
        "usb",
        "usb.md",
    ),
    (
        "RunKernelModulesModule",
        "Kernel Modules",
        "Loaded modules, size, dependents — largest first.",
        "kmod",
        "kernel_modules.md",
    ),
    (
        "RunMountsModule",
        "Mounted Filesystems",
        "What's mounted where, plus usage — not just block devices.",
        "mnt",
        "mounts.md",
    ),
];

fn scan_card(cmd: &str, title: &str, desc: &str, id: &str, filename: &str, exists: bool) -> String {
    let view = if exists {
        format!(
            r#"<a class="btn-link" href="/reports/{filename}" onclick="event.preventDefault(); openViewer('{filename}')">View Output</a>"#
        )
    } else {
        r#"<span class="muted" style="font-size:0.75rem;">not run yet</span>"#.to_string()
    };
    format!(
        r#"<div class="card">
  <div class="card-head">
    <span class="card-title">{title}</span>
    <span class="badge {badge_cls}" id="badge-{id}">{badge_label}</span>
  </div>
  <p class="card-desc">{desc}</p>
  <p class="card-meta"><code>diagnostics/{filename}</code></p>
  <div class="card-actions">
    <button id="btn-{id}" onclick="runScan('{cmd}', '{id}', '{filename}')">Run</button>
    <span id="view-{id}">{view}</span>
  </div>
</div>"#,
        badge_cls = if exists {
            "status-running"
        } else {
            "status-stopped"
        },
        badge_label = if exists { "HAS OUTPUT" } else { "NO OUTPUT" },
    )
}

pub fn scans_page() -> String {
    let cards: String = SCAN_MODULES
        .iter()
        .map(|(cmd, title, desc, id, filename)| {
            let exists = std::path::Path::new("diagnostics").join(filename).is_file();
            scan_card(cmd, title, desc, id, filename, exists)
        })
        .collect();

    format!(
        r#"<div class="dash-head">
  <h1>Diagnostic Scans</h1>
  <p class="muted">Each scan shells out, prints live output server-side, and writes a markdown report to <code>diagnostics/</code> — see <a class="btn-ghost" style="padding:0.1rem 0.4rem;" href="/reports">Reports</a>.</p>
</div>
<div class="grid">{cards}</div>

<section class="cat-section">
  <div class="cat-head"><h2>Files</h2></div>
  <div class="window">
    <div class="titlebar"><span class="dot dot-a"></span><span class="dot dot-b"></span><span class="dot dot-c"></span><span class="titlebar-text">diagnostics/ file viewer</span></div>
    <div class="window-body">
      <div class="card-actions" style="margin-bottom:0.75rem;">
        <button onclick="loadFileList()">Refresh File List</button>
      </div>
      <table class="res-table">
        <thead><tr><th>Folder</th><th>Report</th><th>View</th></tr></thead>
        <tbody id="file-list-body">
        <tr><td><div class="empty-state">click "Refresh File List" to scan diagnostics/</div></td></tr>
      </tbody></table>
    </div>
  </div>
</section>

<section class="cat-section">
  <div class="cat-head"><h2>Archive</h2></div>
  <div class="window">
    <div class="titlebar"><span class="dot dot-a"></span><span class="dot dot-b"></span><span class="dot dot-c"></span><span class="titlebar-text">compress diagnostics/ output</span></div>
    <div class="window-body">
      <label class="field-label">Format</label>
      <div class="theme-dropdown-wrap" style="margin-bottom:0.75rem;">
        <button type="button" class="btn-ghost" id="archive-format-btn" data-value="zip"
                onclick="toggleDropdown(event, 'archive-format-dropdown')" aria-haspopup="listbox" aria-expanded="false">
          <span id="archive-format-label">.ZIP (standard)</span> <span class="caret-down">&#9662;</span>
        </button>
        <div id="archive-format-dropdown" class="theme-dropdown" role="listbox" hidden>
          <div class="theme-item selected" data-value="zip" onclick="selectArchiveFormat('zip', '.ZIP (standard)')">.ZIP (standard)</div>
          <div class="theme-item" data-value="tar" onclick="selectArchiveFormat('tar', '.TAR (uncompressed)')">.TAR (uncompressed)</div>
          <div class="theme-item" data-value="gzip" onclick="selectArchiveFormat('gzip', '.TAR.GZ (compressed)')">.TAR.GZ (compressed)</div>
          <div class="theme-item" data-value="7z" onclick="selectArchiveFormat('7z', '.7Z (high compression)')">.7Z (high compression)</div>
        </div>
      </div>
      <div class="card-actions">
        <button id="btn-archive" onclick="runArchive()">Compress</button>
      </div>
      <p id="archive-result" class="muted" style="margin-top:0.6rem;font-size:0.8rem;"></p>
    </div>
  </div>
</section>

<script>
function runScan(cmd, id, filename) {{
  const btn = document.getElementById('btn-' + id);
  const original = btn.innerText;
  btn.innerText = 'running…'; btn.disabled = true;
  fetch('/cyberdeck/api/command', {{
    method: 'POST',
    headers: {{'Content-Type': 'application/json'}},
    body: JSON.stringify({{ [cmd]: './diagnostics' }})
  }}).then(r => {{
    if (!r.ok) throw new Error('scan failed');
    return r.json();
  }}).then(() => {{
    btn.innerText = 'done ✓';
    document.getElementById('badge-' + id).textContent = 'HAS OUTPUT';
    document.getElementById('badge-' + id).className = 'badge status-running';
    document.getElementById('view-' + id).innerHTML =
      '<a class="btn-link" href="/reports/' + filename + '" onclick="event.preventDefault(); openViewer(\'' + filename + '\')">View Output</a>';
    setTimeout(() => {{ btn.innerText = original; btn.disabled = false; }}, 1200);
  }}).catch(() => {{
    btn.innerText = 'error'; btn.disabled = false;
  }});
}}

function loadFileList() {{
  const body = document.getElementById('file-list-body');
  body.innerHTML = '<tr><td>loading…</td></tr>';
  fetch('/cyberdeck/api/list').then(r => r.json()).then(files => {{
    if (!files.length) {{ body.innerHTML = '<tr><td><div class="empty-state">diagnostics/ is empty</div></td></tr>'; return; }}
    // Server already resolved each file's real title (its own `# Heading`,
    // not the raw filename) and folder — this just renders it, matching
    // the same folder-color hash used on the Reports page.
    body.innerHTML = files.map(f => {{
      const folder = f.folder
        ? '<span class="folder-chip" style="--chip-color: var(--' + folderColor(f.folder) + ');">' + f.folder + '</span>'
        : '<span class="folder-chip folder-chip-root">root</span>';
      const view = f.is_md
        ? '<a class="btn-link" href="/reports/' + f.path + '" onclick="event.preventDefault(); openViewer(\'' + f.path + '\')">View</a>'
        : '<span class="muted">binary</span>';
      return '<tr><td>' + folder + '</td><td>' + f.title +
        '<div class="muted" style="font-size:0.7rem;">' + f.path + '</div></td><td>' + view + '</td></tr>';
    }}).join('');
  }}).catch(() => {{ body.innerHTML = '<tr><td>failed to list files</td></tr>'; }});
}}

function folderColor(folder) {{
  const COLORS = ['cyan', 'purple', 'pink', 'orange', 'acid', 'red'];
  let hash = 0;
  for (let i = 0; i < folder.length; i++) {{ hash = (hash * 31 + folder.charCodeAt(i)) >>> 0; }}
  return COLORS[hash % COLORS.length];
}}

function selectArchiveFormat(value, label) {{
  document.getElementById('archive-format-label').textContent = label;
  document.getElementById('archive-format-btn').dataset.value = value;
  document.querySelectorAll('#archive-format-dropdown .theme-item').forEach(el => {{
    el.classList.toggle('selected', el.dataset.value === value);
  }});
  closeAllDropdowns();
}}
function runArchive() {{
  const fmt = document.getElementById('archive-format-btn').dataset.value;
  const btn = document.getElementById('btn-archive');
  const result = document.getElementById('archive-result');
  btn.innerText = 'compressing…'; btn.disabled = true;
  fetch('/cyberdeck/api/command', {{
    method: 'POST',
    headers: {{'Content-Type': 'application/json'}},
    body: JSON.stringify({{ 'ArchiveFiles': fmt }})
  }}).then(r => r.json()).then(msg => {{
    result.textContent = msg;
    btn.innerText = 'Compress'; btn.disabled = false;
  }}).catch(() => {{
    result.textContent = 'archive failed';
    btn.innerText = 'Compress'; btn.disabled = false;
  }});
}}
</script>"#
    )
}

fn folder_chip(folder: Option<&str>) -> String {
    match folder {
        Some(f) => {
            let color = folder_color(f);
            format!(r#"<span class="folder-chip" style="--chip-color: var(--{color});">{f}</span>"#)
        }
        None => r#"<span class="folder-chip folder-chip-root">root</span>"#.to_string(),
    }
}

fn garbled_badge(garbled: bool) -> &'static str {
    if garbled {
        r#" <span class="badge status-partial" title="This report had a broken code fence (a missing tool's fallback text glued against the closing ```) that swallowed everything after it — auto-repaired at view time, the file on disk is untouched.">AUTO-FIXED</span>"#
    } else {
        ""
    }
}

pub fn reports_page(entries: &[ReportEntry]) -> String {
    let rows: String = if entries.is_empty() {
        r#"<tr><td colspan="4"><div class="empty-state">No reports yet — run a scan first.</div></td></tr>"#
            .to_string()
    } else {
        entries
            .iter()
            .map(|e| {
                format!(
                    r#"<tr>
  <td>{folder}</td>
  <td>
    <a class="btn-link" href="/reports/{path}" onclick="event.preventDefault(); openViewer('{path}')">{title}</a>{garbled}
    <div class="muted" style="font-size:0.7rem;">{path}</div>
  </td>
  <td>{size}</td>
  <td class="muted">{modified}</td>
</tr>"#,
                    folder = folder_chip(e.folder.as_deref()),
                    path = e.path,
                    title = escape(&e.title),
                    garbled = garbled_badge(e.garbled),
                    size = e.size_human,
                    modified = e.modified_human,
                )
            })
            .collect()
    };

    format!(
        r#"<div class="dash-head">
  <h1>Reports</h1>
  <p class="muted">Markdown output from every scan, written to <code>diagnostics/</code>. For
  full rendering (wikilinks, search, nicer typography) see the dedicated
  <a class="btn-ghost" style="padding:0.1rem 0.4rem;" href="http://127.0.0.1:8766" target="_blank" rel="noopener">Diagnostic Reports deck</a> — this list is the raw quick view.</p>
</div>
<table class="res-table">
  <thead><tr><th>Folder</th><th>Report</th><th>Size</th><th>Modified</th></tr></thead>
  <tbody>{rows}</tbody>
</table>"#
    )
}

/// Renders the module's raw markdown output as real HTML — headers, code
/// fences, tables, bold/italic — instead of dumping escaped plain text
/// into one undifferentiated grey box.
pub fn render_markdown(content: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};
    let (repaired, _changed) = crate::sanitize::sanitize(content);
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&repaired, opts);
    let mut html_out = String::new();
    html::push_html(&mut html_out, parser);
    html_out
}

pub fn report_view(title: &str, path: &str, content: &str) -> String {
    format!(
        r#"<div class="dash-head">
  <h1>{title}</h1>
  <p class="muted"><code>diagnostics/{path}</code></p>
  <p><a class="btn-ghost" href="/reports">&larr; back to reports</a></p>
</div>
<div class="markdown-body">{}</div>"#,
        render_markdown(content),
        title = escape(title),
        path = escape(path),
    )
}

/// Shown (with a 404) when a report path is valid but nothing has been
/// written there yet — usually because its scan has never run.
pub fn report_missing(path: &str) -> String {
    format!(
        r#"<div class="dash-head">
  <h1>No report yet</h1>
  <p class="muted"><code>diagnostics/{path}</code></p>
  <p>Nothing has been written here yet — run its scan to generate it.</p>
  <p><a class="btn-ghost" href="/scans">Go to scans</a> <a class="btn-ghost" href="/reports">&larr; back to reports</a></p>
</div>"#,
        path = escape(path),
    )
}

/// Fragment form of [`report_missing`] for the popup viewer.
pub fn report_missing_fragment(path: &str) -> String {
    format!(
        r#"<p>No report at <code>diagnostics/{}</code> yet — run its scan from <a href="/scans">Scans</a>.</p>"#,
        escape(path)
    )
}

/// The same rendered markdown, without the page chrome or the
/// `.markdown-body` wrapper (the popup viewer's modal already provides
/// that container) — used by the popup viewer's `fetch()` call.
pub fn report_fragment(content: &str) -> String {
    render_markdown(content)
}
