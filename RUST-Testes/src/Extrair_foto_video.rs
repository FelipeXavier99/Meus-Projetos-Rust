//Salva uma foto em um vídeo

use std::process::Command;
use std::path::Path;

fn main() {
    // Caminho do arquivo de vídeo
    let video_path = "/caminho/para/o/video/video.mp4";

    // Diretório de saída para o arquivo de imagem
    let output_dir = "/caminho/para/o/diretorio/de/saida";

    // Nome do arquivo de saída para a imagem
    let output_filename = "output.jpg";

    // Escolher horário pra foto do vídeo (tempo em que o FFmpeg começará a capturar o frame)
    let start_time = "00:01:45";

    // Construir o comando FFmpeg
    let output_path = Path::new(output_dir).join(output_filename);
    let output_path_str = output_path.to_str().unwrap();
    let ffmpeg_command = format!("ffmpeg -ss {} -i {} -frames:v 1 -q:v 2 {}", start_time, video_path, output_path_str);

    // Imprimir o comando FFmpeg que será executado
    println!("Comando FFmpeg: {}", ffmpeg_command);

    // Executar o comando FFmpeg
    let output = Command::new("sh")
        .arg("-c")
        .arg(&ffmpeg_command)
        .output()
        .expect("Falha ao executar o comando FFmpeg");

    // Verificar se houve algum erro na execução do comando
    if !output.status.success() {
        eprintln!("Erro ao executar o comando FFmpeg: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Frame capturado com sucesso e salvo como {}", output_path_str);
    }
}
