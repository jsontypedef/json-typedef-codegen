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


        /// <value>
        /// Latitude
        /// </value>
        [JsonProperty("lat")]
        public string Lat { get; set; }

        /// <value>
        /// Longitude
        /// </value>
        [JsonProperty("lng")]
        public string Lng { get; set; }





    }
}
