use std::fs;
use walkdir::WalkDir;

fn main() {
    let dir_to_search = "/opt/programacao/static/videos";  // Substitua pelo caminho do diretório que você deseja pesquisar

    for entry in WalkDir::new(dir_to_search).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_name = entry.file_name().to_string_lossy();
            // Adicione suas condições de pesquisa aqui, por exemplo, usando expressões regulares
            if file_name.contains("mp4") {
                println!("Arquivo encontrado: {}", entry.path().display());
                
            }
        }
    }
}
