[Setup]
AppName=MathPlus
AppVersion=1.2.3
DefaultDirName={pf}\ZFordDev\MathPlus
DefaultGroupName=ZFordDev
OutputDir=.
OutputBaseFilename=MathPlusInstaller
SetupIconFile="{#SourcePath}\..\assets\icon.ico"
Compression=lzma
SolidCompression=yes

[Files]
; Main executable
Source: "{#SourcePath}\..\target\release\MathPlus.exe"; DestDir: "{app}"; Flags: ignoreversion

; Icon file
Source: "{#SourcePath}\..\assets\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\MathPlus"; Filename: "{app}\MathPlus.exe"; IconFilename: "{app}\icon.ico"
Name: "{commondesktop}\MathPlus"; Filename: "{app}\MathPlus.exe"; IconFilename: "{app}\icon.ico"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut"

[Run]
Filename: "{app}\MathPlus.exe"; Description: "Launch MathPlus"; Flags: nowait postinstall skipifsilent