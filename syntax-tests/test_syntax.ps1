# PowerShell Script Syntax Test

<#
.SYNOPSIS
    PowerShell syntax highlighting test
.DESCRIPTION
    Comprehensive test of PowerShell language features
#>

#Requires -Version 5.1

# Variables
$Name = "World"
$Count = 42
$Flag = $true
$Null = $null

# Arrays
$Fruits = @("Apple", "Banana", "Orange")
$Numbers = 1..10

# Hash tables
$Person = @{
    Name = "Alice"
    Age  = 30
    City = "Seattle"
}

# Here-strings
$MultiLine = @"
This is a
multi-line
string
"@

$SingleQuote = @'
No variable expansion: $Name
'@

# Functions
function Get-Greeting {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name,
        
        [ValidateRange(0, 120)]
        [int]$Age = 0
    )
    
    Write-Output "Hello, $Name! You are $Age years old."
}

# Advanced function
function Invoke-Process {
    [CmdletBinding()]
    param(
        [Parameter(ValueFromPipeline = $true)]
        [string[]]$InputObject
    )
    
    begin {
        Write-Verbose "Starting process"
    }
    
    process {
        foreach ($item in $InputObject) {
            Write-Output "Processing: $item"
        }
    }
    
    end {
        Write-Verbose "Completed"
    }
}

# Control flow
if ($Count -gt 10) {
    Write-Host "Greater than 10"
} elseif ($Count -eq 10) {
    Write-Host "Equal to 10"
} else {
    Write-Host "Less than 10"
}

# Switch statement
switch ($Count) {
    { $_ -lt 10 } { Write-Host "Small" }
    { $_ -lt 50 } { Write-Host "Medium" }
    { $_ -ge 50 } { Write-Host "Large" }
    default { Write-Host "Unknown" }
}

# Loops
for ($i = 0; $i -lt 5; $i++) {
    Write-Host "Index: $i"
}

foreach ($fruit in $Fruits) {
    Write-Host "Fruit: $fruit"
}

$Fruits | ForEach-Object {
    Write-Host "Item: $_"
}

while ($Count -gt 0) {
    $Count--
    if ($Count -eq 35) { break }
}

# Try-catch-finally
try {
    Get-Item "C:\NonExistent" -ErrorAction Stop
} catch [System.IO.FileNotFoundException] {
    Write-Warning "File not found"
} catch {
    Write-Error "An error occurred: $_"
} finally {
    Write-Host "Cleanup"
}

# Pipeline
Get-ChildItem -Path "C:\Windows" -Filter "*.exe" |
    Where-Object { $_.Length -gt 1MB } |
    Sort-Object Length -Descending |
    Select-Object Name, Length -First 10 |
    Format-Table -AutoSize

# Operators
$result = (5 + 3) * 2
$comparison = $Count -eq 42
$match = "Hello" -like "H*"
$contains = $Fruits -contains "Apple"

# Type casting
[int]$IntValue = "123"
[datetime]$Date = "2026-02-01"

# Splatting
$params = @{
    Name        = "Alice"
    Age         = 30
    ErrorAction = "Stop"
}
Get-Greeting @params

# Jobs
Start-Job -ScriptBlock {
    Start-Sleep -Seconds 5
    Get-Date
}

Get-Job | Wait-Job | Receive-Job

# Modules
Import-Module ActiveDirectory -ErrorAction SilentlyContinue

# Classes (PowerShell 5+)
class Vehicle {
    [string]$Make
    [string]$Model
    [int]$Year
    
    Vehicle([string]$make, [string]$model, [int]$year) {
        $this.Make = $make
        $this.Model = $model
        $this.Year = $year
    }
    
    [string] ToString() {
        return "$($this.Year) $($this.Make) $($this.Model)"
    }
}

$car = [Vehicle]::new("Tesla", "Model 3", 2024)

# Regular expressions
if ("test@example.com" -match '[\w]+@[\w]+\.[\w]+') {
    Write-Host "Valid email"
}

# Calculated properties
Get-Process | Select-Object Name,
    @{Name="MemoryMB"; Expression={$_.WS / 1MB}} |
    Format-Table
