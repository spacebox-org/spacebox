# A FOSS Dropbox client
## Motivation
I realized recently that Dropbox has an open enough API to allow outside developers to create fully featured
sync clients that are potentially better than the official ones.  There were a couple of really desireable
features that Dropbox hasn't implemented over the years, so I see no reason we can't do it.
## Purpose
This project will provide a community-driven, fully-featured Dropbox client for Linux (initially) and
potentially other platforms.

## Key Features
* New features beyond official
	* .dropboxignore support
* Standard sync client features
	* Selective sync
	* Sync-on-change
	* Dropbox folder relocation
	* Either personal and business accounts
	* HTTP Proxies
	* Autostart

There are some standard client features that have more dubious value such as LAN sync and bandwidth
limits, so some consideration will have to be made before spending the time to implement those.

* Some side perks that I hope to realize
	* Significantly lower memory usage
	* Reduced hard drive writes

* Potential further features that could be added
	* Symbolic link support

## Design
* Rust Dropbox SDK
	* Developed in adjacent repositories
* File system monitoring
	* Responsible for tracking when a file changes
* Change handlers
	* Make decisions about how to map events in the file 
	system layer to actions in the Dropbox interfacing layer.
* Dropbox monitoring and interface
	* Responsible for sending and receiving things from Dropbox
	* Uses the Rust Dropbox SDK mentioned above
* This structure allows each layer to be tested relatively independently, so that is a plus.
