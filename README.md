> This repository is being archived in favor of a 4.0 gdextension version

# Godout
Godout is an addon that hopes to package all kinds of exports under one repository.

## Why?
Godot is one of the best game engines out there and has an excellent API that allows the users to create graphical tools as well. But one of the things that gets ignored is the fact that godot can't export to other file types like to fbx to mp4, to gif etc. Hence Godout

## Wouldn't this be a bulky package with a lot of bloat?
No, I have created the plugin in a way that you can select the codecs you want. Every codec installed would be available through `Godout.category.filetype` eg: `Godout.animation.gif`.

## What all are in the repo as of now?
As of now, the repo only contains encoders for Gif and Webp.
