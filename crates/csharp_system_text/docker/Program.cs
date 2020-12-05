using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegen.Demo
{
    class Program
    {
        static void Main(string[] args)
        {
            string line;
            while ((line = System.Console.ReadLine()) != null)
            {
                MAIN input = JsonSerializer.Deserialize<MAIN>(line);
                System.Console.WriteLine(JsonSerializer.Serialize(input));
            }
        }
    }
}
