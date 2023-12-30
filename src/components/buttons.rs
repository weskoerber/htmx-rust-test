pub(crate) struct Button;

pub(crate) trait Styled {
    const DEFAULT: &'static str;
}

impl Styled for Button {
    const DEFAULT: &'static str = "rounded bg-sky-500 p-2.5 text-white";
}
