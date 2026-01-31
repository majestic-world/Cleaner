namespace CleanUP;
using System.IO.Compression;

internal static class JarCleaner
{
    static void Main()
    {
        const string jarPath = "server.jar";
        
       
        string[] classesToRemove = {"Player", "Creature"};

        if (!File.Exists(jarPath))
        {
            Console.WriteLine($"[ERRO] Arquivo {jarPath} nao encontrado.");
            return;
        }

        try
        {
            Console.WriteLine($"[1/2] Abrindo {jarPath}...");
            using (var archive = ZipFile.Open(jarPath, ZipArchiveMode.Update))
            {
                var entriesToDelete = archive.Entries.Where(entry => 
                {
                    var fileName = Path.GetFileName(entry.Name);
                    return classesToRemove.Any(c => 
                        fileName.Equals($"{c}.class", StringComparison.OrdinalIgnoreCase) || 
                        fileName.StartsWith($"{c}$", StringComparison.OrdinalIgnoreCase)
                    );
                }).ToList();

                Console.WriteLine($"[2/2] Removendo {entriesToDelete.Count} arquivos...");

                foreach (var entry in entriesToDelete)
                {
                    Console.WriteLine($"Arquivo excluido - {entry.FullName}");
                    entry.Delete();
                }
            }

            Console.WriteLine("[SUCESSO] O arquivo JAR foi atualizado com sucesso!");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"[ERRO] Ocorreu um problema: {ex.Message}");
        }

        Console.WriteLine("Pressione qualquer tecla para sair...");
        Console.ReadKey();
    }
}