from dataclasses import dataclass, field
from enum import IntEnum
from typing import Any

from chromiumoxide_py import bindings

Browser = bindings.browser.Browser


class HeadlessMode(IntEnum):
    FALSE = 0
    TRUE = 1
    NEW = 2


@dataclass
class BrowserConfig:
    """Configuration for the browser."""

    headless_mode: HeadlessMode | bool = field(default=HeadlessMode.TRUE)
    """
    Determines whether to run headless version of the browser.
    Defaults to true.
    """

    sandbox: bool = field(default=True)
    """Determines whether to run the browser with a sandbox."""

    window_size: tuple[int, int] | None = field(default=None)
    """Launch the browser with a specific window width and height."""

    port: int = field(default=0)
    """Launch the browser with a specific debugging port."""

    chrome_executable: str | None = field(default=None)
    """Path for Chrome or Chromium.

    If unspecified, the create will try to automatically detect a suitable
    binary."""

    extensions: list[str] = field(default_factory=list)
    """A list of Chrome extensions to load.

    An extension should be a path to a folder containing the extension code.
    CRX files cannot be used directly and must be first extracted.

    Note that Chrome does not support loading extensions in headless-mode.
    See https://bugs.chromium.org/p/chromium/issues/detail?id=706008#c5"""

    process_envs: dict[str, str] | None = field(default=None)
    """Environment variables to set for the Chromium process.
    Passes value through to std::process::Command::envs."""

    user_data_dir: str | None = field(default=None)
    """Data dir for user data"""

    incognito: bool = field(default=False)
    """Whether to launch the `Browser` in incognito mode"""

    launch_timeout: float = field(default=20_000)
    """Timeout duration for `Browser::launch` (in milliseconds)"""

    ignore_https_errors: bool = field(default=True)
    """Ignore https errors, default is true"""

    ignore_invalid_messages: bool = field(default=True)
    """Ignore invalid messages, default is true"""

    disable_https_first: bool = field(default=False)
    """Disable HTTPS-first features (HttpsUpgrades, HttpsFirstBalancedModeAutoEnable)"""

    # viewport: Any | None = field(default=None)
    """The viewport of the browser"""

    request_timeout: float = field(default=20_000)
    """The duration after a request with no response should time out (in milliseconds)"""

    args: list[str] = field(default_factory=list)
    """Additional command line arguments to pass to the browser instance."""

    disable_default_args: bool = field(default=False)
    """Whether to disable DEFAULT_ARGS or not, default is false"""

    request_intercept: bool = field(default=False)
    """Whether to enable request interception"""

    cache_enabled: bool = field(default=True)
    """Whether to enable cache"""

    hidden: bool = field(default=True)
    """Avoid easy bot detection by setting `navigator.webdriver` to false"""

    def __post_init__(self):
        if isinstance(self.headless_mode, bool):
            self.headless_mode = (
                HeadlessMode.TRUE if self.headless_mode else HeadlessMode.FALSE
            )

    def launch(self) -> Browser:
        return Browser.launch(self)  # type: ignore
