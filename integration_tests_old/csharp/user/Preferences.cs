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


        /// <value>
        /// User preferences around do-not-track
        /// </value>
        [JsonProperty("do_not_track")]
        public PreferencesDoNotTrack DoNotTrack { get; set; }

        /// <value>
        /// A title we should use when addressing the user formally.
        /// </value>
        [JsonProperty("title")]
        public PreferencesTitle Title { get; set; }





    }
}
