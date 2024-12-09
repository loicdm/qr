fn main() {
    use qrcode_generator::QrCodeEcc;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        let file_path = args[2].clone();
        match qrcode_generator::to_png_to_file(args[1].clone(), QrCodeEcc::Low, 1024, file_path) {
            Ok(_) => {}
            Err(qr_code_ecc) => {
                eprintln!("{qr_code_ecc}");
            }
        }
    } else {
        match std::env::current_exe() {
            Ok(exe_path) => match exe_path.file_name() {
                Some(exe_file_name) => match exe_file_name.to_str() {
                    Some(exe_file_name_str) => {
                        eprintln!("Usage: {exe_file_name_str} string file_path")
                    }
                    _ => {
                        eprintln!("cannot determine executable name")
                    }
                },
                _ => {
                    eprintln!("cannot determine executable name");
                }
            },
            Err(e) => eprintln!("failed to get current exe path: {e}"),
        };
    }
}
