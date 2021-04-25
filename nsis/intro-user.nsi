
!define REGPATH_UNINSTSUBKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${NAME}"

Name "Hello Installer"
RequestExecutionlevel User
Unicode True
InstallDir "$LocalAppData\Programs\$(^Name)"
InstallDirRegKey HKCU "${REGPATH_UNINSTSUBKEY}" "UninstallString"

Page Directory
Page instfiles

Uninstpage UninstConfirm
Uninstpage InstFiles

Function .onInit
  SetShellVarContext Current

;  ${If} $InstDir == "" ; No /D= nor InstallDirRegKey?
;    GetKnownFolderPath $InstDir ${FOLDERID_UserProgramFiles} ; This folder only exists on Win7+
;    StrCmp $InstDir "" 0 +2
;    StrCpy $InstDir "$LocalAppData\Programs" ; Fallback directory
;
;    StrCpy $InstDir "$InstDir\$(^Name)"
;  ${EndIf}
FunctionEnd

Function un.onInit
  SetShellVarContext Current
FunctionEnd


Section "Program files (Required)"
  SectionIn Ro

  SetOutPath $InstDir
  WriteUninstaller "$InstDir\Uninst.exe"
  WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "DisplayName" "${NAME}"
  WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "DisplayIcon" "$InstDir\MyApp.exe,0"
  WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "UninstallString" '"$InstDir\Uninst.exe"'
  WriteRegStr HKCU "${REGPATH_UNINSTSUBKEY}" "QuietUninstallString" '"$InstDir\Uninst.exe" /S'
  WriteRegDWORD HKCU "${REGPATH_UNINSTSUBKEY}" "NoModify" 1
  WriteRegDWORD HKCU "${REGPATH_UNINSTSUBKEY}" "NoRepair" 1

  !tempfile APP
  File "/oname=$InstDir\MyApp.exe" "${APP}" ; Pretend that we have a real application to install
  !delfile "${APP}"
SectionEnd


Section ""
  SetOutPath $INSTDIR
  File "hello.exe"
SectionEnd


Section -Uninstall
  Delete "$InstDir\Uninst.exe"
  RMDir "$InstDir"
  DeleteRegKey HKCU "${REGPATH_UNINSTSUBKEY}"
SectionEnd

