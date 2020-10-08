using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class V2 : Discriminator
    {

        [JsonProperty("version")]
        public const string Version = "v2";


        [JsonProperty("user")]
        public V2User User { get; set; }





    }
}
