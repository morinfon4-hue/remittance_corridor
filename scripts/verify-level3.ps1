param(
  [switch]$SkipFrontendInstall,
  [switch]$SkipReadmeCheck
)

$ErrorActionPreference = "Stop"

function Assert-Path {
  param(
    [string]$PathValue,
    [string]$Label
  )

  if (-not (Test-Path $PathValue)) {
    throw "Missing required $Label at $PathValue"
  }

  Write-Host "OK: $Label" -ForegroundColor Green
}

function Run-Step {
  param(
    [string]$Name,
    [scriptblock]$Command
  )

  Write-Host ""
  Write-Host "==> $Name" -ForegroundColor Cyan

  & $Command

  if ($LASTEXITCODE -ne 0) {
    throw "$Name failed with exit code $LASTEXITCODE"
  }
}

Write-Host "=== remittance_corridor Level 3 Verification ===" -ForegroundColor Cyan

Assert-Path "Cargo.toml" "root Cargo.toml"
Assert-Path "contracts/remittance_corridor/Cargo.toml" "contract Cargo.toml"
Assert-Path "contracts/remittance_corridor/src/lib.rs" "contract source"
Assert-Path "frontend/package.json" "frontend package.json"
Assert-Path "scripts/deploy-and-save.ps1" "deploy script"
Assert-Path ".github/workflows/ci.yml" "GitHub Actions workflow"
Assert-Path "CONTRACT_ID.txt" "contract id file"
Assert-Path "DEPLOYMENT.md" "deployment notes"
Assert-Path "README.md" "README"
Assert-Path "docs/ARCHITECTURE.md" "architecture docs"
Assert-Path "docs/QUALITY_AND_VERIFICATION.md" "quality docs"
Assert-Path "docs/DEPLOYMENT_NOTES.md" "deployment docs"
Assert-Path "docs/LOCAL_RUN_GUIDE.md" "local run guide"
Assert-Path "evidence/SUBMISSION_CHECKLIST.md" "submission checklist"
Assert-Path "vercel.json" "Vercel config"

if (-not $SkipReadmeCheck) {
  Write-Host ""
  Write-Host "==> README format check" -ForegroundColor Cyan

  $Blocked = Select-String -Path README.md -Pattern "~~~|```|├|└|│" -ErrorAction SilentlyContinue

  if ($Blocked) {
    throw "README format check failed. Remove code fences or Unicode tree characters."
  }

  Write-Host "OK: README format check passed" -ForegroundColor Green
}

Run-Step "Run Soroban contract tests" {
  cargo test --workspace
}

Run-Step "Build Soroban WASM" {
  cargo build --workspace --target wasm32v1-none --release
}

Push-Location frontend

try {
  if (-not $SkipFrontendInstall) {
    Run-Step "Install frontend dependencies" {
      npm install
    }
  }

  Run-Step "Run frontend tests" {
    npm test
  }

  Run-Step "Build frontend" {
    npm run build
  }
}
finally {
  Pop-Location
}

Write-Host ""
Write-Host "=== remittance_corridor Level 3 verification completed successfully ===" -ForegroundColor Green
