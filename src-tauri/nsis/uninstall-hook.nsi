; Wird nach der Deinstallation ausgeführt
; Löscht den AppData-Ordner NUR bei vollständiger Deinstallation (nicht bei Updates)

!macro NSIS_HOOK_POSTUNINSTALL
    ; $UpdateMode = 1 bedeutet Update, nicht vollständige Deinstallation
    ${If} $UpdateMode <> 1
        RMDir /r "$LOCALAPPDATA\SF Hydra Advisor"
    ${EndIf}
!macroend
