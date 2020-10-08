using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// A latitude / longitude pair indicating a position on Earth
    /// </summary>
    
    public class Location 
    {


        /// <summary>
        /// Latitude
        /// </summary>
        [JsonProperty("lat")]
        public string Lat { get; set; }

        /// <summary>
        /// Longitude
        /// </summary>
        [JsonProperty("lng")]
        public string Lng { get; set; }





    }
}
