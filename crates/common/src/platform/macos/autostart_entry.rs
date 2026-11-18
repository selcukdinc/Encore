//! LaunchAgent content, kept platform-independent so packaging paths can be tested anywhere.
use std::path::Path;

fn xml_path(path: &Path) -> Result<String, String> {
    let text = path.to_str().ok_or("Startup path is not valid UTF-8")?;
    if text.chars().any(|c| c.is_control()) {
        return Err("Startup path contains control characters".into());
    }
    Ok(text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace('"', "&quot;").replace('\'', "&apos;"))
}

pub(super) fn launch_agent(executable: &Path, log_path: &Path) -> Result<String, String> {
    // launchd starts the recorder directly, so it is its own responsible process and
    // privacy grants stay with Encore Recorder, exactly as with a manual launch.
    // Running the bundled executable (not /usr/bin/open) also lets Login Items
    // show the item as "Encore" instead of "open" for builds without a Team ID.
    Ok(format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n\
        <plist version=\"1.0\"><dict>\
        <key>Label</key><string>com.encore.recorder</string>\
        <key>ProgramArguments</key><array><string>{}</string></array>\
        <key>StandardErrorPath</key><string>{}</string>\
        <key>RunAtLoad</key><true/>\
        <key>LimitLoadToSessionType</key><string>Aqua</string>\
        <key>ProcessType</key><string>Interactive</string>\
        </dict></plist>\n", xml_path(executable)?, xml_path(log_path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packaged_agent_runs_inner_recorder_executable_with_escaped_paths() {
        let text = launch_agent(
            Path::new("/Applications/Encore & Friends.app/Contents/Resources/Encore Recorder.app/Contents/MacOS/encore-recorder"),
            Path::new("/Users/test/Library/Application Support/Encore/recorder.log"),
        ).unwrap();
        assert!(text.contains("<array><string>/Applications/Encore &amp; Friends.app/Contents/Resources/Encore Recorder.app/Contents/MacOS/encore-recorder</string></array>"));
        assert!(text.contains("<key>StandardErrorPath</key><string>/Users/test/Library/Application Support/Encore/recorder.log</string>"));
        assert!(!text.contains("/usr/bin/open"));
        assert!(text.contains("<key>RunAtLoad</key><true/>"));
    }

    #[test]
    fn development_agent_can_launch_an_unbundled_helper() {
        let text = launch_agent(Path::new("/tmp/Encore <dev>/encore-recorder"), Path::new("/tmp/recorder.log")).unwrap();
        assert!(text.contains("<array><string>/tmp/Encore &lt;dev&gt;/encore-recorder</string></array>"));
        assert!(launch_agent(Path::new("/tmp/bad\npath"), Path::new("/tmp/log")).is_err());
        assert!(launch_agent(Path::new("/tmp/encore-recorder"), Path::new("/tmp/bad\nlog")).is_err());
    }
}
