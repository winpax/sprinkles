# use PowerShell instead of sh:
set shell := ["pwsh.exe", "-NoProfile",  "-c"]

setup:
    just pre-commit

pre-commit:
    pre-commit install
    pre-commit install --hook-type commit-msg
    pre-commit install --hook-type pre-push
