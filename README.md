# chromiumoxide-py

Python bindings for [`chromiumoxide`](https://crates.io/crates/chromiumoxide) — a high-level Rust API for controlling Chrome/Chromium over the [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/).

## Features

| Python API | Rust equivalent |
|------------|----------------|
| `BrowserConfigBuilder` | `BrowserConfig::builder()` |
| `Browser.launch(cfg)` | `Browser::launch(config).await` |
| `browser.new_page(url)` | `browser.new_page(url).await` |
| `page.wait_for_navigation()` | `page.wait_for_navigation().await` |
| `page.content()` | `page.content().await` |
| `page.add_script_to_evaluate_on_new_document(params)` | `page.execute(AddScriptToEvaluateOnNewDocumentParams{…}).await` |
| `page.enable_stealth_mode()` | `page.enable_stealth_mode().await` |
| `page.evaluate(expr)` | `page.evaluate(expr).await` |
| `page.screenshot()` | `page.screenshot(params).await` |
| `page.pdf()` | `page.pdf(params).await` |
| `page.find_element(sel)` | `page.find_element(sel).await` |
| `page.find_elements(sel)` | `page.find_elements(sel).await` |
| `element.click()` | `element.click().await` |
| `element.type_str(text)` | `element.type_str(text).await` |
| `element.press_key(key)` | `element.press_key(key).await` |
| `element.inner_text()` | `element.inner_text().await` |
| `element.inner_html()` | `element.inner_html().await` |
| `element.attribute(name)` | `element.attribute(name).await` |

All async Rust calls are exposed as **synchronous Python calls** using a shared `tokio` runtime.  This matches the ergonomics of `requests` or `playwright-sync`.

---

## Building

Requirements:
- Rust 1.75+ (`rustup` recommended)
- Python 3.8+
- [`maturin`](https://www.maturin.rs/)

```bash
pip install maturin
# Development build (fast recompile, no optimisations)
maturin develop

# Release build (optimised, for production / distribution)
maturin develop --release

# Build a wheel
maturin build --release
```

---

## Quick start

```python
from chromiumoxide_py import BrowserConfig, Browser

cfg  = BrowserConfig.builder().chrome_executable("/usr/bin/chromium").build()
b    = Browser.launch(cfg)
page = b.new_page("https://example.com")
html = page.wait_for_navigation().content()
print(html[:200])
b.close()
```

### Stealth injection (replicating the Rust inject_stealth example)

```python
from pathlib import Path
from chromiumoxide_py import AddScriptToEvaluateOnNewDocumentParams

EVASIONS_PATH = Path("/opt/stealth-scripts")
SCRIPTS = ["chrome.runtime.js", "navigator.plugins.js"]

def inject_stealth(page):
    if EVASIONS_PATH.is_dir():
        for script in SCRIPTS:
            source = (EVASIONS_PATH / script).read_text()
            params = AddScriptToEvaluateOnNewDocumentParams(
                source=source,
                run_immediately=True,
            )
            page.add_script_to_evaluate_on_new_document(params)
    page.enable_stealth_mode()
```

### wait_for_navigation → content

```python
# Direct equivalent of:
#   let html = page.wait_for_navigation().await?.content().await?;
html = page.wait_for_navigation().content()
```

### Element interaction

```python
page.find_element("input#searchInput") \
    .click() \
    .type_str("Rust programming language") \
    .press_key("Enter")

html = page.wait_for_navigation().content()
```

---

## Notes

- **`BrowserConfig` is single-use.** It is consumed by `Browser.launch()`; create a new config for each launch.
- The internal Tokio handler task is managed automatically – you do not need to poll it from Python.
- All methods raise `RuntimeError` on failure, mirroring Rust's `?` propagation.
- `Page.evaluate()` returns the result as a **JSON string**; parse it with `json.loads()` as needed.
- `Page.screenshot()` and `Page.pdf()` return raw `bytes`.
