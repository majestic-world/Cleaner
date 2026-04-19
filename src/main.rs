use std::collections::HashMap;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

fn main() {
    let exe_dir = std::env::current_exe()
        .and_then(|p| Ok(p.parent().unwrap_or(Path::new("")).to_path_buf()))
        .unwrap_or_else(|_| PathBuf::from("."));
    
    let config_path = exe_dir.join("Cleaner.properties");

    if !config_path.exists() {
        println!("[ERRO] Arquivo de configuracao nao encontrado: {}", config_path.display());
        pause();
        return;
    }

    let content = match fs::read_to_string(&config_path) {
        Ok(c) => c,
        Err(e) => {
            println!("[ERRO] Falha ao ler o arquivo de configuracao: {}", e);
            pause();
            return;
        }
    };

    let mut jar_targets: HashMap<String, Vec<String>> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((jar, class_path)) = line.split_once('=') {
            let jar_name = jar.trim().to_string();
            let class_path_formatted = class_path.trim().replace('.', "/");
            jar_targets.entry(jar_name).or_insert_with(Vec::new).push(class_path_formatted);
        }
    }

    if jar_targets.is_empty() {
        println!("[AVISO] Nenhuma configuracao encontrada no arquivo.");
        pause();
        return;
    }

    for (jar_name, classes_to_remove) in &jar_targets {
        let jar_path_str = format!("{}.jar", jar_name);
        
        let jar_path = exe_dir.join(&jar_path_str);
        if !jar_path.exists() {
            println!("[ERRO] Arquivo {} nao encontrado.", jar_path.display());
            continue;
        }

        let temp_jar_path_str = format!("{}_temp.jar", jar_name);
        let temp_jar_path = exe_dir.join(&temp_jar_path_str);

        match process_jar(&jar_path, &temp_jar_path, classes_to_remove) {
            Ok(_) => {
                if let Err(e) = fs::rename(&temp_jar_path, &jar_path) {
                    println!("[ERRO] Falha ao renomear o arquivo {}: {}", jar_path_str, e);
                } else {
                    println!("[SUCESSO] O arquivo {} foi atualizado com sucesso!", jar_path_str);
                }
            }
            Err(e) => {
                println!("[ERRO] Falha ao processar {}: {}", jar_path_str, e);
                let _ = fs::remove_file(&temp_jar_path);
            }
        }
    }

    pause();
}

fn pause() {
    println!("Pressione Enter para sair...");
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

fn process_jar(jar_path: &Path, temp_jar_path: &Path, classes_to_remove: &[String]) -> zip::result::ZipResult<()> {
    let jar_name = jar_path.file_name().unwrap_or_default().to_string_lossy();
    println!("[1/2] Abrindo {}...", jar_name);
    
    let file = File::open(jar_path)?;
    let mut archive = ZipArchive::new(file)?;
    
    let temp_file = File::create(temp_jar_path)?;
    let mut zip_writer = ZipWriter::new(temp_file);

    let mut entries_to_delete_count = 0;
    let mut entries_to_keep = Vec::new();

    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        
        let mut should_remove = false;
        let lower_name = name.to_ascii_lowercase();
        
        for class_path in classes_to_remove {
            let lower_class_path = class_path.to_ascii_lowercase();
            let class_exact = format!("{}.class", lower_class_path);
            let class_prefix = format!("{}$", lower_class_path);
            
            if lower_name == class_exact || lower_name.starts_with(&class_prefix) {
                should_remove = true;
                break;
            }
        }
        
        if should_remove {
            entries_to_delete_count += 1;
            println!("Arquivo excluido de {} - {}", jar_name, name);
        } else {
            entries_to_keep.push(i);
        }
    }

    println!("[2/2] Removendo {} arquivos de {}...", entries_to_delete_count, jar_name);

    for i in entries_to_keep {
        let mut file = archive.by_index(i)?;
        let mut options = FileOptions::default()
            .compression_method(file.compression());
            
        if let Some(mode) = file.unix_mode() {
            options = options.unix_permissions(mode);
        }

        let name = file.name().to_string();
        zip_writer.start_file(&name, options)?;
        io::copy(&mut file, &mut zip_writer)?;
    }

    zip_writer.finish()?;

    Ok(())
}
