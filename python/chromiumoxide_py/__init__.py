import chromiumoxide_py
from chromiumoxide_py.browser import BrowserConfig

from . import bindings

Browser = bindings.browser.Browser
Element = bindings.Element
Page = bindings.Page

__all__ = [
    "bindings",
    "Browser",
    "BrowserConfig",
    "Element",
    "Page",
]
