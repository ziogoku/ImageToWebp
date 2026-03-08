use image::GenericImageView;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Utilizzo: pngtowebp <cartella_input>");
        return Ok(());
    }

    let cartella_input = Path::new(&args[1]);
    let cartella_output = cartella_input.join("webp");

    // Crea la cartella webp se non esiste
    if !cartella_output.exists() {
        fs::create_dir_all(&cartella_output)?;
    }

    println!("🚀 Inizio conversione in WebP (Qualità: 85)...");

    for entry in WalkDir::new(cartella_input)
        .max_depth(1) // Solo file nella cartella principale
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Processiamo solo i file PNG
        let estensione = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        // Lista dei formati supportati
        let formati_ok = ["png", "jpg", "jpeg", "bmp"];

        if path.is_file() && formati_ok.contains(&estensione.as_str()) {
            let nome_file = path.file_name().unwrap();
            let destinazione = cartella_output.join(nome_file).with_extension("webp");

            println!("Elaborazione: {:?}", nome_file);

            // 1. Carichiamo l'immagine
            let img = image::open(path)?;
            let (w, h) = img.dimensions();

            // 2. CREIAMO UNA VARIABILE PER IL BUFFER (Così vive oltre questa riga)
            let rgba_buffer = img.to_rgba8();

            // 3. Passiamo il riferimento del buffer all'encoder
            let encoder = webp::Encoder::from_rgba(&rgba_buffer, w, h);

            // 4. Encode Lossy con qualità 85
            let memory = encoder.encode(85.0);

            // 5. Salvataggio
            fs::write(&destinazione, &*memory)?;

            let peso_orig = fs::metadata(path)?.len() / 1024;
            let peso_nuovo = fs::metadata(&destinazione)?.len() / 1024;
            println!("  ✅ Risparmio: {}KB -> {}KB", peso_orig, peso_nuovo);
        }
    }

    println!("---");
    println!(
        "Conversione completata! Trovi i file in: {:?}",
        cartella_output
    );
    Ok(())
}
