$content = Get-Content -Path "C:\Users\mummy\.openclaw\workspace\src\frontend\parser\top_level.rs" -Raw
$content = $content -replace '    // Parse attributes\r?\n    // TODO: Re-enable attribute parsing when parse_attributes is fixed\r?\n    // let \(input, attrs\) = parse_attributes\(input\)\?;\r?\n    let attrs = vec!\[\];', '    // Parse attributes
    let (input, attrs) = parse_attributes(input)?;'
Set-Content -Path "C:\Users\mummy\.openclaw\workspace\src\frontend\parser\top_level.rs" -Value $content