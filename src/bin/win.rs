use std::process::Command;

fn main() {
    let lnk_path = "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Docker Desktop.lnk";

    let script = format!(
        r#"
        function Get-Shortcut {{
            param (
                [string]$Path
            )
            
            $shell = New-Object -ComObject WScript.Shell
            $shortcut = $shell.CreateShortcut($Path)
            
            $properties = @{{
                TargetPath = $shortcut.TargetPath
                Arguments  = $shortcut.Arguments
                Description = $shortcut.Description
                Hotkey = $shortcut.Hotkey
                IconLocation = $shortcut.IconLocation
                WindowStyle = $shortcut.WindowStyle
                WorkingDirectory = $shortcut.WorkingDirectory
            }}
            
            return [PSCustomObject]$properties
        }}

        Get-Shortcut -Path "{}" | ConvertTo-Json
    "#,
        lnk_path
    );

    let output = Command::new("powershell")
        .arg("-Command")
        .arg(script)
        .output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    // let result: PowerShellLnkParseResult = serde_json::from_str(&output).unwrap();
    println!("{}", output.to_string());
}
