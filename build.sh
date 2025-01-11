#
# Nix support for easy compiling is partially dropped
# due to certain mingw32 packages (required for Windows compiling)
# are broken.
#
# Please send issues/pull requests about them to their respective
# creators.
#
if [ -d /etc/nix/ ]; then
    echo """
Cross-compiling with Nix is broken thus is not supported
You can still build for linux with \"cargo build --release\"
    """
    exit
fi

# Builds for both Windows and Linux
cargo build --release --target x86_64-pc-windows-gnu \
                    --target x86_64-unknown-linux-gnu 