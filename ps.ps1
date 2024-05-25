# Define a function to read .lnk file
function Get-Shortcut {
    param (
        [string]$Path
    )
    
    $shell = New-Object -ComObject WScript.Shell
    $shortcut = $shell.CreateShortcut($Path)
    
    $properties = @{
        TargetPath = $shortcut.TargetPath
        Arguments  = $shortcut.Arguments
        Description = $shortcut.Description
        Hotkey = $shortcut.Hotkey
        IconLocation = $shortcut.IconLocation
        WindowStyle = $shortcut.WindowStyle
        WorkingDirectory = $shortcut.WorkingDirectory
    }
    
    return [PSCustomObject]$properties
}

# Example usage
$lnkPath = "C:\ProgramData\Microsoft\Windows\Start Menu\Docker Desktop.lnk"
# $lnkPath = "C:\Users\shenh\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\CapCut\CapCut.lnk"
# $lnkPath = "C:\ProgramData\Microsoft\Windows\Start Menu\Programs\devclean-ui\devclean-ui.lnk"
# $lnkPath = "C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Visual Studio 2022.lnk"
$shortcutInfo = Get-Shortcut -Path $lnkPath

# Output the results as JSON
$shortcutInfo | ConvertTo-Json