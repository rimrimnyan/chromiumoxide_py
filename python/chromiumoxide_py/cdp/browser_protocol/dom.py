from dataclasses import dataclass, field


@dataclass
class AddScriptToEvaluateOnNewDocument:
    source: str
    world_name: str | None = field(default=None)
    include_command_line_api: bool | None = field(default=None)
    run_immediately: bool | None = field(default=None)

    IDENTIFIER: str = field(init=False, default="Page.addScriptToEvaluateOnNewDocument")
