param(
  [string]$ContractIdFile = "CONTRACT_ID.txt"
)

$ErrorActionPreference = "Stop"

Write-Host "=== remittance_corridor deployment record ===" -ForegroundColor Cyan

if (-not (Test-Path $ContractIdFile)) {
  throw "Missing CONTRACT_ID.txt. Deploy the contract first or add the deployed testnet contract ID."
}

$ContractId = (Get-Content $ContractIdFile -Raw).Trim()

if ($ContractId -eq "") {
  throw "CONTRACT_ID.txt is empty."
}

$Content = @(
  "# Deployment Notes",
  "",
  "remittance_corridor is deployed on Stellar testnet.",
  "",
  "<pre>",
  "Network: Stellar testnet",
  "Contract ID: $ContractId",
  "Explorer: https://stellar.expert/explorer/testnet/contract/$ContractId",
  "</pre>",
  "",
  "The deployed contract ID is also stored in CONTRACT_ID.txt."
)

$Content | Set-Content DEPLOYMENT.md -Encoding UTF8

Write-Host "Deployment notes updated for $ContractId" -ForegroundColor Green
