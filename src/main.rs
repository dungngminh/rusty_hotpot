enum Size {
    XS(i16),
    S(i16),
    M(i16),
    L(i16),
}

#[derive(Debug)]
enum UiState {
    Initial,
    Loading,
    Loaded { data: String },
    Error { message: String },
}

impl Size {
    fn display(&self) {
        match self {
            Size::XS(size) => println!("Size: XS, {size}"),
            Size::S(size) => println!("Size: S, {size}"),
            Size::M(size) => println!("Size: M, {size}"),
            Size::L(size) => println!("Size: L, {size}"),
        }
    }
}

fn main() {
    let xs_size = Size::XS(105);
    let s_size = Size::S(110);
    let m_size = Size::M(115);
    let l_size = Size::L(120);

    xs_size.display();
    s_size.display();
    m_size.display();
    l_size.display();

    let some_optional_size: Option<Size> = Some(Size::XS(1056));

    let none_optional_size: Option<Size> = None;

    some_optional_size.unwrap().display();

    none_optional_size.unwrap_or(xs_size).display();

    let mut ui_state = UiState::Initial;

    println!("Ui State when initial is {ui_state:?}");

    // call api
    ui_state = UiState::Loading;
    println!("Ui State when start call API is {ui_state:?}");

    // call api
    ui_state = UiState::Loaded {
        data: String::from("Data"),
    };
    println!("Ui State when loaded data is {ui_state:?}");

    // call api and has error
    ui_state = UiState::Error {
        message: String::from("Error"),
    };
    println!("Ui State when error is {ui_state:?}");
}
