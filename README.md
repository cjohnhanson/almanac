# almanac

> An almanac (historically spelled almanack) is a regularly published
> listing of a set of current information about one or multiple subjects.
> —Wikipedia

Almanac aggregates agent skills from multiple sources and makes them
discoverable. A skill is a directory containing a SKILL.md file with
YAML frontmatter that declares a name and one-line description. The
body holds the actual instructions.

## How it works

Skills come from three kinds of sources: local directories, built-in
skills compiled into the binary, and (planned) remote git repositories.

`almanac list` prints every available skill with its description.
`almanac show <name>` prints the full SKILL.md content. Coding agents
call these commands to discover what they know how to do, loading full
instructions only for skills relevant to the current task.

## Built-in skills

Almanac ships with skills for debugging, testing strategy, code review,
security review, API design evaluation, writing review, research, and
browser automation. These are compiled into the binary and available
without any configuration.

## Usage

```
almanac list                  # all available skills
almanac show <name>           # full SKILL.md content
almanac search <query>        # search skills by keyword
almanac index                 # machine-readable JSON index
almanac docs [topic]          # bundled documentation
```

## Documentation

- [What is Almanac?](docs/what-is-almanac.md) — skill format, sources, progressive disclosure
- [CLI Reference](docs/cli-reference.md) — complete command documentation
