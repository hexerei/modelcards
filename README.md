# Modelcards

Multifunctional commandline tool to work with [modelcards](https://modelcards.withgoogle.com/).

The CLI mainly supports two modes of working:

- **Pipeline** mode to work in CI/CD pipelines or as stand-alone utility in your terminal
- **Project** mode to create custom schemas and templates

Subcommands that work in any mode are:

- **completion** to generate shell completions
  ```sh
  ❯ modelcards completion
  ```
- **help** prints general help or help for given subcommand
  ```sh
  ❯ modelcards help
  A fast modelcard generator with built-in templates

  Usage: modelcards [OPTIONS] <COMMAND>

  Commands:
    init        Create a new modelcard project
    build       Deletes the output directory if there is one and builds the modelcard
    check       Try to build the project without rendering it. Checks inputs
    validate    Validate the modelcard data file against the schema
    render      Render the modelcard using template
    merge       Merge multiple modelcard data files into one
    completion  Generate shell completion
    help        Print this message or the help of the given subcommand(s)

  Options:
    -r, --root <ROOT>      Directory to use as root of project [default: .]
    -c, --config <CONFIG>  Path to a config file other than config.toml in the root of project [default: config.toml]
    -v, --verbose...       Increase logging verbosity
    -q, --quiet...         Decrease logging verbosity
    -h, --help             Print help
    -V, --version          Print version
  ```
## Pipeline Mode

The pipeline mode currently supports three subcommands:

### merge - Merge multiple json files

The **merge** subcommand is used to merge two or more json files on value level. This allows you to either separate large json structures or create json files with defaults or globals.

This is specifically useful to reduce the work of documentation on the developer side. You could e.g., store global defaults in a separate json file, where you prefill mandatory fields or assign company wide copyrights, references, etc. - then you can put use-case specific documentation (like uses, considerations, etc.) in a separate json file that could be re-used for all models in your use-case and lastly a json file with the details of a specific model. Then you could generate the full modelcard json data file with:

```sh
❯ modelcards merge defaults.json usecase.json model.json -o modelcard.json 
```

#### Syntax

```sh
Usage: modelcards merge [OPTIONS] [SOURCES]...

Arguments:
  [SOURCES]...  The source modelcard data files to be merged

Options:
  -o, --target <TARGET>  The output file to write the merged data to
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging
```

### validate - Validate modelcard data against json schema

Pass modelcard json data file to validate against schema. If no schema is given, the buildt-in schema for the Google Modelcard Toolkit is used.

If you pass more than one json file, they are not validated one-by-one, but in fact they are merged before validation, as if you would first call **merge** command and then **validate** the result.

To validate against Google schema:

```sh
❯ modelcards validate modelcard.json
```

To validate against your own custom schema:

```sh
❯ modelcards validate modelcard.json -s myschema.json
```

#### Syntax

```sh
Usage: modelcards validate [OPTIONS] [SOURCES]...

Arguments:
  [SOURCES]...  The source modelcard data file to be verified

Options:
  -s, --schema <SCHEMA>  The schema file to validate against (defaults to build-in schema)
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -h, --help             Print help
```

### render - Render modelcard with given Jinja template

The render command uses [Jinja templates](https://github.com/mitsuhiko/minijinja) to transform the modelcard json data to what ever format is desired.

Pass modelcard json data file to render. If no template is given, the buildt-in Markdown template for the Google Modelcard Toolkit data schema is used.

If you pass more than one json file, they are not rendered one-by-one, but in fact they are merged before renderibng, as if you would first call **merge** command and then **render** the result.

The result will be stored in a file named like the last modelcard source you passed, but with .md extension.

To render with the default template, you can either call:

```sh
❯ modelcards render modelcard.json
```

This will create ```modelcard.md``` as result.

Or if you pass multiple files:

```sh
❯ modelcards render default.json usecase.json model.json
```

This will create ```model.md``` as result, since the last file passed was the ```model.json```source.

To render using your own custom template:

```sh
❯ modelcards render modelcard.json -t my-html-template.jinja
```

#### Syntax

```sh
Usage: modelcards render [OPTIONS] [SOURCES]...

Arguments:
  [SOURCES]...  The source modelcard data file to be verified

Options:
  -t, --template <TEMPLATE>  The jinjia template file to use (defaults to build-in markdown template)
  -v, --verbose...           Increase logging verbosity
  -q, --quiet...             Decrease logging verbosity
  -h, --help                 Print help
```

### Continuous Integration Sample

To effectively use the cli utility in your machine learning project, assuming you have a default.json, usecase.jsonl, first_model.json and second_model.json in your repository, you could update the model json filew with the most current metrics from your last model version and then merge, validate and render the modelcard.

```sh
# merge both model details to final modelcard for each
modelcards merge default.json usecase.json first_model.json -o modelcard_first.json
modelcards merge default.json usecase.json second_model.json -o modelcard_second.json
# assure that modelcard data is valid (exits with 1 on validation error and 0 if data is valie)
modelcards validate modelcard_first.json
modelcards validate modelcard_second.json
# render the data to markdown
modelcards render modelcard_first.json
modelcards render modelcard_second.json
# optionally create links to the generated modelcards in your README.md
```

## Project Mode

Documentation for project mode will follow, currently three subcommands work in project mode:

### init - Create a new modelcard project

#### Syntax

```sh
Usage: modelcards init [OPTIONS] [NAME]

Arguments:
  [NAME]  Name of the project. Will create a new directory with that name in the current directory [default: .]

Options:
  -f, --force       Force creation of project even if directory is non-empty
  -v, --verbose...  Increase logging verbosity
  -q, --quiet...    Decrease logging verbosity
  -h, --help        Print help
```

### check - Build the project without rendering to check all inputs

#### Syntax

```sh
Usage: modelcards check [OPTIONS]

Options:
  -s, --source <SOURCE>  The source modelcard data file to be verified (defaults to sample.json or settings in config.toml)
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logging verbosity
  -h, --help             Print help
```

### build - Builds modelcard project to output directory

#### Syntax

```sh
Usage: modelcards build [OPTIONS]

Options:
  -s, --source <SOURCE>  The source modelcard data file to be build (defaults to all in 'data' dir in project root)
  -o, --target <TARGET>  Outputs the generated site in the given path (by default 'card' dir in project root)
  -f, --force <FORCE>    Force building the modelcard even if output directory is non-empty [possible values: true, false]
  -v, --verbose...       Increase logging verbosity
  -q, --quiet...         Decrease logg
```



## Features

- [x] Create modelcard from template
- [ ] Hierarchical settings (default, config.toml, env, cli args)
- [ ] Prettier output with crossterm crate
- [ ] Data input from terminal via inquire crate

## Contributions

The schema and templates are based on Google's Model Card Toolkit to ensure compatability with integrations.
These schema and templates are copyright 2019 The TensorFlow Authors. All rights reserved.

Integration of HuggingCard Templates is planned.

## References

- <https://www.nocode.ai/ai-model-cards-101-an-introduction-to-the-key-concepts-and-terminology/>
- <https://modelcards.withgoogle.com/>
- <https://github.com/tensorflow/model-card-toolkit/>
- <https://huggingface.co/docs/hub/model-cards>
- <https://huggingface.co/spaces/huggingface/Model_Cards_Writing_Tool>
- <https://blog.research.google/2020/07/introducing-model-card-toolkit-for.html>
- <https://ianhellstrom.org/ml-cards/>
- <https://github.com/ivylee/model-cards-and-datasheets>

- <https://docs.rs/minijinja/latest/minijinja/index.html>

Not related, but future uses possible:

- <https://www.maybevain.com/writing/using-tailwind-css-with-zola-static-site-generator/>
- <https://github.com/dvogt23/mdbook-yml-header/tree/main>
- <https://github.com/wisbery/yapp>
- <https://github.com/max-heller/mdbook-pandoc>
- <https://github.com/rust-lang/mdBook/wiki/Third-party-plugins>
- <https://rust-lang.github.io/mdBook/format/configuration/renderers.html>
