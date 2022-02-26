#!/bin/bash
SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && (pwd -W 2>/dev/null || pwd))

(cd $SCRIPT_DIR && export DOTNET_ReadyToRun=0 DOTNET_TC_QuickJitForLoops=1 DOTNET_TieredPGO=1 && dotnet run --confiuration release)
