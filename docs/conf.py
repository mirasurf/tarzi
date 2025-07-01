# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
import sys
from pathlib import Path

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "tarzi"
copyright = "2024, Mirasurf"
author = "xmingc"
release = "0.0.4"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.viewcode",
    "sphinx.ext.napoleon",
    "sphinx.ext.intersphinx",
    "sphinx.ext.todo",
    "sphinx.ext.coverage",
    "sphinx.ext.ifconfig",
    "sphinx_autodoc_typehints",
    "sphinx_copybutton",
    "myst_parser",
    "sphinx_tabs.tabs",
    "sphinx_design",
    # "autoapi.extension",  # Temporarily disabled for ReadTheDocs build
]

# Add any paths that contain templates here, relative to this directory.
templates_path = ["_templates"]

# List of patterns, relative to source directory, that match files and
# directories to ignore when looking for source files.
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# The suffix(es) of source filenames.
source_suffix = {
    ".rst": None,
    ".md": "markdown",
}

# The master toctree document.
master_doc = "index"

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "furo"
html_static_path = ["_static"]

# Custom CSS and JavaScript
html_css_files = [
    "custom.css",
]

# Theme options
html_theme_options = {
    "sidebar_hide_name": False,
    "light_logo": "tarzi-logo-light.png",
    "dark_logo": "tarzi-logo-dark.png",
    "source_repository": "https://github.com/mirasurf/tarzi.rs/",
    "source_branch": "main",
    "source_directory": "docs/",
}

# The name of an image file (relative to this directory) to place at the top
# of the sidebar.
html_logo = "_static/tarzi-logo.png"

# The name of an image file (within the static path) to use as favicon of the
# docs.
html_favicon = "_static/favicon.ico"

# -- Extension configuration -------------------------------------------------

# -- Options for autodoc extension -------------------------------------------
autodoc_default_options = {
    "members": True,
    "member-order": "bysource",
    "special-members": "__init__",
    "undoc-members": True,
    "exclude-members": "__weakref__",
}

# -- Options for autosummary extension ---------------------------------------
autosummary_generate = True
autosummary_generate_overwrite = True

# -- Options for napoleon extension ------------------------------------------
napoleon_google_docstring = True
napoleon_numpy_docstring = True
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = False
napoleon_use_admonition_for_notes = False
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True

# -- Options for intersphinx extension ---------------------------------------
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "requests": ("https://requests.readthedocs.io/en/stable/", None),
}

# -- Options for autodoc_typehints extension ---------------------------------
always_document_param_types = True
typehints_fully_qualified = False
typehints_document_rtype = True

# -- Options for sphinx-autoapi extension -----------------------------------
autoapi_type = "python"
autoapi_dirs = ["../"]
autoapi_root = "api"
autoapi_ignore = [
    "*tests*",
    "*test_*",
    "*/target/*",
    "*/examples/*",
    "*/.git/*",
    "*/__pycache__/*",
    "*/.venv/*",
    "*/venv/*",
    "*/.pytest_cache/*",
    "*/python/tarzi/*",  # Ignore the Python package directory to avoid import issues
]
autoapi_options = [
    "members",
    "undoc-members",
    "show-inheritance",
    "show-module-summary",
    "imported-members",
]
autoapi_python_class_content = "both"
autoapi_member_order = "bysource"
autoapi_generate_api_docs = True

# -- Options for myst-parser extension ---------------------------------------
myst_enable_extensions = [
    "colon_fence",
    "deflist",
    "dollarmath",
    "fieldlist",
    "html_admonition",
    "html_image",
    "linkify",
    "replacements",
    "smartquotes",
    "strikethrough",
    "substitution",
    "tasklist",
]

# -- Options for copybutton extension ----------------------------------------
copybutton_prompt_text = r">>> |\.\.\. |\$ |In \[\d*\]: | {2,5}\.\.\.: | {5,8}: "
copybutton_prompt_is_regexp = True
copybutton_line_continuation_character = "\\"

# -- Options for todo extension ----------------------------------------------
todo_include_todos = True

# -- Custom setup ------------------------------------------------------------
def setup(app):
    """Custom setup function for Sphinx."""
    app.add_css_file("custom.css")
    
    # Add custom roles and directives if needed
    pass 