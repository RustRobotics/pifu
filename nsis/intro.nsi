Name "Hello Installer"
RequestExecutionlevel admin
Unicode True
InstallDir $PROGRAMFILES64\HelloInstaller
Page Directory
Page instfiles

Section ""
  SetOutPath $INSTDIR
  File "hello.exe"
SectionEnd
