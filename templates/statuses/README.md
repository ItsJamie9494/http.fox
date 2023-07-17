## Section Formats

### `notices`

```jinja
{% block notices %}

{{ utils::unofficial() }}

{% endblock notices %}
```

This section contains notices, such as those created by the `unofficial()`, `deprecated()`, and `experimental()` macros. It should not contain any other HTML elements

### `description`

```jinja
{% block description %}

The HTTP {{ utils::pre(text="102 Processing") }} informational status response code indicates to client that a full request has been received and the server is working on it.

{% endblock description %}
```

This section contains the main body of the status page. It can contain other HTML elements, although they must be manually styled. To add a new paragraph, use `<br /><br />`.

### `see_also`

```jinja
{% block see_also %}

<li>
    {% include "components/mdnspec" %}
</li>

{% endblock see_also %}
```

This section contains an unordered list of URLs. It is designed to have `header(header)` macros, the `mdnspec` component, and `<li>` elements with a body consisting of the `link(href, text)` macro.

## Available Macros

- `unofficial()`:
    Prints a warning that this current status is unofficial and not specified in any RFC.
- `deprecated()`:
    Prints a deprecation warning and warns the user about using this status in any new code.
- `experimental()`:
    Prints an experimental warning and warns the user to check browser compatibility.

- `warning(text)`:
    Prints a custom warning message.
- `note(text)`:
    Prints a custom detail note.
- `pre(text)`:
    Formats text in a monospace box.
- `link(href, text)`:
    Creates a styled link that opens in a new tab
- `header(header)`:
    Creates a styled link to MDN with the specified header


### Importing Macros

All macros are stored in `utils.html.tera`.

```jinja
{% import "statuses/utils" as utils %}
```

## Available Components

All components are stored in `/templates/components`

- `mdnspec`
    Creates a link to the MDN documentation on the current status. It is intended to be used in the [See Also](#see_also) section

### Importing Components

```jinja
{% include "components/<COMPONENT_NAME>" %}
```

## Available Variables

- `global`
  - `global.title`:
    The title of the site (usually http.fox)
  - `global.base_url`:
    The base URL of the site (usually https://httpfox.gay or localhost:8000)
- `code`:
    Status code (like 404)
- `message`:
    Status message (like Not Found)
- `credits`
  - `credits.name`:
    Name of person providing fox image
  - `credits.url`:
    A URL linking to the person providing fox image
  - `credits.code`:
    Status code (like 404)
> **NOTE**: `credits.code` is mostly used internally, and should not be used in any status pages

### Using Variables

> **NOTE**: Variables cannot be used inside of macros

```jinja
{{ variable_name }}
```