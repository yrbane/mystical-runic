# Mystical Runic Templater

This project is a command-line tool for rendering templates using the `mystical-runic` templating engine. It takes a template file and a data file (in JSON format) as input and outputs the rendered HTML.

## Features

The templating engine `mystical-runic` supports the following features:

*   **Variable Interpolation**: `{{name}}` - Renders a variable from the data file.
*   **Raw HTML**: `{{& html}}` - Renders unescaped HTML.
*   **Conditionals**: `{{if condition}}...{{/if}}` - Renders a block of HTML if a condition is true.
*   **Loops**: `{{for item in items}}...{{/for}}` - Iterates over a collection and renders a block for each item.
*   **Includes**: `{{include "path/to/template.rune"}}` - Includes another template file.
*   **Comments**: `{{! This is a comment }}` - Comments that are not rendered.
*   **Object Property Access**: `{{object.property}}` - Accesses nested properties of objects.

## Usage

To use the `runic_templater`, run the following command:

```bash
cargo run -- --template <path/to/template.rune> --data <path/to/data.json>
```

For example:

```bash
cargo run -- --template templates/main.rune --data data/main.json
```

## Templates

The `templates` directory contains a set of templates that demonstrate all the features of the `mystical-runic` engine.

*   `templates/main.rune`: The main template that showcases all features.
*   `templates/scrolls/wisdom.rune`: A simple template that is included in `main.rune`.

## Data

The `data` directory contains the JSON data file that is used to render the templates.

*   `data/main.json`: The data for the `main.rune` template.

## Running tests

To run the integration tests, use the following command:

```bash
cargo test
```

## Known Issues

*   **Conditionals in Loops**: There appears to be an issue with using conditionals (`{{if ...}}`) on nested properties of objects within a loop (`{{for ...}}`). The conditional block may not render as expected, even if the condition is met.