using CUE4Parse.FileProvider;
using CUE4Parse.MappingsProvider;
using CUE4Parse.UE4.Objects.Core.Serialization;
using CUE4Parse.UE4.Versions;
using Newtonsoft.Json;

static class AExtract
{
    static void Main(string[] args)
    {
        string PaksDirectory = args[0];
        string CommunityDirectory = args[1];
        string Requests = args[2];
        string TargetDirectory = args[3];

        using StreamReader r = new(Path.Join(CommunityDirectory, "CustomVersions.json"));
        string json = r.ReadToEnd();
        List<FCustomVersion>? versions = JsonConvert.DeserializeObject<List<FCustomVersion>>(json);

        DirectoryInfo output_dir = System.IO.Directory.CreateDirectory(
            Path.Join(TargetDirectory, "assets")
        );
        var provider = new DefaultFileProvider(
            new DirectoryInfo(PaksDirectory),
            SearchOption.TopDirectoryOnly,
            new VersionContainer(
                game: EGame.GAME_UE5_3,
                customVersions: new FCustomVersionContainer(versions)
            )
        )
        {
            MappingsContainer = new FileUsmapTypeMappingsProvider(
                Path.Join(CommunityDirectory, "FactoryGame.usmap")
            ),
        };
        provider.Initialize();
        foreach (var f in provider.Files)
        {
            Console.WriteLine(f.Key);
        }
    }
}
