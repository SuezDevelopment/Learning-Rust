
pub fn run_program_get_output<T1, T2>(cmd: &T1, args: &[T2]) -> Result<String>
where
    T1: AsRef<str>,
    T2: AsRef<str>,
{
    let output = make_command(cmd, args).output()?;

    if !output.status.success() {
        return Err(anyhow!("program '{}' returned non-zero", cmd.as_ref()));
    }

    let stdout = String::from_utf8(output.stdout)?;

    Ok(stdout)
}


pub fn relative_duration(t: &Duration) -> String {
    let secs = t.as_secs();

    let v = [
        (secs / 60 / 60 / 24 / 365, "year"),
        (secs / 60 / 60 / 24 / 30, "month"),
        (secs / 60 / 60 / 24 / 7, "week"),
        (secs / 60 / 60 / 24, "day"),
        (secs / 60 / 60, "hour"),
        (secs / 60, "minute"),
        (secs, "second"),
    ];

    let mut plural = "";
    for (num, name) in v {
        if num > 1 {
            plural = "s"
        }

        if num > 0 {
            return format!("{} {}{}", num, name, plural);
        }
    }

    String::from("0 seconds")
}

pub fn trim_long_string(s: &str, limit: usize, suffix: &str) -> String {
    let suffix_len = suffix.len();

    assert!(limit > suffix_len, "number too small");

    let len = s.len();

    // don't do anything if string is smaller than limit
    if len < limit {
        return s.to_string();
    }

    // make new string (without formatting)
    format!(
        "{}{}",
        s.chars().take(limit - suffix_len).collect::<String>(),
        suffix
    )
}