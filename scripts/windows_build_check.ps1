# PowerShell script to compile all packages on Windows
param(
    [string]$Target = "x86_64-pc-windows-msvc",
    [string]$OutputDir = "task/windows_compile_results"
)

$ErrorActionPreference = "Continue"
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

$packages = Get-ChildItem -Path "packages" -Directory
$results = @()

foreach ($pkg in $packages) {
    Write-Host "Checking $($pkg.Name)..." -ForegroundColor Cyan
    
    Push-Location $pkg.FullName
    $output = cargo check --target $Target 2>&1 | Out-String
    $exitCode = $LASTEXITCODE
    Pop-Location
    
    $results += [PSCustomObject]@{
        Package = $pkg.Name
        Success = ($exitCode -eq 0)
        ExitCode = $exitCode
        Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    }
    
    if ($exitCode -ne 0) {
        $output | Out-File -FilePath "$OutputDir/$($pkg.Name)_error.log"
        Write-Host "  FAILED" -ForegroundColor Red
    } else {
        Write-Host "  OK" -ForegroundColor Green
    }
}

# Generate JSON report
$results | ConvertTo-Json | Out-File -FilePath "$OutputDir/compilation_report.json"

# Generate Markdown summary
$summary = @"
# Windows Compilation Report
Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
Target: $Target

## Summary
- Total Packages: $($results.Count)
- Successful: $($results.Where{$_.Success}.Count)
- Failed: $($results.Where{!$_.Success}.Count)

## Results
$(foreach ($r in $results) {
    $status = if ($r.Success) { "✅" } else { "❌" }
    "- $status $($r.Package)"
})
"@

$summary | Out-File -FilePath "$OutputDir/README.md"

Write-Host "`nResults saved to $OutputDir" -ForegroundColor Yellow
