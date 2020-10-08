using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    
    public class V1User 
    {


        [JsonProperty("id")]
        public string Id { get; set; }

        [JsonProperty("favoriteNumbers")]
        public IList<uint> FavoriteNumbers { get; set; }





    }
}
