# A FOSS Dropbox client
## Purpose
This project is motivated mainly by my realization that new Dropbox features could be implemented client-side
without the need for Dropbox the company to get around to it.  The killer feature I needed is support for a
.dropboxignore file that could dynamically exclude files from being synced.  This is a must for developers who
have build artifacts that are hundreds if not thousands of times larger than their source code.  Thus I want
to explore what this looks like if we were to design a client (probably in Rust, to be honest) that is
community-driven and has advanced features to squeeze every last bit of functionality out of the platform
that we can.

## Design
* Bindings to REST API
	* The actual business of serializing and deserializing for the API will be handled in its own module
	to separate concerns, since there is no official SDK for Rust.
* Option #1
	* Separate essentially all functionality into four separate domains:
	* Filesystem monitoring - watch the filesystem for changes and open files
	* Remote monitoring - watch the Dropbox folder for changes
	* Change handlers - Given a certain type of change detected, execute the necessary sync operations
	* Settings window / handler
* Option #2
	* Integrate the change handlers into the two other main subsystems
	* Local handler - watch the local filesystem, and execute a sync if needed
	* Remote handler - watch the remote filesystem, and write to disk if neededA
	* Settings window / handler
* I like Option #1 better because it allows one to implement more complex behaviors (like my ignore behavior)
in the middle layer, abstracted away from all of the nitty-gritty of either side.
* On the downside, Option #1 likely leads to a decent bit more code for not much immediate benefit.  Things like
ignoring files could be done just on the filesystem side of Option #2 just fine.  It would just likely be more
integrated into the filesystem monitoring.
