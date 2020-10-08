using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    
    public class Gamut 
    {



        [JsonProperty("discriminator")]
        public Discriminator Discriminator { get; set; }


        [JsonProperty("elements")]
        public Elements Elements { get; set; }


        [JsonProperty("empty")]
        public Empty Empty { get; set; }


        [JsonProperty("enum")]
        public Enum Enum { get; set; }


        [JsonProperty("type")]
        public Type Type { get; set; }


        [JsonProperty("values")]
        public Values Values { get; set; }





    }
}
