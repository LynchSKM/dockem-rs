#!/bin/bash

# We'll first run the CURL script to pull the binary
# We'll pipe this straight into bash

curl -s https://raw.githubusercontent.com/LynchSKM/dockem-rs/main/scripts/get_dockem_local.sh | bash
sudo mv dockem-rs /usr/local/bin/dockem-rs

