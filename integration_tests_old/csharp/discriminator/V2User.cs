using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    
    public class V2User 
    {



        [JsonProperty("favoriteNumbers")]
        public IList<string> FavoriteNumbers { get; set; }


        [JsonProperty("id")]
        public string Id { get; set; }





    }
}
