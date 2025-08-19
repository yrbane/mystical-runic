use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_basic_rendering() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for test files
    let dir = tempdir()?;
    let template_path = dir.path().join("template.rune");
    let data_path = dir.path().join("data.json");

    // Write a simple template file
    fs::write(&template_path, "<h1>{{ title }}</h1>")?;

    // Write a simple data file
    fs::write(&data_path, r#"{"title": "Magie Moderne"}"#)?;

    // Run the command
    let mut cmd = Command::cargo_bin("runic_templater")?;
    cmd.arg("--template").arg(&template_path)
       .arg("--data").arg(&data_path);

    // Assert that the command runs successfully and produces the expected output
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("<h1>Magie Moderne</h1>"));

    Ok(())
}

#[test]
fn test_complex_rendering() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let template_path = dir.path().join("template_complex.rune");
    let data_path = dir.path().join("data_complex.json");

    // Template with a loop
    fs::write(&template_path, r#"
<ul>
{{for item in items}}
    <li>{{item.name}} - {{item.value}}</li>
{{/for}}
</ul>
"#)?;

    // Data with an array of objects
    fs::write(&data_path, r#"
{
    "items": [
        {"name": "Apple", "value": 10},
        {"name": "Banana", "value": 20},
        {"name": "Cherry", "value": 30}
    ]
}
"#)?;

    let mut cmd = Command::cargo_bin("runic_templater")?;
    cmd.arg("--template").arg(&template_path)
       .arg("--data").arg(&data_path);

    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("<ul>")
                .and(predicate::str::contains("<li>Apple - 10</li>"))
                .and(predicate::str::contains("<li>Banana - 20</li>"))
                .and(predicate::str::contains("<li>Cherry - 30</li>"))
                .and(predicate::str::contains("</ul>"))
        );

    Ok(())
}

#[test]
fn test_full_featured_rendering() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("runic_templater")?;
    cmd.arg("--template").arg("templates/main.rune")
       .arg("--data").arg("data/main.json");

    cmd.assert()
        .success()
        .stdout(
            predicate::str::contains("<title>Chronicles of Eldoria</title>")
                .and(predicate::str::contains("<h1>\n            <svg class=\"magic-icon star\" viewBox=\"0 0 24 24\"><path d=\"M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z\"/></svg>\n            Chronicles of Eldoria\n            <svg class=\"magic-icon star\" viewBox=\"0 0 24 24\"><path d=\"M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z\"/></svg>\n        </h1>"))
                .and(predicate::str::contains("<p>Welcome, noble Lyra!</p>"))
                .and(predicate::str::contains("<h2>Your Arcane Power Level: 9500</h2>"))
                .and(predicate::str::contains("<p class=\"chosen-one\">\n            <svg class=\"magic-icon star\" viewBox=\"0 0 24 24\"><path d=\"M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z\"/></svg>\n            <strong>You are the chosen one, destined for greatness!</strong>\n            <svg class=\"magic-icon star\" viewBox=\"0 0 24 24\"><path d=\"M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z\"/></svg>\n        </p>"))
                .and(predicate::str::contains("<li>\n                <svg class=\"magic-icon orb\" viewBox=\"0 0 24 24\"><path d=\"M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z\"/></svg>\n                <strong>Celestial Orb</strong> - Cost: 100 mana.\n            </li>"))
                .and(predicate::str::contains("<li>\n                <svg class=\"magic-icon orb\" viewBox=\"0 0 24 24\"><path d=\"M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z\"/></svg>\n                <strong>Temporal Shift</strong> - Cost: 75 mana.\n            </li>"))
                .and(predicate::str::contains("<li>\n                <svg class=\"magic-icon orb\" viewBox=\"0 0 24 24\"><path d=\"M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8z\"/></svg>\n                <strong>Ethereal Shield</strong> - Cost: 50 mana.\n            </li>"))
                .and(predicate::str::contains("<em>A shimmering portal flickers into existence...</em></p><button style=\"background: #00ffff; color: #1a0a2a; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer; font-family: 'Cinzel', serif; box-shadow: 0 0 10px #00ffff;\">Enter the Void</button>"))
                .and(predicate::str::contains("<svg class=\"magic-icon scroll\" viewBox=\"0 0 24 24\"><path d=\"M20 2H4c-1.1 0-2 .9-2 2v18l4-2 4 2 4-2 4 2 4-2V4c0-1.1-.9-2-2-2zm0 15.23l-2-1-2 1-2-1-2 1-2-1-2 1V4h16v13.23z\"/></svg>\n            <p><em>\"Wisdom is not a product of schooling but of the lifelong attempt to acquire it.\"</em> - Albert Einstein</p>\n<p>May your path be illuminated, noble Lyra.</p>"))
        );

    Ok(())
}


