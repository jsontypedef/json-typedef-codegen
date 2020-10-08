using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class DiscriminatorBaz : Discriminator
    {

        [JsonProperty("foo")]
        public const string Foo = "baz";


        [JsonProperty("bazThing")]
        public object BazThing { get; set; }





    }
}
