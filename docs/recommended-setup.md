# Recommended setup

Nerminal draws what your shell prints. Most of what makes a terminal pleasant to
use comes from the shell, and none of it ships with the app.

Four steps. Only two of them install anything.

## 1. zsh

macOS has shipped zsh as the default login shell since Catalina, so there is
probably nothing to install. Check:

```
echo $SHELL     # /bin/zsh
zsh --version   # zsh 5.9
```

If the first line says bash, switch and open a new window:

```
chsh -s /bin/zsh
```

Why not stay on bash: zsh completes arguments and flags and not only file names,
cycles through the candidates with repeated Tab, corrects a mistyped path,
shares history across windows, and has the plugin ecosystem the rest of this
page is built on. A stock zsh gives you almost none of that on its own, which is
what step 2 is for.

More at [zsh.org](https://www.zsh.org).

## 2. Prezto

Prezto configures zsh for you. Completion, syntax highlighting as you type,
suggestions drawn from history, git state in the prompt, all as modules you
switch on in one file.

Clone it:

```
git clone --recursive https://github.com/sorin-ionescu/prezto.git "${ZDOTDIR:-$HOME}/.zprezto"
```

Link the configuration files it provides. This fails if you already have any of
them, so move yours aside first:

```zsh
setopt EXTENDED_GLOB
for rcfile in "${ZDOTDIR:-$HOME}"/.zprezto/runcoms/^README.md(.N); do
  ln -s "$rcfile" "${ZDOTDIR:-$HOME}/.${rcfile:t}"
done
```

Open the new `~/.zpreztorc` and set the module list. The order matters:

```zsh
zstyle ':prezto:load' pmodule \
  'environment' \
  'terminal' \
  'editor' \
  'history' \
  'directory' \
  'spectrum' \
  'utility' \
  'completion' \
  'homebrew' \
  'osx' \
  'ssh' \
  'git' \
  'syntax-highlighting' \
  'history-substring-search' \
  'autosuggestions' \
  'prompt'
```

Open a new window. All of the above is already working.

More at [sorin-ionescu/prezto](https://github.com/sorin-ionescu/prezto).

## 3. Powerlevel10k

The prompt. It shows the current git branch and whether the tree is dirty, the
exit code of the last command when it failed, how long a slow command took, and
the context of the tooling on your path.

```
brew install powerlevel10k
```

Tell Prezto to leave the prompt alone, in `~/.zpreztorc`:

```zsh
zstyle ':prezto:module:prompt' theme 'off'
```

Load it from `~/.zshrc`, below the line that sources Prezto:

```zsh
source /opt/homebrew/opt/powerlevel10k/share/powerlevel10k/powerlevel10k.zsh-theme
[[ ! -f ~/.p10k.zsh ]] || source ~/.p10k.zsh
```

Then add the instant prompt at the very top of `~/.zshrc`, above anything that
writes to the console. It paints the prompt from a cache before the rest of the
file has run, so a new window is usable straight away:

```zsh
if [[ -r "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh" ]]; then
  source "${XDG_CACHE_HOME:-$HOME/.cache}/p10k-instant-prompt-${(%):-%n}.zsh"
fi
```

Open a new window and run `p10k configure`. It asks a dozen questions and writes
your answers to `~/.p10k.zsh`.

More at [romkatv/powerlevel10k](https://github.com/romkatv/powerlevel10k).

## 4. Fonts

Nothing to do. `p10k configure` opens by asking whether you can see a row of
icons, because that prompt needs a Nerd Font, a font patched with the extra
glyphs. Nerminal embeds JetBrains Mono Nerd Font and uses it by default, so the
answer to those questions is yes.

If you want a different font it also has to be a Nerd Font, or the icons come
out as empty boxes. Install one:

```
brew install --cask font-hack-nerd-font
```

Pick it under Settings > Appearance, or name it in `~/.nerminal/settings.toml`:

```toml
[appearance.text]
font_name = "Hack Nerd Font Mono"
```

The whole catalogue is at [nerdfonts.com](https://www.nerdfonts.com).

## 5. The tools that go with it

None of these are required. They are the ones worth the install.

```
brew install zsh-completions eza zoxide fzf bat fd jq yq direnv
```

| Tool | What it gives you |
| --- | --- |
| `zsh-completions` | Completion definitions for commands that ship none of their own |
| `eza` | `ls` with colours, icons and a tree mode |
| `zoxide` | A `cd` that learns the directories you actually use |
| `fzf` | Fuzzy finder, and Ctrl-R over your history stops being a guessing game |
| `bat` | `cat` with syntax highlighting and line numbers |
| `fd` | `find` with a syntax you can remember |
| `jq`, `yq` | Query and reshape JSON and YAML |
| `direnv` | Loads `.envrc` when you enter a directory and unloads it when you leave |

The completions have to reach `fpath` before Prezto runs `compinit`, so that line
goes above the block that sources Prezto:

```zsh
fpath=(/opt/homebrew/share/zsh-completions $fpath)
```

The rest goes below it:

```zsh
source <(fzf --zsh)
eval "$(zoxide init zsh)"
eval "$(direnv hook zsh)"

alias ls='eza --icons'
alias j='zi'
```

### Kubernetes

```
brew install kubecolor kubectx k9s stern
```

| Tool | What it gives you |
| --- | --- |
| `kubecolor` | `kubectl` with its output coloured, same flags and same behaviour |
| `kubectx` | Switch cluster and namespace by name instead of by flag |
| `k9s` | The whole cluster in a full screen view |
| `stern` | Tail the logs of several pods at once |

Put `kubecolor` in front of `kubectl` and keep the completions, which are
registered under the real name:

```zsh
alias kubectl=kubecolor
compdef kubecolor=kubectl
```

### Startup time

Three of the lines above run a binary and evaluate what it prints, on every
shell you open. That is a subprocess each, and it is the usual reason a new tab
takes a beat to appear. Cache the output and regenerate it only when the binary
is newer than the cache:

```zsh
function zsh_load_cached_init {
  local name="$1" bin="$2"
  shift 2
  local cache="${XDG_CACHE_HOME:-$HOME/.cache}/zsh/${name}.zsh"
  [[ -n "$commands[$bin]" ]] || return
  if [[ ! -s "$cache" || "$commands[$bin]" -nt "$cache" ]]; then
    mkdir -p "${cache:h}" && "$@" >| "$cache"
  fi
  source "$cache"
}

zsh_load_cached_init fzf    fzf    fzf --zsh
zsh_load_cached_init zoxide zoxide zoxide init zsh
zsh_load_cached_init direnv direnv direnv hook zsh
```
