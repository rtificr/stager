# Stager (STAGeR)
Stager is a runtime environment that lets you interact with digital choose-your-own-adventure stories. 

Stories are stored in a unique file format (.ACT, or just "act") for ease of use with Stager. ".ACT" is essentially a simple markup language that represents an interactive story. For info on how to create your own stories, see the wiki. `//TODO: WIKI`

## Running
Currently, all act files that work on [Stager Legacy](https://github.com/rtificr/stager-legacy) will work on the new version. Once the syntax has been sufficiently changed, an act versioning system may be added to differentiate the supported syntaxes.

In order for an ACT file to be run, it must be located in the same directory as the Stager binary (usually stager.exe or something similar, depending on the operating system) unless you want to provide the whole path. To run an ACT file once it is in the correct location, type "`run <ACT PATH HERE>`", replacing "`ACT PATH HERE`" with either the name of the act or the full path to the act. In order to pick a choice, type the number associated with the choice into the console. 
