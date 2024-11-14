use std::{
    cmp::Ordering,
    fs::File,
    io::{BufWriter, Result, Write},
};

fn generate_enum_display_strings(
    arity: i32,
    condition_fn: impl Fn(u32) -> String,
    mut writer: impl Write,
) -> Result<()> {
    let suffix = "</DisplayString>";

    for i in (-1..arity + 1).rev() {
        for v in 0..16 {
            let mut write_line = |s: &str| writer.write_fmt(format_args!("{s}{suffix}\n"));

            let mut s = format!(
                "    <DisplayString Condition=\"{}\" Optional=\"true\">{{variant{v}.NAME,en}}",
                condition_fn(v)
            );

            match i.cmp(&0) {
                Ordering::Equal => s += &format!(" {{variant{v}.value}}"),

                Ordering::Greater => {
                    s += " (";

                    for j in 0..i {
                        if j > 0 {
                            s += ", ";
                        }

                        s += &format!("{{variant{v}.value.__{j}}}");
                    }

                    if i == arity {
                        s += ", ...";
                    }

                    s += ")";
                }

                _ => (),
            }

            write_line(&s)?;
        }

        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut writer = BufWriter::new(File::create("intrinsic.natvis")?);
    writer.write_all(&std::fs::read("intrinsic_header.natvis")?)?;
    let arity = 4;

    generate_enum_display_strings(
        arity,
        |v| format!("tag == variant{}.DISCR_EXACT", v),
        &mut writer,
    )?;

    generate_enum_display_strings(
        arity,
        |v| {
            format!(
                "in_range(tag, variant{0}.DISCR_BEGIN, variant{0}.DISCR_END)",
                v
            )
        },
        &mut writer,
    )?;

    generate_enum_display_strings(
        arity,
        |v| {
            format!(
                "eq128(tag128_hi, tag128_lo, variant{0}.DISCR128_EXACT_HI, variant{0}.DISCR128_EXACT_LO)",
                v
            )
        },
        &mut writer,
    )?;

    generate_enum_display_strings(
        arity,
        |v| {
            format!(
                "in_range128(tag128_hi, tag128_lo, variant{0}.DISCR128_BEGIN_HI, variant{0}.DISCR128_BEGIN_LO, variant{0}.DISCR128_END_HI, variant{0}.DISCR128_END_LO)",
                v
            )
        },
        &mut writer,
    )?;

    writer.write_all(&std::fs::read("intrinsic_footer.natvis")?)?;
    Ok(())
}
