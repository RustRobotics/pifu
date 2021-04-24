Name "Hello Installer"
RequestExecutionlevel dmin
Unicode True
InstallDir $PROGRAMFILES64\HelloInstaller
Page Directory
Page instfiles

Section ""
  SetOutPath $INSTDIR
  File "hello.exe"
SectionEnd
