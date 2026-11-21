```console
$ almanac --root team init
initialized almanac.yml (library: skills)

$ almanac --root upstream init
initialized almanac.yml (library: skills)

$ almanac init
initialized almanac.yml (library: skills)

```

```markdown {filename="team/skills/testing/SKILL.md"}
---
name: testing
description: How the team tests.
---

Write the failing test first.
```

```markdown {filename="team/skills/writing/SKILL.md"}
---
name: writing
description: The team's writing guide.
---

Write for the reader.
```

```markdown {filename="upstream/skills/linting/SKILL.md"}
---
name: linting
description: How to configure the linters.
---

Turn every warning into an error.
```

```markdown {filename="skills/writing/SKILL.md"}
---
name: writing
description: How to write a commit message.
---

Write the subject line in the imperative mood.
```

```console
$ mv team ../team-skills

$ mv upstream ../upstream

$ git -C ../upstream init -q -b main

$ git -C ../upstream add .

$ git -C ../upstream commit -q -m "Add the linting skill"

```

