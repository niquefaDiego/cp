param (
    [string]$TestCase = "0"
)
Write-Host "TestCase='${TestCase}'"
cargo build
Get-Content data\$TestCase.in | .\target\debug\solution.exe | Out-File -FilePath data\TestCase.out -Encoding utf8
cargo run --bin check $TestCase
