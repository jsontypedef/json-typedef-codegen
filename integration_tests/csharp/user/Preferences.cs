using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// Some preferences the user has indicated to us.
    /// </summary>
    
    public class Preferences 
    {


        /// <summary>
        /// User preferences around do-not-track
        /// </summary>
        [JsonProperty("do_not_track")]
        public PreferencesDoNotTrack DoNotTrack { get; set; }

        /// <summary>
        /// A title we should use when addressing the user formally.
        /// </summary>
        [JsonProperty("title")]
        public PreferencesTitle Title { get; set; }





    }
}
