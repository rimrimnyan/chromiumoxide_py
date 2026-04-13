import chromiumoxide_py
from chromiumoxide_py.browser import BrowserConfig

from . import bindings

Browser = bindings.browser.Browser
Element = bindings.element.Element
Page = bindings.page.Page

__all__ = [
    "bindings",
    "Browser",
    "BrowserConfig",
    "Element",
    "Page",
]
