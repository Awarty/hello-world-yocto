# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get hello-world-yocto could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/hello-world-yocto/0.1.0"
SRC_URI += "git://git@github.com/Awarty/hello-world-yocto.git;protocol=ssh;nobranch=1;branch=main"
SRCREV = "10d7e538ab094b0e00c6bed3a0d48a3285042c0a"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+10d7e538ab"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
"



# FIXME: update generateme with the real MD5 of the license file
LIC_FILES_CHKSUM = " \
    "

SUMMARY = "hello-world-yocto"
HOMEPAGE = "https://github.com/Awarty/hello-world-yocto"
LICENSE = "CLOSED"

# includes this file if it exists but does not fail
# this is useful for anything you may want to override from
# what cargo-bitbake generates.
include hello-world-yocto-${PV}.inc
include hello-world-yocto.inc
