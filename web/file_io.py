"""
File I/O
"""

import os
import pathlib


def _ensure_exists(path: pathlib.Path):
    """
    Ensure that this path exists.

    Args:
        path: The path to ensure that it exists.
    """
    path.mkdir(parents=True, exist_ok=True)
    return path


def _assert_extension(fname: str, extension: str = ".json"):
    """
    Assert that a filename ends with the provided extension.

    Args:
        fname: The filename
        extension: The extension
    """
    if not fname.endswith(".json"):
        return fname + extension
    return fname


def _trim_extension(fname: str, extension: str = ".json"):
    """
    Trim the provided extension off of the filename.

    Args:
        fname: The filename
        extension: The extension
    """
    return fname.removesuffix(extension)


HOME = pathlib.Path(os.path.expanduser("~"))

LOOPER_BASE = _ensure_exists(HOME / "looper")
LOOPER_PROJECTS = _ensure_exists(LOOPER_BASE / "projects")


def save_project(name: str, content: str):
    """
    Save a project to disk.

    Args:
        name: The name of the project.
        content: The content to write to the file.
    """
    path = LOOPER_PROJECTS / _assert_extension(name)
    path.write_text(content)


def restore_project(name: str) -> str:
    """
    Restore a project from disk.

    Args:
        name: The name of the project.
    """
    path = LOOPER_PROJECTS / _assert_extension(name)
    return path.read_text()


def all_projects() -> list[str]:
    """
    Get all available project files.
    """
    return [_trim_extension(f.name) for f in LOOPER_PROJECTS.iterdir() if f.is_file()]
