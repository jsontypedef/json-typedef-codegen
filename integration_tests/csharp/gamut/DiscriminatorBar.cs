using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    
    public class DiscriminatorBar : Discriminator
    {

        [JsonProperty("foo")]
        public const string Foo = "bar";



        [JsonProperty("barThing")]
        public object BarThing { get; set; }





    }
}
