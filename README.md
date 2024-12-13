# Gamelion

Facilitate copying the output of your last command.

## How it works

With `shell-session.txt` containing:

```
~ % cod projects/gamelion
~ % echo "this is a test to show \n how gamelion works"
this is a test to show
 how gamelion works
~ % tmux capture-pane -p > shell-session.txt
```

Use gamelion to get the output of the last command that was ran in this session:

```sh
~ % cat shell-session.txt | gamelion
echo "this is a test to show \n how gamelion works"
this is a test to show
 how gamelion works
```

## The intended way of properly using Gamelion

Assuming you're using tmux, and that the constant `MARKER` is set to something very unique to your shell:

```sh
~ % tmux capture-pane -S - -E - -p | gamelion
echo "this is a test to show \n how gamelion works"
this is a test to show
 how gamelion works
```

The way this works:

- `tmux capture-pane -S - -E - -p` prints all your terminal session 
  - `-S -` means "from the start of the session", instead of the default "from the beginning of what you can see"
  - `-E -` means "until the end"
  - `-p` means print
- In my prompt I have a non-breakable space (thanks to [Ian Henry for the trick](https://ianthehenry.com/posts/tmux-copy-last-command/))
- Gamelion extracts the text between the two last prompts. Since the last command already ended, this is exactly the last command and its output

## How to make it work for you

I'd recommand adding a non-breakable space in your prompt as those are pretty rare, and Gamelion's heuristic for detectic your prompt is very basic. In zsh I do this with `printf '\u00A0'` (another [trick from the same article by Ian Henry](https://ianthehenry.com/posts/tmux-copy-last-command/)).

Then modify the `MARKER` constant to be this unique part of your prompt.

## Why

I often want to copy the output of the last command I ran. There are a few options:

- selecting it with the mouse in the terminal, but my mouse has this issue where sometimes it suddenly stops considering I'm pressing left click so I have so start over
- run the command again and pipe it into `xclip`, but then I'd have to remember to do the `2>&1` trick all the time
- copying the output to a file with `tee`, but when I tried this ended up stripping the color of my terminal outputs
- selecting with the keyboard thanks to tmux, but this is still kind of a manual selection, even if it suffer from less issues than the mouse one
- finally, use tmux, write a program to extract the text between the two last prompts, alias `tmux capture-pane -S - -E - -p | gamelion` to `lop` (Last OutPut. I realized only after the fact that "lop" is an actual word.), use that
