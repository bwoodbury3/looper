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
    path = LOOPER_PROJECTS / name
    path.write_text(content)


def restore_project(name: str) -> str:
    """
    Restore a project from disk.

    Args:
        name: The name of the project.
    """
    path = LOOPER_PROJECTS / name
    return path.read_text()


def all_projects() -> list[str]:
    """
    Get all available project files.
    """
    return [f.name for f in LOOPER_PROJECTS.iterdir() if f.is_file()]
