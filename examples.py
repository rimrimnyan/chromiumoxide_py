"""
chromiumoxide_py usage examples
================================

These examples mirror the three Rust snippets in the task description.
Build the extension first::

    pip install maturin
    maturin develop --release

or install from the wheel::

    pip install chromiumoxide-py
"""

from __future__ import annotations

import pathlib
from collections.abc import Buffer

from chromiumoxide_py.bindings import (
    _AddScriptToEvaluateOnNewDocumentParams,
    _Browser,
    _BrowserConfig,
    _BrowserConfigBuilder,
)

# ── Example 1 ─────────────────────────────────────────────────────────────────
# Equivalent Rust:
#   let (browser, mut handler) = Browser::launch(
#       BrowserConfig::builder()
#           .chrome_executable(CHROME_EXECUTABLE_PATH)
#           .build()?,
#   ).await?;
#   let handle = tokio::task::spawn(async move { ... });
#   let page = browser.new_page("chrome://version/").await?;


def example_launch_and_navigate() -> str:
    # The background Tokio handler task is started automatically inside
    # Browser.launch; you don't need to manage it manually from Python.
    builder = _BrowserConfigBuilder()
    builder.chrome_executable("/usr/bin/chromium")  # adjust path as needed
    builder.no_sandbox()  # common inside Docker

    cfg = builder.build()
    b = _Browser.launch(cfg)
    page = b.new_page("chrome://version/")
    html = page.content()
    b.close()
    return html


# ── Example 2 ─────────────────────────────────────────────────────────────────
# Equivalent Rust:
#   self.page.execute(AddScriptToEvaluateOnNewDocumentParams { ... }).await?;
#   self.page.enable_stealth_mode().await?;

EVASIONS_SCRIPT_PATH = pathlib.Path("/opt/stealth-scripts")  # adjust as needed
SCRIPTS = [
    "chrome.runtime.js",
    "navigator.plugins.js",
    "navigator.languages.js",
    "webgl.vendor.js",
]


def inject_stealth(page) -> None:
    """Inject stealth evasion scripts + enable built-in stealth mode."""
    print("Injecting stealth...")

    if EVASIONS_SCRIPT_PATH.is_dir():
        for script_name in SCRIPTS:
            source = (EVASIONS_SCRIPT_PATH / script_name).read_text()
            params = _AddScriptToEvaluateOnNewDocumentParams(
                source=source,
                include_command_line_api=None,
                world_name=None,
                run_immediately=True,
            )
            page.add_script_to_evaluate_on_new_document(params)
    else:
        print("evasions path specified but not a directory!")

    page.enable_stealth_mode()


# ── Example 3 ─────────────────────────────────────────────────────────────────
# Equivalent Rust:
#   let html = page.wait_for_navigation().await?.content().await?;


def example_wait_for_nav_and_content(page) -> str:
    """Chain wait_for_navigation() → content()."""
    html = page.wait_for_navigation().content()
    return html


# ── Full end-to-end stealth scraper ───────────────────────────────────────────


def scrape_with_stealth(url: str, chrome: str = "/usr/bin/chromium") -> str:
    """
    Launch a stealth browser, navigate to `url`, and return the page HTML.

    Steps:
    1. Launch browser
    2. Open a page
    3. Inject stealth scripts
    4. Navigate to the target URL
    5. Wait for navigation, grab HTML
    6. Close browser
    """
    builder = _BrowserConfigBuilder()
    builder.chrome_executable(chrome)
    builder.no_sandbox()
    cfg = builder.build()

    browser = _Browser.launch(cfg)
    try:
        page = browser.new_page("about:blank")
        inject_stealth(page)
        page.goto(url)
        html = page.wait_for_navigation().content()
        return html
    finally:
        browser.close()


# ── Screenshot helper ─────────────────────────────────────────────────────────


def take_screenshot(
    url: str, out_path: str = "screenshot.png", chrome: str = "/usr/bin/chromium"
) -> None:
    """Navigate to `url` and save a PNG screenshot to `out_path`."""
    builder = _BrowserConfigBuilder()
    builder.chrome_executable(chrome)
    builder.no_sandbox()
    cfg = builder.build()

    browser = _Browser.launch(cfg)
    try:
        page = browser.new_page(url)
        page.wait_for_navigation()
        png_bytes = page.screenshot()
        pathlib.Path(out_path).write_bytes(Buffer(png_bytes))
        print(f"Screenshot saved to {out_path} ({len(png_bytes)} bytes)")
    finally:
        browser.close()


# ── Element interaction ───────────────────────────────────────────────────────


def search_wikipedia(query: str, chrome: str = "/usr/bin/chromium") -> str:
    """Type `query` into Wikipedia's search box and return the result HTML."""
    builder = _BrowserConfigBuilder()
    builder.chrome_executable(chrome)
    builder.no_sandbox()
    cfg = builder.build()

    browser = _Browser.launch(cfg)
    try:
        page = browser.new_page("https://en.wikipedia.org")
        (
            page.find_element("input#searchInput")
            .click()
            .type_str(query)
            .press_key("Enter")
        )
        html = page.wait_for_navigation().content()
        return html
    finally:
        browser.close()


if __name__ == "__main__":
    # Quick smoke-test – just launches the browser and grabs chrome://version/
    html = example_launch_and_navigate()
    print(html[:500])
