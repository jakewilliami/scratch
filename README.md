# Scratch

Open a scratch space to type in.

## What Is This?

Sometimes I need to write something down as quickly as I think of it, so that I can paste it elsewhere as needed.  Sometimes, the time between having the fleeting thought and opening the place to write it is enough for the thought to dilute or even vanish.

This is a common pattern for me, so I would use [Alfred](https://www.alfredapp.com/)'s overlay to write what I am thinking, copy it, close Alfred, and paste it where it needs to go.  This was convenient because it only took two keystrokes (my shortcut to bring up Alfred; <kbd>⌘ + ⌘</kbd>) before I could begin writing.  However, it would take _four or five_ keystrokes between writing and pasting (<kbd>⌘ + A</kbd>, <kbd>⌘ + X</kbd>, <kbd>ESC</kbd>).  We can reduce it to one.

The idea is simple but elegant to fit my needs: bring up a text overlay upon keyboard shortcut, type what I need to type, and when I close the window the contents of the buffer are copied to my clipboard.  That  is precisely what this program does.

## Quick Start

Run the program to start the process without installing it:

```command-line
$ just  # or `just run`
```

Alternatively, you can build it into a binary or bundle:

```commandline
$ just build
```

On macOS, you can build and install it to `/Applications/` and load it as a background process with `launchctl`:

```commandline
$ just install
```

If you have already done this but are actively developing it and wish to reinstall it, you can run:

```commandline
$ just reinstall
```

To uninstall it, run:

```commandline
$ just uninstall
```

### Controls

The controls are simple:
  - <kbd>⌘ + '</kbd> toggles the overlay;
  - <kbd>ESC</kbd> closes the overlay, copying the contents of the buffer to your clipboard.

## Development

Font asset obtained from here:

```
https://cdnjs.cloudflare.com/ajax/libs/juliamono/0.061/JuliaMono-Regular.woff2
```

## Caveats

This program was developed on macOS for macOS.  It will work on other platforms, but may not have the same user experience.

### Development History

Initially (pre-[`b240513`](https://github.com/jakewilliami/scratch/commit/b240513)), I tried using a GUI library called [Ply](https://github.com/TheRedDeveloper/ply-engine).  However, I found it a little immature for my needs (it's one of the newer libraries; I only found it because it was the one that was most recently added to [`areweguiyet.com`](https://areweguiyet.com/)).  (Note that this version of the project required the font to be decompressed to TTF, for which I used [`woff2_decompress`](https://stackoverflow.com/a/73733882).)  Even though I basically had a MVP, I found it caused too much friction to change things/add features, so I rewrote the app to use one of the big players (at time of writing, June 2026), [Dioxus](github.com/DioxusLabs/dioxus).  I chose Dioxus over [Tauri](https://github.com/tauri-apps/tauri) because it was almost pure Rust (except for CSS) and I couldn't be bothered writing HTML/Handlebars templates for such a sumple application.

Before this was a background application ([v1.0.0](https://github.com/jakewilliami/scratch/releases/tag/v1.0.0)), we had to define the shortcut on the system (outside of the application).  However, running the application from scratch (no pun intended) every time the shortcut activated was slow, which defeats the purpose of the whole application.  Now that it's constantly running in the background (as of [v2.0.0](https://github.com/jakewilliami/scratch/releases/tag/v2.0.0)), waiting for the keystroke to display the overlay, we do not need to mess around with [binding a keyboard shortcut to run an application via a new Automator service](https://apple.stackexchange.com/a/40887/366960) (like we are currently doing for the [totally not dead] project, [`mm`](https://github.com/jakewilliami/mm)).  The only requirement is to load it with the background launch agent (`launchctl`), which is decidedly simpler.

## Citation

If your research depends on this project, please consider giving us a formal citation: [`citation.bib`](./citation.bib).
