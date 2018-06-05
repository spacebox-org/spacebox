# Minimum Viable Product
(TBD, but really important to get documented)

# File System Monitoring
Turns out, there's good-looking, cross-platform crate called `notify`
that seems to obviate most of the need to write our own method of keeping
track of file changes.  Thus the `fs` module as it stands is superflous.

The biggest known-unknown here is debouncing upload attempts to Dropbox, 
because that could amplify traffic pretty signficantly (even though 
`notify` does some level of debouncing itself). For instance, what if
10,000 files get changed in the span of a few seconds, but then 20 seconds
later, while the upload is still in progress, one of those files gets changed
three more times?  Do we report that to Dropbox as one single write, or
do we report each write and risk the upload amplification?

A high-priority, post-MVP goal would be to either back-contribute to the
`notify` crate to support more OS-specific backends beyond `inotify`, or
if needed, fork that project and do that development within Spacebox-org
as a seperate crate.

# Filter Layer / Router Layer

# Shipping and Receiving Layer