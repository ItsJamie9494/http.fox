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
    {{ utils::link(href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/102", text="102 Processing") }}
</li>

{% endblock see_also %}
```

This section contains an unordered list of URLs. It should only contain `<li>` elements, and these elements should have a body created by the `link(href, text)` macro.

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

## Importing Macros

All macros are stored in `utils.html.tera`.

```jinja
{% import "statuses/utils" as utils %}
```