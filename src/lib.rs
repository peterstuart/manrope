#[cfg(feature = "bold")]
pub mod bold {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-bold", 6, 80);
}

#[cfg(feature = "extrabold")]
pub mod extrabold {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-extrabold", 6, 80);
}

#[cfg(feature = "extralight")]
pub mod extralight {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-extralight", 6, 80);
}

#[cfg(feature = "light")]
pub mod light {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-light", 6, 80);
}

#[cfg(feature = "medium")]
pub mod medium {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-medium", 6, 80);
}

#[cfg(feature = "regular")]
pub mod regular {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-regular", 6, 80);
}

#[cfg(feature = "semibold")]
pub mod semibold {
    use manrope_macros::generate_fonts;

    generate_fonts!("manrope-semibold", 6, 80);
}
