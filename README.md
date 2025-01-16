<div align="center">

# iced_receipts

A multi-screen desktop application built with
[iced](https://github.com/iced-rs/iced) demonstrating some neat patterns for
state management, screen transitions, and form handling.

[![Made with iced](https://iced.rs/badge.svg)](https://github.com/iced-rs/iced)

<img src="demo.gif" alt="App Demonstration" width="600"/>

</div>

## Overview

This example showcases a few useful patterns for building desktop applications with `iced`:

- Screens sharing single simple straightforward state, as said by someone struck
  with some alliteration spell
- A clean and flexible `Action` API for handling screen events and operations
- Form handling with keyboard navigation (Tab and Escape)

## Project Structure

```
src/
├── main.rs        # App entry point and top level state management
├── list.rs        # Simple sales list screen
├── sale.rs        # Edit/view mode screens example
│   ├── edit.rs    # Edit screen for creating and updating sales
│   └── show.rs    # Read-only mode for sales
└── action.rs      # Action API for handling operations
```

## Action API

The example uses a `Action` type providing a unified way to handle both
operations and tasks across screens, where operations are in-app actions
directly by your application while tasks are handled by the `iced` runtime.

Although not used in this example, this `Action` type allows the user to return
both an operation and a task, instead of the more commonly seen `enum` approach
where you can only return one or the other. This is useful, for instance, for
minor UI-related tasks such as focusing a specific input field after an
operation is handled by your app.

```rust
pub struct Action<Operation, Message> {
    pub operation: Option<Operation>,
    pub task: Task<Message>,
}
```

More information about the `Action` type can be found in the
[action.rs](src/action.rs) file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

## Contributing

This is an example project meant for learning purposes. Feel free to use these
patterns in your own projects!