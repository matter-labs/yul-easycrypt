//!
//! Yul to EasyCrypt translation
//!

pub mod arguments;
use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() {
    std::process::exit(match main_inner() {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    })
}

fn print_version() {
    println!(
        "Yul to EasyCrypt transpiler, part of {} v{}",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );
}

///
/// The auxiliary `main` function to facilitate the `?` error conversion operator.
///
fn main_inner() -> anyhow::Result<()> {
    let arguments = Arguments::new();
    arguments.validate()?;

    if arguments.version {
        print_version();
        return Ok(());
    }

    let _input_files = arguments.input_files_paths()?;

    // let path = match input_files.len() {
    //     1 => input_files.first().expect("Always exists"),
    //     0 => anyhow::bail!("The input file is missing"),
    //     length => anyhow::bail!("Only one input file is allowed but found {}", length,),
    // };

    //     project.contracts.iter().for_each(|(_path, contr)| {
    //         if let Some(obj) = contr.get_yul_object() {
    //             //WritePrinter::default().visit_object(obj);

    //             let module = Translator::transpile(obj).unwrap();
    //             //println!("{:#?}", m);

    //             println!(
    //                 r"
    // require import UInt256 PurePrimops YulPrimops.

    // op STRING : int = 0.
    // "
    //             );
    //             let mut printer = ECPrinter::new(module.dependency_order.iter());
    //             printer.print_all(&module);
    //         }
    //     });

    Ok(())
}
