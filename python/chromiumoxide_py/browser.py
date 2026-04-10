from dataclasses import dataclass, field
from typing import Any

from chromiumoxide_py.bindings import _Browser, _BrowserConfig


@dataclass
class BrowserConfig:
    chrome_executable: str | None = field(default=None)
    headless: bool = field(default=False)
    sandbox: bool = field(default=False)
    args: str | None = field(default=None)
    window_width: int | None = field(default=None)
    window_height: int | None = field(default=None)
    user_data_dir: str | None = field(default=None)

    def launch(self) -> _Browser:

        builder = _BrowserConfig.builder()

        if self.chrome_executable:
            builder.chrome_executable(self.chrome_executable)
        if not self.headless:
            builder.with_head()
        if not self.sandbox:
            builder.no_sandbox()
        if self.args:
            builder.arg(self.args)
        if self.window_width and self.window_height:
            builder.window_size(self.window_width, self.window_height)
        if self.user_data_dir:
            builder.user_data_dir(self.user_data_dir)

        config = builder.build()

        return _Browser.launch(config)
