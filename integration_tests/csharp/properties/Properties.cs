using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    
    public class Properties 
    {



        [JsonProperty("a")]
        public string A { get; set; }


        [JsonProperty("b")]
        public DateTime B { get; set; }


        [JsonProperty("c")]
        public string C { get; set; }


        [JsonProperty("d")]
        public D D { get; set; }





    }
}
