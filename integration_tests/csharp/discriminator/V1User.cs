using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    
    public class V1User 
    {



        [JsonProperty("favoriteNumbers")]
        public IList<uint> FavoriteNumbers { get; set; }


        [JsonProperty("id")]
        public string Id { get; set; }





    }
}
