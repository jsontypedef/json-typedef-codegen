using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class User 
    {


        [JsonProperty("id")]
        public string Id { get; set; }

        [JsonProperty("labels")]
        public IDictionary<string, string> Labels { get; set; }

        [JsonProperty("name")]
        public Name Name { get; set; }

        [JsonProperty("preferences")]
        public Preferences Preferences { get; set; }

        [JsonProperty("first_known_location")]
        public Location FirstKnownLocation { get; set; }

        [JsonProperty("last_known_location")]
        public Location LastKnownLocation { get; set; }





    }
}
