param (
    [string]$TestCase = "0"
)

cargo build

if ($LASTEXITCODE -ne 0) {
    Write-Host "Cargo build failed with status code: $LASTEXITCODE"
    exit $LASTEXITCODE # Exit the PowerShell script with the same exit code
}

Get-Content data\$TestCase.in |`
    .\target\debug\solution.exe |`
    Out-File -FilePath data\$TestCase.out -Encoding utf8

if ($LASTEXITCODE -ne 0) {
    Write-Host "Solution failed with status code: $LASTEXITCODE"
    exit $LASTEXITCODE # Exit the PowerShell script with the same exit code
}

cargo run --bin check $TestCase

if ($LASTEXITCODE -ne 0) {
    Write-Host "Check failed with status code: $LASTEXITCODE"
    exit $LASTEXITCODE # Exit the PowerShell script with the same exit code
}
