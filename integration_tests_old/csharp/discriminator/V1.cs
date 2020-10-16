using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    
    public class V1 : Discriminator
    {

        [JsonProperty("version")]
        public const string Version = "v1";



        [JsonProperty("user")]
        public V1User User { get; set; }





    }
}
