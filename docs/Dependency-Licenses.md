# What is this document?
We need to keep a list of all the licenses we need to track
and attributions to be made.  That way when we get some time,
we can actually do justice to that as opposed to this provisional
thing.

# Tooling
Turns out there's a tool called `cargo-license` which will get you
the licenses of all of your dependencies to make it easy to ensure
you aren't running afoul of someone's rights.

# Notes
We need to avoid GPL and LGPL like the plague.  They are very difficult
from which to isolate onself.  Just take a look at Google and Bionic.
*They rewrote a libc for Android, just to insulate Android apps from
the GPL of Linux/glibc.*  Seriously, stay away, or we'll have to take
this code to something like GPLv3 which would be sad.