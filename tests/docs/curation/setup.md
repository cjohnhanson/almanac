```markdown {filename="upstream/skills/writing/SKILL.md"}
---
name: writing
description: How to write a commit message.
---

Write the subject line in the imperative mood.
```

```console
$ mv upstream ..

$ git -C ../upstream init -q -b main

$ git -C ../upstream add .

$ git -C ../upstream commit -q -m "Add the writing skill"

$ almanac init
initialized almanac.yml (library: skills)

```

