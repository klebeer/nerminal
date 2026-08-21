use thousands::Separable;

pub fn format_credits(credits: i64) -> String {
    credits.separate_with_commas()
}

pub fn format_cost_cents(cents: i64) -> String {
    let dollars = cents / 100;
    let remainder = (cents.abs() % 100) as u8;
    if dollars < 0 {
        format!(
            "-${}.{remainder:02}",
            dollars.unsigned_abs().separate_with_commas()
        )
    } else {
        format!("${}.{remainder:02}", dollars.separate_with_commas())
    }
}
