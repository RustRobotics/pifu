
;; UI
!include "MUI2.nsh"
!include "x64.nsh"
!include "Sections.nsh"
!include "LogicLib.nsh"
!include "Memento.nsh"
!include "WordFunc.nsh"
!include "Util.nsh"
!include "Integration.nsh"

;; Defines
!define VERSION "1.0.0"
!define PRODUCT_NAME "Hello ${VERSION}"
!define APPNAME "hello"
!define MUI_FOLDER "CustomWindow"
!define COMPANYNAME "rs-builder.Ltd"
!define HELPURL "https://biofan.org"
!define /date PRODUCT_DATE %Y%m%d

;; General
Name "${PRODUCT_NAME}"
Caption "${PRODUCT_NAME} Setup"
Icon "install.ico"
UninstallIcon "uninstall.ico"
Unicode True
SetCompressor /SOLID lzma
OutFile "CustomWindow-${VERSION}-x86_64-${PRODUCT_DATE}.exe"
RequestExecutionlevel admin
InstallDir $PROGRAMFILES64\HelloInstaller

ShowInstDetails show
ShowUninstDetails show
BrandingText "${HELPURL}"

;Interface Configuration
!define MUI_ICON "install.ico"
!define MUI_UNICON "uninstall.ico"
!define MUI_HEADERIMAGE
!define MUI_HEADERIMAGE_BITMAP "branding.bmp"
!define MUI_WELCOMEFINISHPAGE_BITMAP "branding.bmp"
!define MUI_UNWELCOMEFINISHPAGE_BITMAP "branding.bmp"
!define MUI_ABORTWARNING

; Pages
!define MUI_WELCOMEPAGE_TITLE "Welcome to the ${PRODUCT_NAME} Setup Wizard"
!define MUI_WELCOMEPAGE_TEXT "This wizard will guide you through the installation of ${PRODUCT_NAME}"
!insertmacro MUI_PAGE_WELCOME

!insertmacro MUI_PAGE_LICENSE "LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

!define MUI_FINISHPAGE_NOAUTOCLOSE
!define MUI_FINISHPAGE_RUN "$INSTDIR\${APPNAME}.exe"
!define MUI_FINISHPAGE_RUN_CHECKED
!define MUI_FINISHPAGE_RUN_TEXT "Run ${PRODUCT_NAME}"
!define MUI_FINISHPAGE_RUN_FUNCTION "LaunchLink"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_COMPONENTS
!insertmacro MUI_UNPAGE_CONFIRM

!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

;Installer section
Section "Install" SecFiles
  SetOverwrite on
  SetOutPath "${INSTDIR}"
  File "${APPNAME}.exe"

  ; Create desktop shortcut
  CreateShortCut "$DESKTOP\${PRODUCT_NAME}.lnk" "$INSTDIR\${APPNAME}.exe" ""

  ;Store installation folder
  WriteRegStr HKCU "Software\${PRODUCT_NAME}" "" $INSTDIR
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "DisplayName" "${PRODUCT_NAME}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "UninstallString" "$\"$INSTDIR\Uninstall.exe$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "QuietUninstallString" "$\"$INSTDIR\Uninstall.exe$\" /S"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "InstallLocation" "$\"$INSTDIR$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "DisplayIcon" "$\"$INSTDIR\${APPNAME}.exe$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "Publisher" "$\"${COMPANYNAME}$\""
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\${COMPANYNAME} ${APPNAME}" "HelpLink" "$\"${HELPURL}$\""

  ; Create uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"
SectionEnd

Section "Uninstall"
  Delete $INSTDIR\${APPNAME}.exe
SectionEnd


Function LaunchLink
  ExecShell "" "$INSTDIR\${APPNAME}.exe"
FunctionEnd

Function .onInit
FunctionEnd

Function un.onInit
FunctionEnd
