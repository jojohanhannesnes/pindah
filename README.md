Trouble<br>
[Command](https://doc.rust-lang.org/std/process/struct.Command.html)

This approach doesn't change the working directory of your current shell session directly but rather spawns a new shell process that executes the cd command. Changing the working directory of the current shell session from a child process (your Rust program) is not possible because child processes run in isolated environments.
<br>
Workaround = direct bash

# Config

1. Copy the bash function to your .zshrc or .bashrc
2. Customize the alias according to your preferences for the command.
3. cargo build --release
4. Add releases to $PATH

# How to use

```
p save {{directory alias name}} saving the config to `$HOME/.pindah`
p move {{asd}} move to the alias directory if exist
```

https://github.com/jojohanhannesnes/pindah/assets/118413317/a49e25b7-aea9-4284-815a-14ef0b46c0a4

