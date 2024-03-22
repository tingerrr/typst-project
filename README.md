# typst-project
typst-project is a small library for interacting with typst projects. It is currently used in
[typst-test], but may be upstreamd to other projects or typst itself at some point.

Planned features are reading and writing 3rd-party tool configs directly into the typst.toml
manifest file with minimal churn for the user using toml-edit.

THe LICENSE of this project is currently unspecified but will likely be Apache 2.0 as a lot of code
was hoisted the [typst package bundler][bundler].

[typst-test]: https://github.com/tingerrr/typst-test
[bundler]: https://github.com/typst/packages
