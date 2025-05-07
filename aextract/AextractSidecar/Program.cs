static class AExtract
{
    static void Main(string[] args)
    {
        string Requests = args[0];
        string TargetDirectory = args[1];

        DirectoryInfo output_dir = System.IO.Directory.CreateDirectory(Path.Join(TargetDirectory, "assets"));
    }
}
