if (-not (Test-Path build)) {
    New-Item -ItemType Directory -Force -Path build
}
if (-not (Test-Path build/obj)) {
    New-Item -ItemType Directory -Force -Path build/obj
}
if (-not (Test-Path build/bin)) {
    New-Item -ItemType Directory -Force -Path build/bin
}
if (-not (Test-Path build/lib)) {
    New-Item -ItemType Directory -Force -Path build/lib
}

make --makefile=win32/Makefile --debug=b $args

# Remove empty directories using robocopy silently
robocopy build build /S /move /NJH /NJS /NDL > $null 2>&1
