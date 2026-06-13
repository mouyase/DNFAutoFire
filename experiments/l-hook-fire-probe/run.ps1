$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8

function Text($codes) {
    $chars = foreach ($code in $codes.Split(' ')) {
        if ($code.Length -gt 0) { [char][Convert]::ToInt32($code, 16) }
    }
    -join $chars
}

function Say($codes) {
    Write-Host (Text $codes)
}

function Ask($codes) {
    Read-Host (Text $codes)
}

function Ask-SendMode {
    Say "8bf7 9009 62e9 53d1 9001 65b9 5f0f ff1a"
    Say "0020 0020 0031 002e 0020 0073 0063 0061 006e 0063 006f 0064 0065 ff08 626b 63cf 7801 ff09"
    Say "0020 0020 0032 002e 0020 0076 006b ff08 865a 62df 952e ff09"
    Say "0020 0020 0033 002e 0020 006d 0069 0078 0065 0064 ff08 865a 62df 952e 002b 626b 63cf 7801 ff09"
    Say "0020 0020 0034 002e 0020 0067 0061 006d 0065 ff08 65e7 7248 6e38 620f 6a21 5f0f 003a 0020 0076 006b 0046 0046 002b 626b 63cf 7801 ff09"
    $sendChoice = Ask "8bf7 8f93 5165 0020 0031 002f 0032 002f 0033 002f 0034 002c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0034"
    if ($sendChoice -eq "2") { return "vk" }
    if ($sendChoice -eq "3") { return "mixed" }
    if ($sendChoice -eq "1") { return "scancode" }
    return "game"
}

try {
    $scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $repoRoot = Resolve-Path (Join-Path $scriptDir "..\..")
    Set-Location $repoRoot

    $cargoBin = Join-Path $env:USERPROFILE ".cargo\bin"
    $env:Path = "$cargoBin;$env:Path"

    Write-Host "========================================"
    Say "0044 004e 0046 0020 004c 0020 952e 0020 0048 006f 006f 006b 0020 8fde 53d1 5b9e 9a8c"
    Write-Host "========================================"
    Write-Host ""
    Say "5982 679c 0020 0044 004e 0046 0020 662f 7ba1 7406 5458 6743 9650 8fd0 884c 002c 8bf7 53f3 952e 0020 0072 0075 006e 002e 0062 0061 0074 0020 9009 62e9 201c 4ee5 7ba1 7406 5458 8eab 4efd 8fd0 884c 201d 3002"
    Write-Host ""
    Say "8bf7 9009 62e9 8fd0 884c 6a21 5f0f ff1a"
    Say "0020 0020 0031 002e 0020 53ea 62e6 622a 4e0d 8fde 53d1 ff08 63a8 8350 5148 6d4b 8fd9 4e2a ff09"
    Say "0020 0020 0032 002e 0020 62e6 622a 7269 7406 0020 004c 002c 5e76 8fde 53d1 0020 0073 0079 006e 0074 0068 0065 0074 0069 0063 0020 004c"
    Say "0020 0020 0033 002e 0020 89c2 5bdf 6240 6709 6309 952e ff08 4e0d 62e6 622a 3001 4e0d 8fde 53d1 ff09"
    Say "0020 0020 0034 002e 0020 5012 8ba1 65f6 540e 76f4 63a5 53d1 9001 0020 004c ff08 4e0d 9700 8981 6309 4f4f 0020 004c ff09"
    Say "0020 0020 0035 002e 0020 6781 901f 8fde 53d1 ff08 0067 0061 006d 0065 6a21 5f0f ff0c 9ed8 8ba4 0031 006d 0073 002f 0031 006d 0073 ff09"
    Say "0020 0020 0036 002e 0020 0036 0030 6b21 002f 79d2 7a33 5b9a 0044 0065 006d 006f ff08 0067 0061 006d 0065 6a21 5f0f ff0c 0038 006d 0073 002f 0038 006d 0073 ff09"
    Write-Host "  7. J/L/H multi-key Demo (later pressed fires first, game mode, 60/s, 8ms/8ms)"
    Write-Host ""

    $choice = Ask "8bf7 8f93 5165 0020 0031 002f 0032 002f 0033 002f 0034 002f 0035 002f 0036 002f 0037 002c 7136 540e 56de 8f66"

    if ($choice -eq "1") {
        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 53ea 62e6 622a 4e0d 8fde 53d1 6a21 5f0f"
        Say "5207 5230 0020 0044 004e 0046 0020 540e 6309 4f4f 0020 004c ff1b 5982 679c 0020 0068 006f 006f 006b 0020 6709 6548 ff0c 0044 004e 0046 0020 4e0d 5e94 8be5 91ca 653e 0020 004c 0020 6280 80fd 3002"
        Say "6309 0020 0043 0074 0072 006c 002b 0043 0020 53ef 4ee5 9000 51fa 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode suppress-only
        exit $LASTEXITCODE
    }

    if ($choice -eq "2") {
        Write-Host ""
        $sendMode = Ask-SendMode
        Write-Host ""
        $rate = Ask "8bf7 8f93 5165 8fde 53d1 9891 7387 002f 79d2 ff0c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0031 0032 0030"
        if ([string]::IsNullOrWhiteSpace($rate)) { $rate = "120" }

        $downMs = Ask "8bf7 8f93 5165 6309 4e0b 4fdd 6301 6beb 79d2 6570 ff0c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0032"
        if ([string]::IsNullOrWhiteSpace($downMs)) { $downMs = "2" }

        $upMs = Ask "8bf7 8f93 5165 62ac 8d77 540e 7b49 5f85 6beb 79d2 6570 ff0c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0036"
        if ([string]::IsNullOrWhiteSpace($upMs)) { $upMs = "6" }

        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 62e6 622a 7269 7406 0020 004c 0020 002b 0020 8fde 53d1 0020 0073 0079 006e 0074 0068 0065 0074 0069 0063 0020 004c"
        Write-Host "send-mode=$sendMode, rate=$rate, down-ms=$downMs, up-ms=$upMs"
        Say "5207 5230 0020 0044 004e 0046 0020 540e 6309 4f4f 0020 004c 3002 6309 0020 0043 0074 0072 006c 002b 0043 0020 53ef 4ee5 9000 51fa 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode normal --send-mode $sendMode --rate $rate --down-ms $downMs --up-ms $upMs
        exit $LASTEXITCODE
    }

    if ($choice -eq "3") {
        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 89c2 5bdf 6240 6709 6309 952e 6a21 5f0f"
        Say "8fd9 4e2a 6a21 5f0f 4e0d 62e6 622a 4efb 4f55 6309 952e ff0c 4e5f 4e0d 8fde 53d1 3002"
        Say "8bf7 5148 5728 8bb0 4e8b 672c 6216 0050 006f 0077 0065 0072 0053 0068 0065 006c 006c 7a97 53e3 6309 0041 3001 004c 3001 7a7a 683c ff0c 770b 7edf 8ba1 662f 5426 589e 52a0 3002"
        Say "6309 0020 0043 0074 0072 006c 002b 0043 0020 53ef 4ee5 9000 51fa 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode observe-all
        exit $LASTEXITCODE
    }

    if ($choice -eq "4") {
        Write-Host ""
        $sendMode = Ask-SendMode
        $count = Ask "8bf7 8f93 5165 76f4 63a5 53d1 9001 6b21 6570 ff0c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0032 0030"
        if ([string]::IsNullOrWhiteSpace($count)) { $count = "20" }
        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 5012 8ba1 65f6 540e 76f4 63a5 53d1 9001 0020 004c"
        Write-Host "send-mode=$sendMode, test-count=$count"
        Say "8bf7 5728 0033 79d2 5185 5207 5230 8bb0 4e8b 672c 6216 0044 004e 0046 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode test-send --send-mode $sendMode --test-count $count
        exit $LASTEXITCODE
    }

    if ($choice -eq "5") {
        Write-Host ""
        $downMs = Ask "8bf7 8f93 5165 6309 4e0b 4fdd 6301 6beb 79d2 6570 ff0c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0031"
        if ([string]::IsNullOrWhiteSpace($downMs)) { $downMs = "1" }

        $upMs = Ask "8bf7 8f93 5165 62ac 8d77 540e 7b49 5f85 6beb 79d2 6570 ff0c 76f4 63a5 56de 8f66 9ed8 8ba4 0020 0031"
        if ([string]::IsNullOrWhiteSpace($upMs)) { $upMs = "1" }

        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 6781 901f 8fde 53d1"
        Write-Host "send-mode=game, max-speed=true, down-ms=$downMs, up-ms=$upMs"
        Say "5207 5230 0020 0044 004e 0046 0020 540e 6309 4f4f 0020 004c 3002 6309 0020 0043 0074 0072 006c 002b 0043 0020 53ef 4ee5 9000 51fa 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode normal --send-mode game --max-speed --down-ms $downMs --up-ms $upMs
        exit $LASTEXITCODE
    }

    if ($choice -eq "6") {
        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 0036 0030 6b21 002f 79d2 7a33 5b9a 0044 0065 006d 006f"
        Write-Host "send-mode=game, rate=60, down-ms=8, up-ms=8"
        Say "5207 5230 0020 0044 004e 0046 0020 540e 6309 4f4f 0020 004c 3002 6309 0020 0043 0074 0072 006c 002b 0043 0020 53ef 4ee5 9000 51fa 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode normal --send-mode game --rate 60 --down-ms 8 --up-ms 8
        exit $LASTEXITCODE
    }

    if ($choice -eq "7") {
        Write-Host ""
        Say "6b63 5728 542f 52a8 ff1a 004a 002f 004c 002f 0048 0020 591a 952e 0044 0065 006d 006f"
        Write-Host "send-mode=game, rate=60, down-ms=8, up-ms=8, rule=later pressed fires first"
        Say "5207 5230 0020 0044 004e 0046 0020 540e 6309 4f4f 0020 004a 002f 004c 002f 0048 3002 540e 6309 4e0b 7684 952e 4f1a 5728 4e0b 4e00 4e2a 53d1 9001 5468 671f 5148 89e6 53d1 3002"
        Say "6309 0020 0043 0074 0072 006c 002b 0043 0020 53ef 4ee5 9000 51fa 3002"
        Write-Host ""
        cargo run --manifest-path "experiments\l-hook-fire-probe\Cargo.toml" -- --mode normal --send-mode game --multi-jlh --rate 60 --down-ms 8 --up-ms 8
        exit $LASTEXITCODE
    }

    Write-Host ""
    Say "8f93 5165 65e0 6548 ff0c 5df2 9000 51fa 3002"
} catch {
    Write-Host ""
    Say "542f 52a8 5931 8d25 ff0c 9519 8bef 4fe1 606f ff1a"
    Write-Host $_.Exception.Message
    exit 1
}
