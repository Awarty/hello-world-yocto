# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get hello-world-yocto could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/hello-world-yocto/0.1.0"
SRC_URI += "git://git@github.com/Awarty/hello-world-yocto.git;protocol=ssh;nobranch=1;branch=main"
SRCREV = "3bf4bf40ab0a86218bcc7760fe5ae210a6ee81bb"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+3bf4bf40ab"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
    crate://crates.io/autocfg/1.1.0 \
    crate://crates.io/bitflags/1.3.2 \
    crate://crates.io/cc/1.0.78 \
    crate://crates.io/cfg-if/1.0.0 \
    crate://crates.io/foreign-types-shared/0.1.1 \
    crate://crates.io/foreign-types/0.3.2 \
    crate://crates.io/libc/0.2.139 \
    crate://crates.io/once_cell/1.17.0 \
    crate://crates.io/openssl-macros/0.1.0 \
    crate://crates.io/openssl-src/111.24.0+1.1.1s \
    crate://crates.io/openssl-sys/0.9.80 \
    crate://crates.io/openssl/0.10.45 \
    crate://crates.io/pkg-config/0.3.26 \
    crate://crates.io/proc-macro2/1.0.50 \
    crate://crates.io/quote/1.0.23 \
    crate://crates.io/syn/1.0.107 \
    crate://crates.io/unicode-ident/1.0.6 \
    crate://crates.io/vcpkg/0.2.15 \
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
