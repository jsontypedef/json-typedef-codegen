using System;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace JtdCodegenE2E
{
    class Program
    {
        static void Main(string[] args)
        {
            string line;
            int i = 0;
            while ((line = System.Console.ReadLine()) != null)
            {
                try
                {
                    MAIN input = JsonSerializer.Deserialize<MAIN>(line);
                    System.Console.WriteLine(JsonSerializer.Serialize(input));
                } catch (Exception e)
                {
                    throw new Exception(String.Format("Error on line {0}", i), e);
                }

                i++;
            }
        }
    }
}
