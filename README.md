# git_check

This is a simple tool to look for keywords in tracked files in git for a potential commit.

No tests as of yet, this is just a spike.

I know there are plenty of other tools out there, but I just want simple.

You place a `pre-commit` script in `.git/hooks` directory in the repo you want to have the checks done.

Example script, put `git_check` in your path or reference directly:
```bash
#!/bin/sh
  
git_check

if [ $? -ne 0 ]; then
    echo "Aborting commit due to sensitive information."
    exit 1
fi

```

Also you need to place a version of the `config.toml` in the repo within the `.git_check` directory.
